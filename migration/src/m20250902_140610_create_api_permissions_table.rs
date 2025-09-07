use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create api_permissions table
        manager
            .create_table(
                Table::create()
                    .table(ApiPermissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ApiPermissions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ApiPermissions::Endpoint).string().not_null())
                    .col(ColumnDef::new(ApiPermissions::Method).string().not_null())
                    .col(ColumnDef::new(ApiPermissions::Module).string().not_null())
                    .col(
                        ColumnDef::new(ApiPermissions::RequiredPermissions)
                            .json()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ApiPermissions::Description).string())
                    .col(
                        ColumnDef::new(ApiPermissions::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ApiPermissions::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ApiPermissions {
    Table,
    Id,
    Endpoint,
    Method,
    Module,
    RequiredPermissions,
    Description,
    CreatedAt,
}
