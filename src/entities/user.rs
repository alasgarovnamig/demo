// src/entities/user.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::entities::r#enum::user_type::UserType;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub partner_id: i32,
    pub email: String,
    // pub username: String,
    pub password_hash: String,
    pub user_type: UserType,
    pub is_system_user: bool, // For application users
    pub is_admin: bool, // Partner admin flag
    pub can_access_all_partners: bool, // For main partner users
    pub created_by: Option<i32>,
    pub is_active: bool,
    pub last_login: Option<DateTime>,
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
    #[sea_orm(has_many = "super::audit_log::Entity")]
    AuditLogs,
}

impl Related<super::partner::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Partner.def()
    }
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_role::Relation::Role.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::user_role::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}