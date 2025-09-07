use sea_orm_migration::{prelude::*, schema::*};
use crate::m20250902_135953_create_permissions_table::Permissions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // let roles: Vec<(i32, i32, &str, &str, &str, i32)> = vec![
        //     (1, 1, "super_admin", "Super Admin", "Full system access - only for UPA", 1),
        //     (2, 1, "partner_admin", "Partner Admin", "Full partner management access", 1),
        //     (3, 1, "api_user", "API User", "System integration user", 1),
        //     (4, 1, "user_manager", "User Manager", "Manage users and roles", 1),
        //     // ("finance_manager", "Finance Manager", "Manage financial operations", true),
        //     // ("finance_viewer", "Finance Viewer", "View financial data only", true),
        //     // ("operations_manager", "Operations Manager", "Manage daily operations", true),
        //     // ("operations_viewer", "Operations Viewer", "View operations data", true),
        //     // ("report_manager", "Report Manager", "Create and export reports", true),
        //     // ("report_viewer", "Report Viewer", "View reports only", true),
        //     // ("cross_partner_operator", "Cross Partner Operator", "Operate across child partners", true),
        // ];
        let permissions: Vec<(i32,&str, &str, &str, &str, &str,)> = vec![
            // PARTNER İZİNLERİ
            (1,"partners:create", "partners", "create", "all", "Yeni partner oluşturabilir - sadece UPA"),
            (2,"partners:read", "partners", "read", "partner", "Partner bilgilerini görüntüleyebilir"),
            (3,"partners:update", "partners", "update", "partner", "Partner bilgilerini güncelleyebilir"),
            (4,"partners:delete", "partners", "delete", "all", "Partner silebilir - sadece UPA"),


            // USER İZİNLERİ
            (5,"users:create", "users", "create", "partner", "Kendi partnerına kullanıcı ekleyebilir"),
            (6,"users:read", "users", "read", "partner", "Kullanıcıları görüntüleyebilir"),
            (7,"users:update", "users", "update", "own", "Kullanıcı bilgilerini güncelleyebilir"),
            (8,"users:delete", "users", "delete", "partner", "Kullanıcı silebilir"),

            // ROL İZİNLERİ
            (9,"roles:read", "roles", "read", "partner", "Rolleri görüntüleyebilir"),
            (10,"roles:assign", "roles", "assign", "partner", "Kullanıcılara rol atayabilir"),
            (11,"roles:create", "roles", "create", "partner", "Yeni rol oluşturabilir"),
            (12,"roles:update", "roles", "update", "partner", "Rol güncelleyebilir"),
            (13,"roles:delete", "roles", "delete", "partner", "Rol silebilir"),

            // // FATURA İZİNLERİ
            // ("invoices:create", "invoices", "create", "partner", "Fatura oluşturabilir"),
            // ("invoices:read", "invoices", "read", "partner", "Faturaları görüntüleyebilir"),
            // ("invoices:update", "invoices", "update", "partner", "Fatura düzenleyebilir"),
            // ("invoices:delete", "invoices", "delete", "partner", "Fatura silebilir"),
            // ("invoices:approve", "invoices", "approve", "partner", "Fatura onaylayabilir"),
            // ("invoices:export", "invoices", "export", "partner", "Faturaları dışa aktarabilir"),
            //
            // // İŞLEM İZİNLERİ
            // ("transactions:create", "transactions", "create", "partner", "İşlem oluşturabilir"),
            // ("transactions:read", "transactions", "read", "partner", "İşlemleri görüntüleyebilir"),
            // ("transactions:update", "transactions", "update", "partner", "İşlem güncelleyebilir"),
            // ("transactions:approve", "transactions", "approve", "partner", "İşlem onaylayabilir"),
            // ("transactions:export", "transactions", "export", "partner", "İşlemleri dışa aktarabilir"),
            //
            // // OPERASYON İZİNLERİ
            // ("operations:create", "operations", "create", "partner", "Operasyon oluşturabilir"),
            // ("operations:read", "operations", "read", "partner", "Operasyonları görüntüleyebilir"),
            // ("operations:update", "operations", "update", "partner", "Operasyon güncelleyebilir"),
            // ("operations:delete", "operations", "delete", "partner", "Operasyon silebilir"),
            // ("operations:approve", "operations", "approve", "partner", "Operasyon onaylayabilir"),
            //
            // // RAPOR İZİNLERİ
            // ("reports:read", "reports", "read", "partner", "Raporları görüntüleyebilir"),
            // ("reports:create", "reports", "create", "partner", "Rapor oluşturabilir"),
            // ("reports:export", "reports", "export", "partner", "Rapor dışa aktarabilir"),
        ];

        for (id, key, resource, action, scope_str, description) in permissions {
            let insert_stmt = Query::insert()
                .into_table(Permissions::Table)
                .columns([
                    Permissions::Id,
                    Permissions::Resource,
                    Permissions::Action,
                    Permissions::Scope,
                    // Permissions::Conditions,
                    Permissions::Description,
                    Permissions::CreatedAt,
                ])
                .values_panic([
                    id.into(),
                    resource.into(),
                    action.into(),
                    scope_str.into(),
                    description.into(),
                    Expr::value(Func::cust("NOW")),
                ])
                .to_owned();

            manager.exec_stmt(insert_stmt).await?;
            println!("✓ Created Permission: {} (ID: {})", key, id);
        }
        Ok(())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_stmt = Query::delete()
            .from_table(Permissions::Table)
            .cond_where(Expr::col(Permissions::Id).is_in([1, 2, 3, 4,5,6,7,8,9,10,11,12,13]))
            .to_owned();

        manager.exec_stmt(delete_stmt).await?;
        println!("✓ Roles seed reverted!");

        Ok(())
    }
}