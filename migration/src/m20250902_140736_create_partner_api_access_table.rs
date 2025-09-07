use crate::m20250902_135115_create_partners_table::Partners;
use crate::m20250902_140610_create_api_permissions_table::ApiPermissions;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create partner_api_access table
        manager
            .create_table(
                Table::create()
                    .table(PartnerApiAccess::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PartnerApiAccess::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PartnerApiAccess::PartnerId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PartnerApiAccess::ApiPermissionId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PartnerApiAccess::IsGranted)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(PartnerApiAccess::GrantedBy).integer())
                    .col(
                        ColumnDef::new(PartnerApiAccess::GrantedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PartnerApiAccess::Table, PartnerApiAccess::PartnerId)
                            .to(Partners::Table, Partners::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PartnerApiAccess::Table, PartnerApiAccess::ApiPermissionId)
                            .to(ApiPermissions::Table, ApiPermissions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PartnerApiAccess::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum PartnerApiAccess {
    Table,
    Id,
    PartnerId,
    ApiPermissionId,
    IsGranted,
    GrantedBy,
    GrantedAt,
}
