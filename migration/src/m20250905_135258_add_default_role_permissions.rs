use sea_orm_migration::{prelude::*, schema::*};
use crate::m20250902_140431_create_role_permissions_table::RolePermissions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let role_permissions = vec![
            // SUPER ADMIN - HER ŞEYE ERİŞİM
            (1, vec![1,2,3,4,5,6,7,8,9,10,11,12,13]),
            // ("super_admin", vec![
            //     "partners:create", "partners:read", "partners:update", "partners:delete",
            //     "users:create", "users:read", "users:update", "users:delete",
            //     "roles:read", "roles:assign", "roles:create", "roles:update", "roles:delete",
            //     "invoices:create", "invoices:read", "invoices:update", "invoices:delete", "invoices:approve", "invoices:export",
            //     "transactions:create", "transactions:read", "transactions:update", "transactions:approve", "transactions:export",
            //     "operations:create", "operations:read", "operations:update", "operations:delete", "operations:approve",
            //     "reports:read", "reports:create", "reports:export",
            // ]),


            // PARTNER ADMIN - KENDİ PARTNERİNDE HER ŞEY
            (2, vec![2,3,5,6,7,8,9,10]),
            // ("partner_admin", vec![
            //     "partners:read", "partners:update",
            //     "users:create", "users:read", "users:update", "users:delete",
            //     "roles:read", "roles:assign",
            //     "invoices:create", "invoices:read", "invoices:update", "invoices:delete", "invoices:approve", "invoices:export",
            //     "transactions:create", "transactions:read", "transactions:update", "transactions:approve", "transactions:export",
            //     "operations:create", "operations:read", "operations:update", "operations:delete", "operations:approve",
            //     "reports:read", "reports:create", "reports:export",
            // ]),

            // API USER - SİSTEM ENTEGRASYONU
            (3, vec![2,6]),
            // ("api_user", vec![
            //     "partners:read",
            //     "users:read",
            //     "invoices:create", "invoices:read", "invoices:update",
            //     "transactions:create", "transactions:read",
            //     "operations:read",
            //     "reports:read",
            // ]),

            // // FINANCE MANAGER - MALİ İŞLEMLER YÖNETİMİ
            // ("finance_manager", vec![
            //     "invoices:create", "invoices:read", "invoices:update", "invoices:approve", "invoices:export",
            //     "transactions:create", "transactions:read", "transactions:update", "transactions:approve", "transactions:export",
            //     "reports:read", "reports:create", "reports:export",
            // ]),
            //
            // // FINANCE VIEWER - MALİ VERİLERİ SADECE GÖRME
            // ("finance_viewer", vec![
            //     "invoices:read",
            //     "transactions:read",
            //     "reports:read",
            // ]),
            //
            // // OPERATIONS MANAGER - OPERASYON YÖNETİMİ
            // ("operations_manager", vec![
            //     "operations:create", "operations:read", "operations:update", "operations:delete", "operations:approve",
            //     "reports:read", "reports:create",
            // ]),
            //
            // // OPERATIONS VIEWER - OPERASYON GÖRÜNTÜLEME
            // ("operations_viewer", vec![
            //     "operations:read",
            //     "reports:read",
            // ]),

            // USER MANAGER - KULLANICI YÖNETİMİ
            (4, vec![5,6,7,8,9,10]),
            // ("user_manager", vec![
            //     "users:create", "users:read", "users:update", "users:delete",
            //     "roles:read", "roles:assign",
            // ]),

            // // REPORT MANAGER - RAPOR YÖNETİMİ
            // ("report_manager", vec![
            //     "reports:read", "reports:create", "reports:export",
            // ]),
            //
            // // REPORT VIEWER - RAPOR GÖRÜNTÜLEME
            // ("report_viewer", vec![
            //     "reports:read",
            // ]),
            //
            // // CROSS PARTNER OPERATOR - ALT ŞİRKETLERDE İŞLEM
            // ("cross_partner_operator", vec![
            //     "partners:read",
            //     "users:read",
            //     "invoices:read", "invoices:create",
            //     "transactions:read", "transactions:create",
            //     "operations:read",
            //     "reports:read",
            // ]),
        ];
        let mut counter: i32 = 1;

        for (role_id, permission_ids) in role_permissions {
            let permission_count = permission_ids.len();
            for permission_id in permission_ids {
                let insert_stmt = Query::insert()
                    .into_table(RolePermissions::Table)
                    .columns([
                        RolePermissions::Id,
                        RolePermissions::RoleId,
                        RolePermissions::PermissionId,
                        RolePermissions::GrantedAt,
                    ])
                    .values_panic([
                        counter.into(),
                        role_id.into(),
                        permission_id.into(),
                        Expr::value(Func::cust("NOW")),
                    ])
                    .to_owned();

                manager.get_connection().execute(manager.get_database_backend().build(&insert_stmt)).await?;
                counter += 1;
            }
            println!("✓ Assigned {} permissions to role ID: {}", permission_count, role_id);
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_stmt = Query::delete()
            .from_table(RolePermissions::Table)
            .to_owned();

        manager.get_connection().execute(manager.get_database_backend().build(&delete_stmt)).await?;
        println!("✓ All role permissions removed!");

        Ok(())
    }
}

