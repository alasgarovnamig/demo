// src/entities/permission.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::entities::r#enum::permission_scope::PermissionScope;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "permissions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub resource: String, // e.g., "partners", "users", "invoices"
    pub action: String, // e.g., "create", "read", "update", "delete"
    pub scope: PermissionScope,
    pub conditions: Option<Json>, // OPA conditions
    pub description: Option<String>,
    pub created_at: DateTime,
}



#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::role_permission::Entity")]
    RolePermissions,
}

impl ActiveModelBehavior for ActiveModel {}
