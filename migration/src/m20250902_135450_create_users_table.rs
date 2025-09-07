use crate::m20250902_135115_create_partners_table::Partners;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create users table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // .col(ColumnDef::new(Users::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Users::PartnerId).integer().not_null())
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    // .col(ColumnDef::new(Users::Username).string().not_null())
                    .col(ColumnDef::new(Users::PasswordHash).string().not_null())
                    .col(ColumnDef::new(Users::UserType).string().not_null())
                    .col(
                        ColumnDef::new(Users::IsSystemUser)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Users::IsAdmin)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Users::CanAccessAllPartners)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Users::CreatedBy).integer())
                    .col(
                        ColumnDef::new(Users::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(Users::LastLogin).timestamp())
                    .col(ColumnDef::new(Users::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Users::Table, Users::PartnerId)
                            .to(Partners::Table, Partners::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    PartnerId,
    Email,
    Username,
    PasswordHash,
    UserType,
    IsSystemUser,
    IsAdmin,
    CanAccessAllPartners,
    CreatedBy,
    IsActive,
    LastLogin,
    CreatedAt,
    UpdatedAt,
}
