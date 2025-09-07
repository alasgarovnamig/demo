use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create permissions table
        manager
            .create_table(
                Table::create()
                    .table(Permissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Permissions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Permissions::Resource).string().not_null())
                    .col(ColumnDef::new(Permissions::Action).string().not_null())
                    .col(ColumnDef::new(Permissions::Scope).string().not_null())
                    .col(ColumnDef::new(Permissions::Conditions).json())
                    .col(ColumnDef::new(Permissions::Description).string())
                    .col(
                        ColumnDef::new(Permissions::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique index for resource-action combination
        manager
            .create_index(
                Index::create()
                    .name("idx_permissions_resource_action")
                    .table(Permissions::Table)
                    .col(Permissions::Resource)
                    .col(Permissions::Action)
                    .col(Permissions::Scope)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Permissions::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Permissions {
    Table,
    Id,
    Resource,
    Action,
    Scope,
    Conditions,
    Description,
    CreatedAt,
}
