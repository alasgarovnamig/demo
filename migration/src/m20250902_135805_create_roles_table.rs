use crate::m20250902_135115_create_partners_table::Partners;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create roles table
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Roles::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // .col(ColumnDef::new(Roles::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Roles::PartnerId).integer())
                    .col(ColumnDef::new(Roles::Name).string().not_null())
                    .col(ColumnDef::new(Roles::Code).string().not_null().unique_key())
                    .col(ColumnDef::new(Roles::Description).string())
                    .col(
                        ColumnDef::new(Roles::IsSystemRole)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Roles::CreatedBy).integer())
                    .col(ColumnDef::new(Roles::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Roles::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Roles::Table, Roles::PartnerId)
                            .to(Partners::Table, Partners::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Roles::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Roles {
    Table,
    Id,
    PartnerId,
    Name,
    Code,
    Description,
    IsSystemRole,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
