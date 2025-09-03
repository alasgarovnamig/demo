pub use sea_orm_migration::prelude::*;



pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250902_135115_create_partners_table::Migration),
            Box::new(m20250902_135450_create_users_table::Migration),
            Box::new(m20250902_135805_create_roles_table::Migration),
            Box::new(m20250902_135953_create_permissions_table::Migration),
            Box::new(m20250902_140227_create_user_roles_table::Migration),
            Box::new(m20250902_140431_create_role_permissions_table::Migration),
            Box::new(m20250902_140610_create_api_permissions_table::Migration),
            Box::new(m20250902_140736_create_partner_api_access_table::Migration),
            Box::new(m20250902_140858_create_audit_logs_table::Migration),
        ]
    }
}
mod m20250902_135115_create_partners_table;
mod m20250902_135450_create_users_table;
mod m20250902_135805_create_roles_table;
mod m20250902_135953_create_permissions_table;
mod m20250902_140227_create_user_roles_table;
mod m20250902_140431_create_role_permissions_table;
mod m20250902_140610_create_api_permissions_table;
mod m20250902_140736_create_partner_api_access_table;
mod m20250902_140858_create_audit_logs_table;
