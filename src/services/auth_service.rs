// src/services/auth_service.rs
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use crate::entities::{user, partner, role, permission, user_role, role_permission, r#enum};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // User ID
    pub partner_id: Uuid,
    pub is_main_partner: bool,
    pub can_access_all_partners: bool,
    pub roles: Vec<String>,
    pub permissions: Vec<PermissionClaim>,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionClaim {
    pub resource: String,
    pub action: String,
    pub scope: String,
}

#[derive(Clone)]
pub struct AuthService {
    db: DatabaseConnection,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(db: DatabaseConnection, jwt_secret: String) -> Self {
        Self { db, jwt_secret }
    }

    pub async fn authenticate(&self, email: &str, password: &str) -> Result<String, AppError> {
        // Find user
        let user = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .filter(user::Column::IsActive.eq(true))
            .one(&self.db)
            .await?
            .ok_or(AppError::Unauthorized)?;

        // Verify password
        if !verify(password, &user.password_hash)? {
            return Err(AppError::Unauthorized);
        }

        // Get partner info
        let partner = partner::Entity::find_by_id(user.partner_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Partner not found".into()))?;

        // Get user permissions
        let permissions = self.get_user_permissions(user.id).await?;

        // Create claims
        let claims = Claims {
            sub: user.id,
            partner_id: user.partner_id,
            is_main_partner: partner.is_main_partner,
            can_access_all_partners: user.can_access_all_partners,
            roles: self.get_user_roles(user.id).await?,
            permissions,
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };

        // Generate token
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;

        Ok(token)
    }

    pub async fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    pub async fn hash_password(&self, password: &str) -> Result<String, AppError> {
        Ok(hash(password, DEFAULT_COST)?)
    }

    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<String>, AppError> {
        let roles = user_role::Entity::find()
            .filter(user_role::Column::UserId.eq(user_id))
            .find_also_related(role::Entity)
            .all(&self.db)
            .await?;

        Ok(roles
            .into_iter()
            .filter_map(|(_, role)| role.map(|r| r.code))
            .collect())
    }

    async fn get_user_permissions(&self, user_id: Uuid) -> Result<Vec<PermissionClaim>, AppError> {
        // Get user's roles
        let user_roles = user_role::Entity::find()
            .filter(user_role::Column::UserId.eq(user_id))
            .all(&self.db)
            .await?;

        let role_ids: Vec<Uuid> = user_roles.iter().map(|ur| ur.role_id).collect();

        // Get permissions for roles
        let role_permissions = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.is_in(role_ids))
            .find_also_related(permission::Entity)
            .all(&self.db)
            .await?;

        let permissions = role_permissions
            .into_iter()
            .filter_map(|(_, perm)| {
                perm.map(|p| PermissionClaim {
                    resource: p.resource,
                    action: p.action,
                    scope:  p.scope.into(),
                })
            })
            .collect();

        Ok(permissions)
    }

    pub async fn check_permission(
        &self,
        claims: &Claims,
        resource: &str,
        action: &str,
        target_partner_id: Option<Uuid>,
    ) -> Result<bool, AppError> {
        // Main partner users with full access
        if claims.is_main_partner && claims.can_access_all_partners {
            return Ok(true);
        }

        // Check if user has the required permission
        let has_permission = claims.permissions.iter().any(|p| {
            p.resource == resource && p.action == action
        });

        if !has_permission {
            return Ok(false);
        }

        // Check scope
        if let Some(target_id) = target_partner_id {
            if target_id != claims.partner_id && !claims.can_access_all_partners {
                return Ok(false);
            }
        }

        Ok(true)
    }
}