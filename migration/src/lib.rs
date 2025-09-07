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
            Box::new(m20250905_095624_add_main_partner::Migration),
            Box::new(m20250905_113457_add_main_partner_admin_user::Migration),
            Box::new(m20250905_123641_default_roles::Migration),
            Box::new(m20250905_130736_add_default_permissions::Migration),
            Box::new(m20250905_135258_add_default_role_permissions::Migration),
            Box::new(m20250905_141701_add_default_user_roles::Migration),
            Box::new(m20250905_145214_add_default_api_permissions::Migration),
            Box::new(m20250905_170243_add_default_partner_api_accesses::Migration),
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
mod m20250905_095624_add_main_partner;
mod m20250905_113457_add_main_partner_admin_user;
mod m20250905_123641_default_roles;
mod m20250905_130736_add_default_permissions;
mod m20250905_135258_add_default_role_permissions;
mod m20250905_141701_add_default_user_roles;
mod m20250905_145214_add_default_api_permissions;
mod m20250905_170243_add_default_partner_api_accesses;


