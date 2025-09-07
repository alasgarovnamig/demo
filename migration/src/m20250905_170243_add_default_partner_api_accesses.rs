use sea_orm_migration::{prelude::*, schema::*};
use crate::m20250902_140736_create_partner_api_access_table::PartnerApiAccess;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let partner_api_access = vec![
            // UPA - HER ŞEYİ KULLANABİLİR
            (1,vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18],"United Payment tüm API'lara erişebilir")
            // ("UPA", vec![
            //     "partner_list", "partner_create", "partner_get", "partner_update", "partner_delete", "partner_users", "partner_stats",
            //     "user_list", "user_create", "user_get", "user_update", "user_delete", "user_permissions",
            //     "role_list", "role_create", "role_assign", "role_permissions",
            //     "permission_list",
            // ], "United Payment tüm API'lara erişebilir"),

            // // MOCRYPT - STANDART MÜŞTERİ ERİŞİMİ
            // ("MOCRYPT", vec![
            //     "partner_get", "partner_users", "partner_stats",
            //     "user_list", "user_create", "user_get", "user_update", "user_delete", "user_permissions",
            //     "role_list", "role_assign", "role_permissions",
            //     "permission_list",
            // ], "Mocrypt standart müşteri API erişimi"),
            //
            // // PASHA HOLDING - HOLDİNG ERİŞİMİ (ALT ŞİRKETLER DAHİL)
            // ("PASHA_HOLD", vec![
            //     "partner_list", "partner_get", "partner_users", "partner_stats",
            //     "user_list", "user_create", "user_get", "user_update", "user_delete", "user_permissions",
            //     "role_list", "role_assign", "role_permissions",
            //     "permission_list",
            // ], "Pasha Holding ve alt şirket API erişimi"),
            //
            // // PASHABANK - BANKA ERİŞİMİ
            // ("PASHABANK", vec![
            //     "partner_get", "partner_stats",
            //     "user_list", "user_get", "user_permissions",
            //     "role_list", "role_permissions",
            //     "permission_list",
            // ], "PashaBank sınırlı API erişimi"),
            //
            // // PASHA SIGORTA - SİGORTA ERİŞİMİ
            // ("PASHA_SIG", vec![
            //     "partner_get", "partner_stats",
            //     "user_list", "user_get", "user_permissions",
            //     "role_list", "role_permissions",
            //     "permission_list",
            // ], "Pasha Sigorta sınırlı API erişimi"),
        ];
        let mut counter: i32 = 1;
        for (partner_id, api_permission_ids, description) in partner_api_access {
                for api_permission_id in api_permission_ids {
                    let insert_stmt = Query::insert()
                        .into_table(PartnerApiAccess::Table)
                        .columns([
                            PartnerApiAccess::Id,
                            PartnerApiAccess::PartnerId,
                            PartnerApiAccess::ApiPermissionId,
                            PartnerApiAccess::IsGranted,
                            PartnerApiAccess::GrantedAt,
                        ])
                        .values_panic([
                            counter.into(),
                            partner_id.into(),
                            api_permission_id.into(),
                            true.into(),
                            Expr::value(Func::cust("NOW")),
                        ])
                        .to_owned();

                    manager.get_connection().execute(manager.get_database_backend().build(&insert_stmt)).await?;
                    counter += 1;
                }
                // println!("  ✓ {}: {} → {} API endpoints", description, partner_key, api_keys.len());
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_stmt = Query::delete()
            .from_table(PartnerApiAccess::Table)
            .to_owned();

        manager.get_connection().execute(manager.get_database_backend().build(&delete_stmt)).await?;
        println!("✓ All partner api accesses removed!");

        Ok(())
    }
}
