use crate::m20250902_135805_create_roles_table::Roles;
use sea_orm_migration::{prelude::*, schema::*};
use sea_query::{Expr, Func, Query};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let roles: Vec<(i32, i32, &str, &str, &str, i32)> = vec![
            (1, 1, "super_admin", "Super Admin", "Full system access - only for UPA", 1),
            (2, 1, "partner_admin", "Partner Admin", "Full partner management access", 1),
            (3, 1, "api_user", "API User", "System integration user", 1),
            (4, 1, "user_manager", "User Manager", "Manage users and roles", 1),
            // ("finance_manager", "Finance Manager", "Manage financial operations", true),
            // ("finance_viewer", "Finance Viewer", "View financial data only", true),
            // ("operations_manager", "Operations Manager", "Manage daily operations", true),
            // ("operations_viewer", "Operations Viewer", "View operations data", true),
            // ("report_manager", "Report Manager", "Create and export reports", true),
            // ("report_viewer", "Report Viewer", "View reports only", true),
            // ("cross_partner_operator", "Cross Partner Operator", "Operate across child partners", true),
        ];

        for (id, partner_id, name, code, description, created_by) in roles {
            let insert_stmt = Query::insert()
                .into_table(Roles::Table)
                .columns([
                    Roles::Id,
                    Roles::PartnerId,
                    Roles::Name,
                    Roles::Code,
                    Roles::Description,
                    Roles::IsSystemRole,
                    Roles::CreatedBy,
                    Roles::CreatedAt,
                    Roles::UpdatedAt,
                ])
                .values_panic([
                    id.into(),
                    partner_id.into(),
                    name.into(),
                    code.into(),   
                    description.into(),
                    true.into(),       // is_system
                    created_by.into(), // is_system_user
                    Expr::value(Func::cust("NOW")),
                    Expr::value(Func::cust("NOW")),
                ])
                .to_owned();

            manager.exec_stmt(insert_stmt).await?;
            println!("✓ Created Role: {} (ID: {})", name, id);
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_stmt = Query::delete()
            .from_table(Roles::Table)
            .cond_where(Expr::col(Roles::Id).is_in([1, 2, 3, 4]))
            .to_owned();

        manager.exec_stmt(delete_stmt).await?;
        println!("✓ Roles seed reverted!");

        Ok(())
    }
}
