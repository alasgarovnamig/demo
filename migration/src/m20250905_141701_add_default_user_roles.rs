use sea_orm_migration::{prelude::*, schema::*};
use crate::m20250902_140227_create_user_roles_table::UserRoles;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let user_role_assignments: Vec<(i32,i32, i32, &str)> = vec![
            // UPA USERS
            (1,1, 1, "UPA Admin'e süper admin rolü"),
            // ("upa_api", "api_user", "UPA API user'a sistem entegrasyon rolü"),
            // ("upa_portal", "operations_manager", "UPA Portal user'a operasyon yönetim rolü"),

            // // MOCRYPT USERS
            // ("mocrypt_admin", "partner_admin", "Mocrypt Admin'e partner admin rolü"),
            // ("mocrypt_api", "api_user", "Mocrypt API user'a sistem entegrasyon rolü"),
            // ("mocrypt_portal", "finance_manager", "Mocrypt Portal user'a finans yönetim rolü (kripto işlemler için)"),
            //
            // // PASHA HOLDING USERS
            // ("pasha_admin", "partner_admin", "Pasha Admin'e partner admin rolü"),
            // ("pasha_admin", "cross_partner_operator", "Pasha Admin'e alt şirket erişim rolü"),
            // ("pasha_api", "api_user", "Pasha API user'a sistem entegrasyon rolü"),
            //
            // // PASHABANK USERS
            // ("pashabank_portal", "finance_manager", "PashaBank Portal user'a finans yönetim rolü"),
            // ("pashabank_api", "api_user", "PashaBank API user'a sistem entegrasyon rolü"),
            //
            // // PASHA SIGORTA USERS
            // ("sigorta_portal", "operations_manager", "Pasha Sigorta Portal user'a operasyon yönetim rolü"),
            // ("sigorta_api", "api_user", "Pasha Sigorta API user'a sistem entegrasyon rolü"),
        ];
        for (id, user_id, role_id, description) in user_role_assignments {
            let insert_stmt = Query::insert()
                .into_table(UserRoles::Table)
                .columns([
                    UserRoles::Id,
                    UserRoles::UserId,
                    UserRoles::RoleId,
                    UserRoles::AssignedAt,
                ])
                .values_panic([
                    id.into(),
                    user_id.into(),
                    role_id.into(),
                    Expr::value(Func::cust("NOW")),
                ])
                .to_owned();

            manager.get_connection().execute(manager.get_database_backend().build(&insert_stmt)).await?;
            println!("✓ Assigned role {} to user {}: {}", role_id, user_id, description);
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_stmt = Query::delete()
            .from_table(UserRoles::Table)
            .to_owned();

        manager.get_connection().execute(manager.get_database_backend().build(&delete_stmt)).await?;
        println!("✓ All user role assignments removed!");

        Ok(())
    }
}

