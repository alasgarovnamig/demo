use crate::m20250902_135805_create_roles_table::Roles;
use crate::m20250902_135953_create_permissions_table::Permissions;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create role_permissions table
        manager
            .create_table(
                Table::create()
                    .table(RolePermissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RolePermissions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RolePermissions::RoleId).uuid().not_null())
                    .col(
                        ColumnDef::new(RolePermissions::PermissionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RolePermissions::GrantedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RolePermissions::Table, RolePermissions::RoleId)
                            .to(Roles::Table, Roles::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RolePermissions::Table, RolePermissions::PermissionId)
                            .to(Permissions::Table, Permissions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RolePermissions::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum RolePermissions {
    Table,
    Id,
    RoleId,
    PermissionId,
    GrantedAt,
}
