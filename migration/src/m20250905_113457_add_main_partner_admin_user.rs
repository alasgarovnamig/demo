use sea_orm_migration::prelude::*;
use sea_query::{Expr, Func, Query};
use crate::m20250902_135450_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let users: Vec<(i32, i32, &str, &str, &str)> = vec![
            (
                1, // id
                1, // partner_id (United Payment Azerbaijan)
                "admin@unitedpayment.com",
                "$2b$12$BI67ZClbpeqnMijl9KRbPusCSIpMQeidW6SUWdHYN3aOB8Nu6Jnzu", // bcrypt hash of: Up@!735538?=@1993!
                "portal",
            ),
        ];

        for (id, partner_id, email, password_hash, user_type) in users {
            let insert_stmt = Query::insert()
                .into_table(Users::Table)
                .columns([
                    Users::Id,
                    Users::PartnerId,
                    Users::Email,
                    Users::PasswordHash,
                    Users::UserType,
                    Users::IsSystemUser,
                    Users::IsAdmin,
                    Users::CanAccessAllPartners,
                    Users::IsActive,
                    Users::CreatedAt,
                    Users::UpdatedAt,
                ])
                .values_panic([
                    id.into(),
                    partner_id.into(),
                    email.into(),
                    password_hash.into(),
                    user_type.into(),
                    false.into(), // is_system_user
                    true.into(),  // is_admin
                    true.into(),  // can_access_all_partners
                    true.into(),  // is_active
                    Expr::value(Func::cust("NOW")),
                    Expr::value(Func::cust("NOW")),
                ])
                .to_owned();

            manager.exec_stmt(insert_stmt).await?;
            println!("✓ Created User: {} (ID: {})", email, id);
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_stmt = Query::delete()
            .from_table(Users::Table)
            .cond_where(Expr::col(Users::Id).is_in([1]))
            .to_owned();

        manager.exec_stmt(delete_stmt).await?;
        println!("✓ Users seed reverted!");

        Ok(())
    }
}
