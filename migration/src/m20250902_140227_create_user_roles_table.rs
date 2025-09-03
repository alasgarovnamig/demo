use crate::m20250902_135450_create_users_table::Users;
use crate::m20250902_135805_create_roles_table::Roles;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create user_roles table
        manager
            .create_table(
                Table::create()
                    .table(UserRoles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRoles::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserRoles::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserRoles::RoleId).uuid().not_null())
                    .col(ColumnDef::new(UserRoles::AssignedBy).uuid())
                    .col(ColumnDef::new(UserRoles::AssignedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserRoles::Table, UserRoles::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserRoles::Table, UserRoles::RoleId)
                            .to(Roles::Table, Roles::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique index for user-role combination
        manager
            .create_index(
                Index::create()
                    .name("idx_user_roles_unique")
                    .table(UserRoles::Table)
                    .col(UserRoles::UserId)
                    .col(UserRoles::RoleId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRoles::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum UserRoles {
    Table,
    Id,
    UserId,
    RoleId,
    AssignedBy,
    AssignedAt,
}
