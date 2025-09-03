// src/services/opa_data_sync.rs
use crate::entities::{partner, permission, role, role_permission, user, user_role};
use crate::error::AppError;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone)]
pub struct OpaDataSync {
    db: DatabaseConnection,
    opa_url: String,
    client: reqwest::Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpaData {
    pub partners: Vec<PartnerData>,
    pub users: Vec<UserData>,
    pub roles: Vec<RoleData>,
    pub permissions: Vec<PermissionData>,
    pub user_roles: Vec<UserRoleData>,
    pub role_permissions: Vec<RolePermissionData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartnerData {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub is_main_partner: bool,
    pub is_active: bool,
    pub parent_partner_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: Uuid,
    pub partner_id: Uuid,
    pub email: String,
    pub username: String,
    pub user_type: String,
    pub is_admin: bool,
    pub can_access_all_partners: bool,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleData {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub partner_id: Option<Uuid>,
    pub is_system_role: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionData {
    pub id: Uuid,
    pub resource: String,
    pub action: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleData {
    pub user_id: Uuid,
    pub role_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RolePermissionData {
    pub role_id: Uuid,
    pub permission_id: Uuid,
}

impl OpaDataSync {
    pub fn new(db: DatabaseConnection, opa_url: String) -> Self {
        Self {
            db,
            opa_url,
            client: reqwest::Client::new(),
        }
    }

    /// OPA'ya tüm verileri yükler (başlangıçta ve periyodik olarak)
    pub async fn sync_all_data(&self) -> Result<(), AppError> {
        println!("Starting OPA data synchronization...");

        // Veritabanından tüm verileri çek
        let opa_data = self.fetch_all_data().await?;

        // OPA'ya gönder
        self.push_to_opa(opa_data).await?;

        println!("✓ OPA data synchronization completed");
        Ok(())
    }

    /// Veritabanından tüm ilgili verileri çeker
    async fn fetch_all_data(&self) -> Result<OpaData, AppError> {
        // Partners
        let partners = partner::Entity::find()
            .all(&self.db)
            .await?
            .into_iter()
            .map(|p| PartnerData {
                id: p.id,
                code: p.code,
                name: p.name,
                is_main_partner: p.is_main_partner,
                is_active: p.is_active,
                parent_partner_id: p.parent_partner_id,
            })
            .collect();

        // Users
        let users = user::Entity::find()
            .all(&self.db)
            .await?
            .into_iter()
            .map(|u| UserData {
                id: u.id,
                partner_id: u.partner_id,
                email: u.email,
                username: u.username,
                user_type: u.user_type.into(),
                is_admin: u.is_admin,
                can_access_all_partners: u.can_access_all_partners,
                is_active: u.is_active,
            })
            .collect();

        // Roles
        let roles = role::Entity::find()
            .all(&self.db)
            .await?
            .into_iter()
            .map(|r| RoleData {
                id: r.id,
                code: r.code,
                name: r.name,
                partner_id: r.partner_id,
                is_system_role: r.is_system_role,
            })
            .collect();

        // Permissions
        let permissions = permission::Entity::find()
            .all(&self.db)
            .await?
            .into_iter()
            .map(|p| PermissionData {
                id: p.id,
                resource: p.resource,
                action: p.action,
                scope: p.scope.into(),
            })
            .collect();

        // User-Role ilişkileri
        let user_roles = user_role::Entity::find()
            .all(&self.db)
            .await?
            .into_iter()
            .map(|ur| UserRoleData {
                user_id: ur.user_id,
                role_id: ur.role_id,
            })
            .collect();

        // Role-Permission ilişkileri
        let role_permissions = role_permission::Entity::find()
            .all(&self.db)
            .await?
            .into_iter()
            .map(|rp| RolePermissionData {
                role_id: rp.role_id,
                permission_id: rp.permission_id,
            })
            .collect();

        Ok(OpaData {
            partners,
            users,
            roles,
            permissions,
            user_roles,
            role_permissions,
        })
    }

    /// OPA'ya veri gönderir
    async fn push_to_opa(&self, data: OpaData) -> Result<(), AppError> {
        // OPA'ya veri gönderme endpoint'i
        let url = format!("{}/v1/data/system", self.opa_url);

        let response = self
            .client
            .put(&url)
            .json(&serde_json::json!({
                "partners": data.partners,
                "users": data.users,
                "roles": data.roles,
                "permissions": data.permissions,
                "user_roles": data.user_roles,
                "role_permissions": data.role_permissions,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ExternalService(format!(
                "OPA data sync failed: {}",
                error_text
            )));
        }

        Ok(())
    }

    /// Belirli bir kullanıcının verilerini günceller
    pub async fn sync_user_data(&self, user_id: Uuid) -> Result<(), AppError> {
        // Kullanıcı bilgilerini çek
        let user = user::Entity::find_by_id(user_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("User not found".into()))?;

        // Kullanıcının rollerini çek
        let user_roles = user_role::Entity::find()
            .filter(user_role::Column::UserId.eq(user_id))
            .all(&self.db)
            .await?;

        // OPA'ya kullanıcı güncellemesi gönder
        let url = format!("{}/v1/data/system/users/{}", self.opa_url, user_id);

        self.client
            .put(&url)
            .json(&serde_json::json!({
                "id": user.id,
                "partner_id": user.partner_id,
                "email": user.email,
                "username": user.username,
                "user_type": user.user_type.to_string(), // Burada değişiklik
                "is_admin": user.is_admin,
                "can_access_all_partners": user.can_access_all_partners,
                "is_active": user.is_active,
                "roles": user_roles.iter().map(|ur| ur.role_id).collect::<Vec<_>>(),
            }))
            .send()
            .await?;

        Ok(())
    }

    /// Belirli bir partner'ın verilerini günceller
    pub async fn sync_partner_data(&self, partner_id: Uuid) -> Result<(), AppError> {
        let partner = partner::Entity::find_by_id(partner_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Partner not found".into()))?;

        let url = format!("{}/v1/data/system/partners/{}", self.opa_url, partner_id);

        self.client
            .put(&url)
            .json(&serde_json::json!({
                "id": partner.id,
                "code": partner.code,
                "name": partner.name,
                "is_main_partner": partner.is_main_partner,
                "is_active": partner.is_active,
                "parent_partner_id": partner.parent_partner_id,
            }))
            .send()
            .await?;

        Ok(())
    }

    /// Role değişikliklerini senkronize eder
    pub async fn sync_role_permissions(&self, role_id: Uuid) -> Result<(), AppError> {
        let permissions = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.eq(role_id))
            .all(&self.db)
            .await?;

        let url = format!(
            "{}/v1/data/system/role_permissions/{}",
            self.opa_url, role_id
        );

        self.client
            .put(&url)
            .json(
                &permissions
                    .iter()
                    .map(|rp| rp.permission_id)
                    .collect::<Vec<_>>(),
            )
            .send()
            .await?;

        Ok(())
    }
}
