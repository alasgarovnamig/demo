// src/services/user_service.rs
use sea_orm::{
    DatabaseConnection, EntityTrait, ActiveModelTrait, Set, QueryFilter, ColumnTrait,
    TransactionTrait
};
use uuid::Uuid;
use crate::entities::{user, user_role, role,r#enum};
use crate::services::auth_service::{AuthService, Claims};
use serde::{Deserialize, Serialize};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub user_type: r#enum::user_type::UserType,
    pub is_system_user: bool,
    pub role_ids: Vec<Uuid>,
}

#[derive(Clone)]
pub struct UserService {
    db: DatabaseConnection,
    auth_service: AuthService,
}

impl UserService {
    pub fn new(db: DatabaseConnection, auth_service: AuthService) -> Self {
        Self { db, auth_service }
    }

    pub async fn create_user(
        &self,
        partner_id: Uuid,
        req: CreateUserRequest,
        created_by: Uuid,
        claims: &Claims,
    ) -> Result<user::Model, AppError> {
        // Check if creator can create users for this partner
        if !claims.is_main_partner && claims.partner_id != partner_id {
            return Err(AppError::Forbidden);
        }

        let txn = self.db.begin().await?;

        // Create user
        let password_hash = self.auth_service.hash_password(&req.password).await?;
        let user_id = Uuid::new_v4();

        let new_user = user::ActiveModel {
            id: Set(user_id),
            partner_id: Set(partner_id),
            email: Set(req.email),
            username: Set(req.username),
            password_hash: Set(password_hash),
            user_type: Set(req.user_type),
            is_system_user: Set(req.is_system_user),
            is_admin: Set(false),
            can_access_all_partners: Set(claims.is_main_partner && claims.can_access_all_partners),
            created_by: Set(Some(created_by)),
            is_active: Set(true),
            last_login: Set(None),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
        };

        let user_model = new_user.insert(&txn).await?;

        // Assign roles
        for role_id in req.role_ids {
            // Verify role belongs to partner or is system role
            let role_entity = role::Entity::find_by_id(role_id)
                .one(&txn)
                .await?
                .ok_or(AppError::NotFound("Role not found".into()))?;

            if !role_entity.is_system_role && role_entity.partner_id != Some(partner_id) {
                return Err(AppError::Forbidden);
            }

            let user_role_model = user_role::ActiveModel {
                id: Set(Uuid::new_v4()),
                user_id: Set(user_id),
                role_id: Set(role_id),
                assigned_by: Set(Some(created_by)),
                assigned_at: Set(chrono::Utc::now().naive_utc()),
            };

            user_role_model.insert(&txn).await?;
        }

        txn.commit().await?;

        Ok(user_model)
    }

    pub async fn list_partner_users(
        &self,
        partner_id: Uuid,
        claims: &Claims,
    ) -> Result<Vec<user::Model>, AppError> {
        // Check access rights
        if !claims.can_access_all_partners && claims.partner_id != partner_id {
            return Err(AppError::Forbidden);
        }

        Ok(user::Entity::find()
            .filter(user::Column::PartnerId.eq(partner_id))
            .all(&self.db)
            .await?)
    }

    pub async fn assign_role(
        &self,
        user_id: Uuid,
        role_id: Uuid,
        assigned_by: Uuid,
        claims: &Claims,
    ) -> Result<(), AppError> {
        // Get user to check partner
        let user = user::Entity::find_by_id(user_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("User not found".into()))?;

        // Check if assigner can modify this user
        if !claims.can_access_all_partners && claims.partner_id != user.partner_id {
            return Err(AppError::Forbidden);
        }

        // Get role to verify
        let role = role::Entity::find_by_id(role_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Role not found".into()))?;

        // Check if role can be assigned
        if !role.is_system_role && role.partner_id != Some(user.partner_id) {
            return Err(AppError::Forbidden);
        }

        // Check if admin is trying to assign a role they don't have
        if !claims.is_main_partner {
            let has_role = claims.roles.contains(&role.code);
            if !has_role {
                return Err(AppError::Forbidden);
            }
        }

        let user_role_model = user_role::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            role_id: Set(role_id),
            assigned_by: Set(Some(assigned_by)),
            assigned_at: Set(chrono::Utc::now().naive_utc()),
        };

        user_role_model.insert(&self.db).await?;

        Ok(())
    }
}