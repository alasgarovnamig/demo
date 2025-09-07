use serde_json::json;
use sea_orm_migration::{prelude::*, schema::*};
use crate::m20250902_140610_create_api_permissions_table::ApiPermissions;
use crate::Value::Json;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let api_permissions = vec![
            // PARTNER API'LARI
            (1,"partner_list", "/api/partners", "GET", "partners", json!(["partners:read"]), "Partner listesini görüntüleme"),
            (2,"partner_create", "/api/partners", "POST", "partners", json!(["partners:create"]), "Yeni partner oluşturma"),
            (3,"partner_get", "/api/partners/*", "GET", "partners", json!(["partners:read"]), "Partner detayını görüntüleme"),
            (4,"partner_update", "/api/partners/*", "PUT", "partners", json!(["partners:update"]), "Partner güncelleme"),
            (5,"partner_delete", "/api/partners/*", "DELETE", "partners", json!(["partners:delete"]), "Partner silme"),
            (6,"partner_users", "/api/partners/*/users", "GET", "partners", json!(["users:read"]), "Partner kullanıcılarını listeleme"),
            (7,"partner_stats", "/api/partners/*/stats", "GET", "partners", json!(["partners:read"]), "Partner istatistiklerini görme"),
            
            // USER API'LARI
            (8,"user_list", "/api/users", "GET", "users", json!(["users:read"]), "Kullanıcı listesini görüntüleme"),
            (9,"user_create", "/api/users", "POST", "users", json!(["users:create"]), "Yeni kullanıcı oluşturma"),
            (10,"user_get", "/api/users/*", "GET", "users", json!(["users:read"]), "Kullanıcı detayını görüntüleme"),
            (11,"user_update", "/api/users/*", "PUT", "users", json!(["users:update"]), "Kullanıcı güncelleme"),
            (12,"user_delete", "/api/users/*", "DELETE", "users", json!(["users:delete"]), "Kullanıcı silme"),
            (13,"user_permissions", "/api/users/*/permissions", "GET", "users", json!(["users:read"]), "Kullanıcı yetkilerini görme"),
            
            // ROLE API'LARI
            (14,"role_list", "/api/roles", "GET", "roles", json!(["roles:read"]), "Rol listesini görüntüleme"),
            (15,"role_create", "/api/roles", "POST", "roles", json!(["roles:create"]), "Yeni rol oluşturma"),
            (16,"role_assign", "/api/users/*/roles/*", "POST", "roles", json!(["roles:assign"]), "Kullanıcıya rol atama"),
            (17,"role_permissions", "/api/roles/*/permissions", "GET", "roles", json!(["roles:read"]), "Rol yetkilerini görüntüleme"),
            
            // PERMISSION API'LARI
            (18,"permission_list", "/api/permissions", "GET", "permissions", json!(["roles:read"]), "Yetki listesini görüntüleme"),
            
            // AUTH API'LARI (Public - no permission needed)
            (19,"auth_login", "/auth/login", "POST", "auth", json!([]), "Kullanıcı girişi"),
            (20,"auth_refresh", "/auth/refresh", "POST", "auth", json!([]), "Token yenileme"),
            
            // HEALTH CHECK (Public)
            (21,"health", "/health", "GET", "health", json!([]), "Sistem sağlık kontrolü"),
        ];

        for (id,key, endpoint, method_str, module, required_perms, description) in api_permissions {

            let json_data = serde_json::to_string(&required_perms).unwrap();

            let insert_stmt = Query::insert()
                .into_table(ApiPermissions::Table)
                .columns([
                    ApiPermissions::Id,
                    ApiPermissions::Endpoint,
                    ApiPermissions::Method,
                    ApiPermissions::Module,
                    ApiPermissions::RequiredPermissions,
                    ApiPermissions::Description,
                    ApiPermissions::CreatedAt,
                ])
                .values_panic([
                    id.into(),
                    endpoint.into(),
                    method_str.into(),
                    module.into(),
                    Json(Option::from(Box::new(required_perms))).into(),
                    description.into(),
                    Expr::value(Func::cust("NOW")),
                ])
                .to_owned();

            manager.get_connection().execute(manager.get_database_backend().build(&insert_stmt)).await?;
            println!("  ✓ Created API permission: {} {} - {}", method_str, endpoint, description);
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_stmt = Query::delete()
            .from_table(ApiPermissions::Table) // Import edileni kullan
            .to_owned();

        manager.get_connection().execute(manager.get_database_backend().build(&delete_stmt)).await?;
        println!("✓ All API permissions removed!");

        Ok(())
    }
}

