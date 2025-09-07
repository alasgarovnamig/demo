// // src/services/opa_data_sync.rs
// use crate::entities::{partner, permission, role, role_permission, user, user_role};
// use crate::error::AppError;
// use sea_orm::ColumnTrait;
// use sea_orm::QueryFilter;
// use sea_orm::{DatabaseConnection, EntityTrait};
// use serde::{Deserialize, Serialize};
// // use std::collections::HashMap;
// // use uuid::Uuid;
//
// #[derive(Clone)]
// pub struct OpaDataSync {
//     db: DatabaseConnection,
//     opa_url: String,
//     client: reqwest::Client,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct OpaData {
//     pub partners: Vec<PartnerData>,
//     pub users: Vec<UserData>,
//     pub roles: Vec<RoleData>,
//     pub permissions: Vec<PermissionData>,
//     pub user_roles: Vec<UserRoleData>,
//     pub role_permissions: Vec<RolePermissionData>,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct PartnerData {
//     pub id: i32,
//     pub code: String,
//     pub name: String,
//     pub is_main_partner: bool,
//     pub is_active: bool,
//     pub parent_partner_id: Option<i32>,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct UserData {
//     pub id: i32,
//     pub partner_id: i32,
//     pub email: String,
//     // pub username: String,
//     pub user_type: String,
//     pub is_admin: bool,
//     pub can_access_all_partners: bool,
//     pub is_active: bool,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct RoleData {
//     pub id: i32,
//     pub code: String,
//     pub name: String,
//     pub partner_id: Option<i32>,
//     pub is_system_role: bool,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct PermissionData {
//     pub id: i32,
//     pub resource: String,
//     pub action: String,
//     pub scope: String,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct UserRoleData {
//     pub user_id: i32,
//     pub role_id: i32,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct RolePermissionData {
//     pub role_id: i32,
//     pub permission_id: i32,
// }
//
// impl OpaDataSync {
//     pub fn new(db: DatabaseConnection, opa_url: String) -> Self {
//         Self {
//             db,
//             opa_url,
//             client: reqwest::Client::new(),
//         }
//     }
//
//     /// OPA'ya tüm verileri yükler (başlangıçta ve periyodik olarak)
//     pub async fn sync_all_data(&self) -> Result<(), AppError> {
//         println!("Starting OPA data synchronization...");
//
//         // Veritabanından tüm verileri çek
//         let opa_data = self.fetch_all_data().await?;
//
//         // OPA'ya gönder
//         self.push_to_opa(opa_data).await?;
//
//         println!("✓ OPA data synchronization completed");
//         Ok(())
//     }
//
//     /// Veritabanından tüm ilgili verileri çeker
//     async fn fetch_all_data(&self) -> Result<OpaData, AppError> {
//         // Partners
//         let partners = partner::Entity::find()
//             .all(&self.db)
//             .await?
//             .into_iter()
//             .map(|p| PartnerData {
//                 id: p.id,
//                 code: p.code,
//                 name: p.name,
//                 is_main_partner: p.is_main_partner,
//                 is_active: p.is_active,
//                 parent_partner_id: p.parent_partner_id,
//             })
//             .collect();
//
//         // Users
//         let users = user::Entity::find()
//             .all(&self.db)
//             .await?
//             .into_iter()
//             .map(|u| UserData {
//                 id: u.id,
//                 partner_id: u.partner_id,
//                 email: u.email,
//                 // username: u.username,
//                 user_type: u.user_type.into(),
//                 is_admin: u.is_admin,
//                 can_access_all_partners: u.can_access_all_partners,
//                 is_active: u.is_active,
//             })
//             .collect();
//
//         // Roles
//         let roles = role::Entity::find()
//             .all(&self.db)
//             .await?
//             .into_iter()
//             .map(|r| RoleData {
//                 id: r.id,
//                 code: r.code,
//                 name: r.name,
//                 partner_id: r.partner_id,
//                 is_system_role: r.is_system_role,
//             })
//             .collect();
//
//         // Permissions
//         let permissions = permission::Entity::find()
//             .all(&self.db)
//             .await?
//             .into_iter()
//             .map(|p| PermissionData {
//                 id: p.id,
//                 resource: p.resource,
//                 action: p.action,
//                 scope: p.scope.into(),
//             })
//             .collect();
//
//         // User-Role ilişkileri
//         let user_roles = user_role::Entity::find()
//             .all(&self.db)
//             .await?
//             .into_iter()
//             .map(|ur| UserRoleData {
//                 user_id: ur.user_id,
//                 role_id: ur.role_id,
//             })
//             .collect();
//
//         // Role-Permission ilişkileri
//         let role_permissions = role_permission::Entity::find()
//             .all(&self.db)
//             .await?
//             .into_iter()
//             .map(|rp| RolePermissionData {
//                 role_id: rp.role_id,
//                 permission_id: rp.permission_id,
//             })
//             .collect();
//
//         Ok(OpaData {
//             partners,
//             users,
//             roles,
//             permissions,
//             user_roles,
//             role_permissions,
//         })
//     }
//
//     /// OPA'ya veri gönderir
//     async fn push_to_opa(&self, data: OpaData) -> Result<(), AppError> {
//         // OPA'ya veri gönderme endpoint'i
//         let url = format!("{}/v1/data/system", self.opa_url);
//
//         let response = self
//             .client
//             .put(&url)
//             .json(&serde_json::json!({
//                 "partners": data.partners,
//                 "users": data.users,
//                 "roles": data.roles,
//                 "permissions": data.permissions,
//                 "user_roles": data.user_roles,
//                 "role_permissions": data.role_permissions,
//             }))
//             .send()
//             .await?;
//
//         if !response.status().is_success() {
//             let error_text = response
//                 .text()
//                 .await
//                 .unwrap_or_else(|_| "Unknown error".to_string());
//             return Err(AppError::ExternalService(format!(
//                 "OPA data sync failed: {}",
//                 error_text
//             )));
//         }
//
//         Ok(())
//     }
//
//     /// Belirli bir kullanıcının verilerini günceller
//     pub async fn sync_user_data(&self, user_id: i32) -> Result<(), AppError> {
//         // Kullanıcı bilgilerini çek
//         let user = user::Entity::find_by_id(user_id)
//             .one(&self.db)
//             .await?
//             .ok_or(AppError::NotFound("User not found".into()))?;
//
//         // Kullanıcının rollerini çek
//         let user_roles = user_role::Entity::find()
//             .filter(user_role::Column::UserId.eq(user_id))
//             .all(&self.db)
//             .await?;
//
//         // OPA'ya kullanıcı güncellemesi gönder
//         let url = format!("{}/v1/data/system/users/{}", self.opa_url, user_id);
//
//         self.client
//             .put(&url)
//             .json(&serde_json::json!({
//                 "id": user.id,
//                 "partner_id": user.partner_id,
//                 "email": user.email,
//                 // "username": user.username,
//                 "user_type": user.user_type.to_string(), // Burada değişiklik
//                 "is_admin": user.is_admin,
//                 "can_access_all_partners": user.can_access_all_partners,
//                 "is_active": user.is_active,
//                 "roles": user_roles.iter().map(|ur| ur.role_id).collect::<Vec<_>>(),
//             }))
//             .send()
//             .await?;
//
//         Ok(())
//     }
//
//     /// Belirli bir partner'ın verilerini günceller
//     pub async fn sync_partner_data(&self, partner_id: i32) -> Result<(), AppError> {
//         let partner = partner::Entity::find_by_id(partner_id)
//             .one(&self.db)
//             .await?
//             .ok_or(AppError::NotFound("Partner not found".into()))?;
//
//         let url = format!("{}/v1/data/system/partners/{}", self.opa_url, partner_id);
//
//         self.client
//             .put(&url)
//             .json(&serde_json::json!({
//                 "id": partner.id,
//                 "code": partner.code,
//                 "name": partner.name,
//                 "is_main_partner": partner.is_main_partner,
//                 "is_active": partner.is_active,
//                 "parent_partner_id": partner.parent_partner_id,
//             }))
//             .send()
//             .await?;
//
//         Ok(())
//     }
//
//     /// Role değişikliklerini senkronize eder
//     pub async fn sync_role_permissions(&self, role_id: i32) -> Result<(), AppError> {
//         let permissions = role_permission::Entity::find()
//             .filter(role_permission::Column::RoleId.eq(role_id))
//             .all(&self.db)
//             .await?;
//
//         let url = format!(
//             "{}/v1/data/system/role_permissions/{}",
//             self.opa_url, role_id
//         );
//
//         self.client
//             .put(&url)
//             .json(
//                 &permissions
//                     .iter()
//                     .map(|rp| rp.permission_id)
//                     .collect::<Vec<_>>(),
//             )
//             .send()
//             .await?;
//
//         Ok(())
//     }
// }


// src/services/opa_data_sync.rs
// Bu servis, PostgreSQL'deki verileri OPA'ya yükler ve senkronize tutar

// Gerekli kütüphaneleri import ediyoruz
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use uuid::Uuid;
use crate::entities::{
    partner,
    user,
    role,
    permission,
    user_role,
    role_permission,
    api_permission,
    partner_api_access
};
use crate::error::AppError;

// OPA sync servisimizin ana struct'ı
// Clone trait'i ekliyoruz ki kopyalanabilsin (background task için gerekli)
#[derive(Clone)]
pub struct OpaDataSync {
    db: DatabaseConnection,  // Veritabanı bağlantısı
    opa_url: String,        // OPA server'ın adresi (örn: http://localhost:8181)
    client: reqwest::Client, // HTTP istekleri için client
}

// OPA'ya gönderilecek tüm verileri içeren ana struct
#[derive(Debug, Serialize, Deserialize)]
pub struct OpaData {
    pub partners: Vec<PartnerData>,              // Tüm partner kayıtları
    pub users: Vec<UserData>,                    // Tüm kullanıcı kayıtları
    pub roles: Vec<RoleData>,                    // Tüm rol tanımları
    pub permissions: Vec<PermissionData>,        // Tüm izin tanımları
    pub user_roles: Vec<UserRoleData>,          // Hangi user'ın hangi rolü var
    pub role_permissions: Vec<RolePermissionData>, // Hangi rolün hangi izni var
    pub api_permissions: Vec<ApiPermissionData>,   // API endpoint'leri ve gereken izinler
    pub partner_api_access: Vec<PartnerApiAccessData>, // Hangi partner hangi API'yi kullanabilir
}

// Partner verisi - OPA'da partner kontrolü için
#[derive(Debug, Serialize, Deserialize)]
pub struct PartnerData {
    pub id: i32,                        // Partner'ın unique ID'si
    pub code: String,                    // Partner kodu (UPA, MOCRYPT vs)
    pub name: String,                    // Partner adı
    pub is_main_partner: bool,          // Ana şirket mi? (sadece UPA için true)
    pub is_active: bool,                 // Partner aktif mi?
    pub parent_partner_id: Option<i32>, // Üst partner varsa ID'si (PashaBank -> Pasha Holding)
}

// Kullanıcı verisi - OPA'da user kontrolü için
#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: i32,                       // User'ın unique ID'si
    pub partner_id: i32,               // Hangi partner'a ait
    pub email: String,                  // Email adresi (login için)
    // pub username: String,               // Kullanıcı adı
    pub user_type: String,              // User tipi (Admin, Portal, System, Operator)
    pub is_admin: bool,                 // Admin kullanıcı mı?
    pub can_access_all_partners: bool, // Tüm partnerlere erişebilir mi? (sadece UPA için)
    pub is_active: bool,                // Kullanıcı aktif mi?
}

// Rol verisi - OPA'da rol kontrolü için
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleData {
    pub id: i32,                      // Rol ID'si
    pub code: String,                  // Rol kodu (super_admin, partner_admin vs)
    pub name: String,                  // Rol adı
    pub partner_id: Option<i32>,     // Partner'a özel rol ise partner ID'si
    pub is_system_role: bool,         // Sistem rolü mü? (tüm partnerlar kullanabilir)
}

// İzin verisi - OPA'da permission kontrolü için
#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionData {
    pub id: i32,           // Permission ID'si
    pub resource: String,   // Kaynak (users, invoices, reports vs)
    pub action: String,     // İşlem (create, read, update, delete vs)
    pub scope: String,      // Kapsam (own, partner, all)
}

// User-Role ilişkisi - Hangi user'ın hangi rolü var
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleData {
    pub user_id: i32,  // User ID
    pub role_id: i32,  // Role ID
}

// Role-Permission ilişkisi - Hangi rolün hangi izni var
#[derive(Debug, Serialize, Deserialize)]
pub struct RolePermissionData {
    pub role_id: i32,       // Role ID
    pub permission_id: i32, // Permission ID
}

// API endpoint tanımı - Hangi endpoint hangi izinleri gerektiriyor
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiPermissionData {
    pub id: i32,                           // API permission ID
    pub endpoint: String,                   // Endpoint path (/api/users, /api/invoices vs)
    pub method: String,                     // HTTP method (GET, POST, PUT, DELETE)
    pub module: String,                     // Hangi modüle ait (users, finance, reports vs)
    pub required_permissions: Vec<i32>,    // Bu endpoint için gereken permission ID'leri
}

// Partner API erişimi - Hangi partner hangi API'yi kullanabilir
#[derive(Debug, Serialize, Deserialize)]
pub struct PartnerApiAccessData {
    pub partner_id: i32,         // Partner ID
    pub api_permission_id: i32,  // API permission ID
    pub is_granted: bool,         // Erişim var mı? (true/false)
}

// Implementation bloğu - servisin fonksiyonları
impl OpaDataSync {
    // Yeni bir OpaDataSync servisi oluştur
    pub fn new(db: DatabaseConnection, opa_url: String) -> Self {
        Self {
            db,                           // Veritabanı bağlantısını sakla
            opa_url,                      // OPA URL'ini sakla
            client: reqwest::Client::new(), // Yeni HTTP client oluştur
        }
    }

    // Ana senkronizasyon fonksiyonu - tüm veriyi OPA'ya yükler
    pub async fn sync_all_data(&self) -> Result<(), AppError> {
        println!("🔄 Starting OPA data synchronization...");

        // 1. Veritabanından tüm verileri çek
        let opa_data = self.fetch_all_data().await?;

        // 2. Verileri OPA'ya gönder
        self.push_to_opa(opa_data).await?;

        println!("✅ OPA data synchronization completed");
        Ok(())
    }

    // Veritabanından tüm verileri çeken fonksiyon
    async fn fetch_all_data(&self) -> Result<OpaData, AppError> {
        // PARTNERS - Tüm partner kayıtlarını çek
        println!("  📂 Fetching partners...");
        let partners = partner::Entity::find()  // Partner tablosundan
            .all(&self.db)                      // Tüm kayıtları al
            .await?                              // Asenkron bekle, hata varsa dön
            .into_iter()                         // Iterator'e çevir
            .map(|p| PartnerData {              // Her partner'ı PartnerData'ya dönüştür
                id: p.id,
                code: p.code,
                name: p.name,
                is_main_partner: p.is_main_partner,
                is_active: p.is_active,
                parent_partner_id: p.parent_partner_id,
            })
            .collect();                          // Vec'e topla

        // USERS - Tüm kullanıcı kayıtlarını çek
        println!("  👥 Fetching users...");
        let users = user::Entity::find()        // User tablosundan
            .all(&self.db)                      // Tüm kayıtları al
            .await?
            .into_iter()
            .map(|u| UserData {                 // Her user'ı UserData'ya dönüştür
                id: u.id,
                partner_id: u.partner_id,
                email: u.email,
                // username: u.username,
                user_type: u.user_type.to_string(), // Enum'u string'e çevir
                is_admin: u.is_admin,
                can_access_all_partners: u.can_access_all_partners,
                is_active: u.is_active,
            })
            .collect();

        // ROLES - Tüm rol kayıtlarını çek
        println!("  🎭 Fetching roles...");
        let roles = role::Entity::find()        // Role tablosundan
            .all(&self.db)
            .await?
            .into_iter()
            .map(|r| RoleData {                 // Her rolü RoleData'ya dönüştür
                id: r.id,
                code: r.code,
                name: r.name,
                partner_id: r.partner_id,
                is_system_role: r.is_system_role,
            })
            .collect();

        // PERMISSIONS - Tüm izin kayıtlarını çek
        println!("  🔑 Fetching permissions...");
        let permissions = permission::Entity::find()  // Permission tablosundan
            .all(&self.db)
            .await?
            .into_iter()
            .map(|p| PermissionData {               // Her permission'ı PermissionData'ya dönüştür
                id: p.id,
                resource: p.resource,
                action: p.action,
                scope: p.scope.into(),         // Enum'u string'e çevir
            })
            .collect();

        // USER_ROLES - Hangi user'ın hangi rolü var
        println!("  🔗 Fetching user-role mappings...");
        let user_roles = user_role::Entity::find()  // User_roles tablosundan
            .all(&self.db)
            .await?
            .into_iter()
            .map(|ur| UserRoleData {               // Sadece ID'leri al
                user_id: ur.user_id,
                role_id: ur.role_id,
            })
            .collect();

        // ROLE_PERMISSIONS - Hangi rolün hangi izni var
        println!("  🔗 Fetching role-permission mappings...");
        let role_permissions = role_permission::Entity::find()  // Role_permissions tablosundan
            .all(&self.db)
            .await?
            .into_iter()
            .map(|rp| RolePermissionData {              // Sadece ID'leri al
                role_id: rp.role_id,
                permission_id: rp.permission_id,
            })
            .collect();

        // API_PERMISSIONS - API endpoint tanımları ve gereken izinler
        println!("  🌐 Fetching API permissions...");
        let api_permissions = api_permission::Entity::find()  // Api_permissions tablosundan
            .all(&self.db)
            .await?
            .into_iter()
            .map(|ap| {
                // required_permissions JSON array olarak saklanıyor, Vec<Uuid>'ye çevir
                let perm_ids: Vec<i32> = serde_json::from_value(ap.required_permissions)
                    .unwrap_or_else(|_| Vec::new());  // Parse edilemezse boş Vec dön

                ApiPermissionData {
                    id: ap.id,
                    endpoint: ap.endpoint,
                    method: ap.method.into(),    // Enum'u string'e çevir
                    module: ap.module,
                    required_permissions: perm_ids,
                }
            })
            .collect();

        // PARTNER_API_ACCESS - Hangi partner hangi API'ye erişebilir
        println!("  🔐 Fetching partner API access...");
        let partner_api_access = partner_api_access::Entity::find()  // Partner_api_access tablosundan
            .all(&self.db)
            .await?
            .into_iter()
            .filter(|paa| paa.is_granted)  // Sadece izin verilenleri al
            .map(|paa| PartnerApiAccessData {
                partner_id: paa.partner_id,
                api_permission_id: paa.api_permission_id,
                is_granted: paa.is_granted,
            })
            .collect();

        // Tüm verileri OpaData struct'ında topla ve dön
        Ok(OpaData {
            partners,
            users,
            roles,
            permissions,
            user_roles,
            role_permissions,
            api_permissions,
            partner_api_access,
        })
    }

    // Verileri OPA'ya gönderen fonksiyon
    async fn push_to_opa(&self, data: OpaData) -> Result<(), AppError> {
        // OPA'nın data API endpoint'i
        // PUT /v1/data/system ile "system" namespace'ine veri yüklüyoruz
        let url = format!("{}/v1/data/system", self.opa_url);

        // İstatistikler için sayıları sakla
        let stats = (
            data.partners.len(),
            data.users.len(),
            data.roles.len(),
            data.permissions.len(),
            data.user_roles.len(),
            data.role_permissions.len(),
            data.api_permissions.len(),
            data.partner_api_access.len(),
        );

        // HTTP PUT isteği gönder
        let response = self.client
            .put(&url)                    // PUT metodu kullan
            .json(&serde_json::json!({   // JSON body oluştur
                "partners": data.partners,
                "users": data.users,
                "roles": data.roles,
                "permissions": data.permissions,
                "user_roles": data.user_roles,
                "role_permissions": data.role_permissions,
                "api_permissions": data.api_permissions,
                "partner_api_access": data.partner_api_access,
            }))
            .send()                       // İsteği gönder
            .await?;                      // Sonucu bekle

        // Response kontrolü
        if !response.status().is_success() {  // 200-299 arası değilse
            let error_text = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ExternalService(
                format!("OPA data sync failed: {}", error_text)
            ));
        }

        // Başarılı - istatistikleri yazdır
        println!("  ✅ Synced {} partners", stats.0);
        println!("  ✅ Synced {} users", stats.1);
        println!("  ✅ Synced {} roles", stats.2);
        println!("  ✅ Synced {} permissions", stats.3);
        println!("  ✅ Synced {} user-role mappings", stats.4);
        println!("  ✅ Synced {} role-permission mappings", stats.5);
        println!("  ✅ Synced {} API permissions", stats.6);
        println!("  ✅ Synced {} partner API access records", stats.7);

        Ok(())
    }
}