// src/entities/partner.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
pub(crate) use crate::entities::r#enum::partner_type::PartnerType;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "partners")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub name: String,
    pub code: String, // Unique partner code
    pub partner_type: PartnerType,
    pub parent_partner_id: Option<i32>, // For hierarchical partners
    pub is_main_partner: bool, // Your company flag
    pub is_active: bool,
    pub settings: Json, // Partner specific settings
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user::Entity")]
    Users,
    #[sea_orm(has_many = "super::partner_api_access::Entity")]
    ApiAccess,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}