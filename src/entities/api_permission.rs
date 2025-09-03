// src/entities/api_permission.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::entities::r#enum::http_method::HttpMethod;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "api_permissions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub endpoint: String, // API endpoint pattern
    pub method: HttpMethod,
    pub module: String, // Module name
    pub required_permissions: Json, // Array of permission IDs
    pub description: Option<String>,
    pub created_at: DateTime,
}



#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::partner_api_access::Entity")]
    PartnerApiAccess,
}

impl ActiveModelBehavior for ActiveModel {}