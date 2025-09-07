// src/services/permission_service.rs
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, QueryFilter, ColumnTrait, TransactionTrait, QuerySelect, NotSet};
// use uuid::Uuid;
use crate::entities::{permission, role, role_permission, api_permission, partner_api_access,r#enum};
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sea_orm::sea_query::UnOper;
use sea_orm::sqlx::encode::IsNull::No;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePermissionRequest {
    pub resource: String,
    pub action: String,
    pub scope: r#enum::permission_scope::PermissionScope,
    pub conditions: Option<serde_json::Value>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApiPermissionRequest {
    pub endpoint: String,
    pub method: r#enum::http_method::HttpMethod,
    pub module: String,
    pub required_permissions: Vec<i32>,
    pub description: Option<String>,
}

#[derive(Clone)]
pub struct PermissionService {
    db: DatabaseConnection,
}

impl PermissionService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_permission(
        &self,
        req: CreatePermissionRequest,
    ) -> Result<permission::Model, AppError> {
        let permission = permission::ActiveModel {
            id: NotSet,
            resource: Set(req.resource),
            action: Set(req.action),
            scope: Set(req.scope),
            conditions: Set(req.conditions),
            description: Set(req.description),
            created_at: Set(chrono::Utc::now().naive_utc()),
        };

        Ok(permission.insert(&self.db).await?)
    }

    pub async fn create_api_permission(
        &self,
        req: CreateApiPermissionRequest,
    ) -> Result<api_permission::Model, AppError> {
        let api_perm = api_permission::ActiveModel {
            id: NotSet,
            endpoint: Set(req.endpoint),
            method: Set(req.method),
            module: Set(req.module),
            required_permissions: Set(serde_json::json!(req.required_permissions)),
            description: Set(req.description),
            created_at: Set(chrono::Utc::now().naive_utc()),
        };

        Ok(api_perm.insert(&self.db).await?)
    }

    pub async fn grant_api_access_to_partner(
        &self,
        partner_id: i32,
        api_permission_id: i32,
        granted_by: i32,
    ) -> Result<(), AppError> {
        let access = partner_api_access::ActiveModel {
            id: NotSet,
            partner_id: Set(partner_id),
            api_permission_id: Set(api_permission_id),
            is_granted: Set(true),
            granted_by: Set(Some(granted_by)),
            granted_at: Set(chrono::Utc::now().naive_utc()),
        };

        access.insert(&self.db).await?;
        Ok(())
    }

    pub async fn check_api_access(
        &self,
        partner_id: i32,
        endpoint: &str,
        method: &str,
    ) -> Result<bool, AppError> {
        // Find matching API permission
        let api_perm = api_permission::Entity::find()
            .filter(api_permission::Column::Endpoint.eq(endpoint))
            .filter(api_permission::Column::Method.eq(method))
            .one(&self.db)
            .await?;

        if let Some(api_perm) = api_perm {
            // Check if partner has access
            let access = partner_api_access::Entity::find()
                .filter(partner_api_access::Column::PartnerId.eq(partner_id))
                .filter(partner_api_access::Column::ApiPermissionId.eq(api_perm.id))
                .filter(partner_api_access::Column::IsGranted.eq(true))
                .one(&self.db)
                .await?;

            Ok(access.is_some())
        } else {
            Ok(false)
        }
    }

    pub async fn assign_permission_to_role(
        &self,
        role_id: i32,
        permission_id: i32,
    ) -> Result<(), AppError> {
        let role_perm = role_permission::ActiveModel {
            id:NotSet,
            role_id: Set(role_id),
            permission_id: Set(permission_id),
            granted_at: Set(chrono::Utc::now().naive_utc()),
        };

        role_perm.insert(&self.db).await?;
        Ok(())
    }

    pub async fn get_role_permissions(
        &self,
        role_id: i32,
    ) -> Result<Vec<permission::Model>, AppError> {
        let perms = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.eq(role_id))
            .find_also_related(permission::Entity)
            .all(&self.db)
            .await?;

        Ok(perms
            .into_iter()
            .filter_map(|(_, perm)| perm)
            .collect())
    }

    pub async fn get_module_permissions(
        &self,
        module: &str,
    ) -> Result<Vec<api_permission::Model>, AppError> {
        Ok(api_permission::Entity::find()
            .filter(api_permission::Column::Module.eq(module))
            .all(&self.db)
            .await?)
    }
}