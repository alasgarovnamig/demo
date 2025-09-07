// src/entities/role.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub partner_id: Option<i32>, // Null for system roles
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub is_system_role: bool,
    pub created_by: Option<i32>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::partner::Entity",
        from = "Column::PartnerId",
        to = "super::partner::Column::Id"
    )]
    Partner,
    #[sea_orm(has_many = "super::user_role::Entity")]
    UserRoles,
    #[sea_orm(has_many = "super::role_permission::Entity")]
    RolePermissions,
}

impl Related<super::permission::Entity> for Entity {
    fn to() -> RelationDef {
        super::role_permission::Relation::Permission.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::role_permission::Relation::Role.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}