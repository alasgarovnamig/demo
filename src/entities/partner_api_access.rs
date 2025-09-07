// src/entities/partner_api_access.rs
use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "partner_api_access")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub partner_id: i32,
    pub api_permission_id: i32,
    pub is_granted: bool,
    pub granted_by: Option<i32>,
    pub granted_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::partner::Entity",
        from = "Column::PartnerId",
        to = "super::partner::Column::Id"
    )]
    Partner,
    #[sea_orm(
        belongs_to = "super::api_permission::Entity",
        from = "Column::ApiPermissionId",
        to = "super::api_permission::Column::Id"
    )]
    ApiPermission,
}

impl Related<super::partner::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Partner.def()
    }
}

impl Related<super::api_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApiPermission.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}