use crate::m20250902_135115_create_partners_table::Partners;
use crate::m20250902_135450_create_users_table::Users;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create audit_logs table
        manager
            .create_table(
                Table::create()
                    .table(AuditLogs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AuditLogs::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AuditLogs::UserId).uuid())
                    .col(ColumnDef::new(AuditLogs::PartnerId).uuid())
                    .col(ColumnDef::new(AuditLogs::Action).string().not_null())
                    .col(ColumnDef::new(AuditLogs::Resource).string().not_null())
                    .col(ColumnDef::new(AuditLogs::ResourceId).string())
                    .col(ColumnDef::new(AuditLogs::Changes).json())
                    .col(ColumnDef::new(AuditLogs::IpAddress).string())
                    .col(ColumnDef::new(AuditLogs::UserAgent).string())
                    .col(ColumnDef::new(AuditLogs::CreatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(AuditLogs::Table, AuditLogs::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AuditLogs::Table, AuditLogs::PartnerId)
                            .to(Partners::Table, Partners::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes for performance
        manager
            .create_index(
                Index::create()
                    .name("idx_users_partner_id")
                    .table(Users::Table)
                    .col(Users::PartnerId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_audit_logs_user_id")
                    .table(AuditLogs::Table)
                    .col(AuditLogs::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_audit_logs_created_at")
                    .table(AuditLogs::Table)
                    .col(AuditLogs::CreatedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuditLogs::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum AuditLogs {
    Table,
    Id,
    UserId,
    PartnerId,
    Action,
    Resource,
    ResourceId,
    Changes,
    IpAddress,
    UserAgent,
    CreatedAt,
}
