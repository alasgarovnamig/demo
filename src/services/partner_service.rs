// src/services/partner_service.rs
use sea_orm::{
    DatabaseConnection, EntityTrait, ActiveModelTrait, Set, QueryFilter, ColumnTrait,
    TransactionTrait, DbErr
};
use uuid::Uuid;
use crate::entities::{partner, user, role,r#enum};
use crate::error::AppError;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePartnerRequest {
    pub name: String,
    pub code: String,
    pub partner_type: r#enum::partner_type::PartnerType,
    pub parent_partner_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePartnerAdminRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct PartnerService {
    db: DatabaseConnection,
    auth_service: super::auth_service::AuthService,
}

impl PartnerService {
    pub fn new(db: DatabaseConnection, auth_service: super::auth_service::AuthService) -> Self {
        Self { db, auth_service }
    }

    pub async fn create_partner(
        &self,
        req: CreatePartnerRequest,
        admin_req: CreatePartnerAdminRequest,
    ) -> Result<partner::Model, AppError> {
        let txn = self.db.begin().await?;

        // Create partner
        let partner_id = Uuid::new_v4();
        let partner = partner::ActiveModel {
            id: Set(partner_id),
            name: Set(req.name),
            code: Set(req.code),
            partner_type: Set(req.partner_type),
            parent_partner_id: Set(req.parent_partner_id),
            is_main_partner: Set(false),
            is_active: Set(true),
            settings: Set(serde_json::json!({})),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
        };

        let partner_model = partner.insert(&txn).await?;

        // Create admin user
        let password_hash = self.auth_service.hash_password(&admin_req.password).await?;
        let admin_user = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            partner_id: Set(partner_id),
            email: Set(admin_req.email),
            username: Set(admin_req.username),
            password_hash: Set(password_hash),
            user_type: Set(r#enum::user_type::UserType::Admin),
            is_system_user: Set(false),
            is_admin: Set(true),
            can_access_all_partners: Set(false),
            created_by: Set(None),
            is_active: Set(true),
            last_login: Set(None),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
        };

        admin_user.insert(&txn).await?;

        // Create default admin role for partner
        let admin_role = role::ActiveModel {
            id: Set(Uuid::new_v4()),
            partner_id: Set(Some(partner_id)),
            name: Set(format!("{} Admin", partner_model.name)),
            code: Set(format!("{}_admin", partner_model.code)),
            description: Set(Some("Partner admin role with full permissions".into())),
            is_system_role: Set(false),
            created_by: Set(None),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
        };

        admin_role.insert(&txn).await?;

        txn.commit().await?;

        Ok(partner_model)
    }

    pub async fn get_partner(&self, id: Uuid) -> Result<Option<partner::Model>, AppError> {
        Ok(partner::Entity::find_by_id(id).one(&self.db).await?)
    }

    pub async fn list_partners(
        &self,
        parent_id: Option<Uuid>,
    ) -> Result<Vec<partner::Model>, AppError> {
        let mut query = partner::Entity::find();

        if let Some(parent) = parent_id {
            query = query.filter(partner::Column::ParentPartnerId.eq(parent));
        }

        Ok(query.all(&self.db).await?)
    }

    pub async fn update_partner_status(
        &self,
        id: Uuid,
        is_active: bool,
    ) -> Result<partner::Model, AppError> {
        let partner = partner::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Partner not found".into()))?;

        let mut partner: partner::ActiveModel = partner.into();
        partner.is_active = Set(is_active);
        partner.updated_at = Set(chrono::Utc::now().naive_utc());

        Ok(partner.update(&self.db).await?)
    }
}