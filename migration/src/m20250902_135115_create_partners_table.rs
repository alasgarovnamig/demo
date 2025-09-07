use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create partners table
        manager
            .create_table(
                Table::create()
                    .table(Partners::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Partners::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // .col(ColumnDef::new(Partners::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Partners::Name).string().not_null())
                    .col(
                        ColumnDef::new(Partners::Code)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Partners::PartnerType).string().not_null())
                    .col(ColumnDef::new(Partners::ParentPartnerId).integer())
                    .col(
                        ColumnDef::new(Partners::IsMainPartner)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Partners::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(Partners::Settings).json().not_null())
                    .col(ColumnDef::new(Partners::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Partners::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Partners::Table, Partners::ParentPartnerId)
                            .to(Partners::Table, Partners::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Partners::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Partners {
    Table,
    Id,
    Name,
    Code,
    PartnerType,
    ParentPartnerId,
    IsMainPartner,
    IsActive,
    Settings,
    CreatedAt,
    UpdatedAt,
}
