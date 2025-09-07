use crate::m20250902_135115_create_partners_table::Partners;
use Value::Json;
use sea_orm_migration::prelude::*;
use serde_json::json;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let partners: Vec<(i32, &str, &str, &str, Option<i32>, bool, serde_json::Value)> = vec![
            (
                1,
                "United Payment Azerbaijan",
                "UPA",
                "main",
                None,
                true,
                json!({
                    "features": ["all"],
                    "modules": ["partners", "users", "finance", "operations", "reports"],
                    "country": "AZ",
                    "currency": "AZN"
                }),
            ),
            (
                2,
                "Mocrypt",
                "MCR",
                "standard",
                None,
                false,
                json!({
                    "features": ["crypto_payments", "wallet_management"],
                    "modules": ["invoices", "transactions", "reports"],
                    "industry": "fintech",
                    "type": "crypto"
                }),
            ),
            // (
            //     3,
            //     "Pasha Holding",
            //     "PASHA_HOLD",
            //     "standard",
            //     None,
            //     false,
            //     json!({
            //         "features": ["multi_company", "consolidated_reporting"],
            //         "modules": ["partners", "users", "invoices", "transactions", "reports"],
            //         "industry": "holding",
            //         "subsidiaries_count": 2
            //     }),
            // ),
            // (
            //     4,
            //     "PashaBank",
            //     "PASHABANK",
            //     "Subsidiary",
            //     Some(3),
            //     false,
            //     json!({
            //         "features": ["banking", "loans", "cards"],
            //         "modules": ["invoices", "transactions", "reports"],
            //         "industry": "banking",
            //         "parent": "PASHA_HOLD"
            //     }),
            // ),
            // (
            //     5,
            //     "Pasha Sigorta",
            //     "PASHA_SIG",
            //     "Subsidiary",
            //     Some(3),
            //     false,
            //     json!({
            //         "features": ["insurance", "claims"],
            //         "modules": ["invoices", "transactions", "reports"],
            //         "industry": "insurance",
            //         "parent": "PASHA_HOLD"
            //     }),
            // ),
        ];

        for (id, name, code, partner_type, parent_id, is_main, settings) in partners {
            let insert_stmt = Query::insert()
                .into_table(Partners::Table)
                .columns([
                    Partners::Id,
                    Partners::Name,
                    Partners::Code,
                    Partners::PartnerType,
                    Partners::ParentPartnerId,
                    Partners::IsMainPartner,
                    Partners::IsActive,
                    Partners::CreatedAt,
                    Partners::UpdatedAt,
                    Partners::Settings,
                ])
                .values_panic([
                    id.into(),
                    name.into(),
                    code.into(),
                    partner_type.into(),
                    match parent_id {
                        Some(pid) => pid.into(),
                        None => None::<i32>.into(),
                    },
                    is_main.into(),
                    true.into(),
                    // 🔥 Zaman damgaları (PostgreSQL)
                    Expr::value(Func::cust("NOW")),
                    Expr::value(Func::cust("NOW")),
                    Json(Option::from(Box::new(settings))).into(), // Expr::cust(format!("'{}'::json", settings.to_string().replace('\'', "''")))
                ])
                .to_owned();

            manager.exec_stmt(insert_stmt).await?;
            println!("✓ Created: {} (ID: {})", name, id);
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_stmt = Query::delete()
            .from_table(Partners::Table)
            .cond_where(Expr::col(Partners::Id).is_in([1]))
            .to_owned();

        manager.exec_stmt(delete_stmt).await?;
        println!("✓ All partners seed reverted!");

        Ok(())
    }
}
