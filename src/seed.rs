// // src/seed.rs
// use crate::entities::{r#enum, partner, permission, role, role_permission, user};
// use crate::services::auth_service::AuthService;
// use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
// use uuid::Uuid;
//
// use sea_orm::{ColumnTrait, QueryFilter};
// // ✅ DOĞRU: r#enum klasöründen import
// use crate::entities::r#enum::permission_scope::PermissionScope;
// pub async fn seed_initial_data(
//     db: &DatabaseConnection,
//     auth_service: &AuthService,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     println!("Seeding initial data...");
//
//     // Create main partner (United Payment)
//     let main_partner_id = Uuid::new_v4();
//     let main_partner = partner::ActiveModel {
//         id: Set(main_partner_id),
//         name: Set("Moka United Azerbaijan".to_string()),
//         code: Set("MUA".to_string()),
//         partner_type: Set(r#enum::partner_type::PartnerType::Main),
//         parent_partner_id: Set(None),
//         is_main_partner: Set(true),
//         is_active: Set(true),
//         // TODO: settingsin ne ise yaradigini arasdir
//         settings: Set(serde_json::json!({
//             "features": ["all"],
//             "limits": {}
//         })),
//         created_at: Set(chrono::Utc::now().naive_utc()),
//         updated_at: Set(chrono::Utc::now().naive_utc()),
//     };
//     main_partner.insert(db).await?;
//
//     // Create main partner (United Payment)
//     let standard_partner_id = Uuid::new_v4();
//     let standard_partner = partner::ActiveModel {
//         id: Set(standard_partner_id),
//         name: Set("Mocrypt".to_string()),
//         code: Set("MCR".to_string()),
//         partner_type: Set(r#enum::partner_type::PartnerType::Standard),
//         parent_partner_id: Set(Some(main_partner_id)),
//         is_main_partner: Set(false),
//         is_active: Set(true),
//         // TODO: settingsin ne ise yaradigini arasdir
//         settings: Set(serde_json::json!({
//             "features": ["all"],
//             "limits": {}
//         })),
//         created_at: Set(chrono::Utc::now().naive_utc()),
//         updated_at: Set(chrono::Utc::now().naive_utc()),
//     };
//     standard_partner.insert(db).await?;
//
//     // For the main partner, super administrator user portal management
//     let super_admin_id = Uuid::new_v4();
//     let password_hash = auth_service
//         .hash_password("UpAz@?!%12345F79512=84@!")
//         .await?;
//     let super_admin = user::ActiveModel {
//         id: Set(super_admin_id),
//         partner_id: Set(main_partner_id),
//         email: Set("admin@unitedpayment.com".to_string()),
//         //TODO: buna ehtiyac yoxdu
//         username: Set("up_az_super_admin".to_string()),
//         password_hash: Set(password_hash),
//         user_type: Set(r#enum::user_type::UserType::Admin),
//         is_system_user: Set(false),
//         is_admin: Set(true),
//         can_access_all_partners: Set(true),
//         created_by: Set(None),
//         is_active: Set(true),
//         last_login: Set(None),
//         created_at: Set(chrono::Utc::now().naive_utc()),
//         updated_at: Set(chrono::Utc::now().naive_utc()),
//     };
//     super_admin.insert(db).await?;
//
//     // For the partner admin , partner administrator user portal management
//     let partner_admin_id = Uuid::new_v4();
//     let partner_admin_password_hash = auth_service
//         .hash_password("UpAz@?!%12345F79512=85@!")
//         .await?;
//     let partner_admin = user::ActiveModel {
//         id: Set(partner_admin_id),
//         partner_id: Set(standard_partner_id),
//         email: Set("mocrypt_admin@unitedpayment.com".to_string()),
//         //TODO: buna ehtiyac yoxdu
//         username: Set("mocrypt_admin".to_string()),
//         password_hash: Set(partner_admin_password_hash),
//         user_type: Set(r#enum::user_type::UserType::Portal),
//         is_system_user: Set(false),
//         is_admin: Set(true),
//         can_access_all_partners: Set(false),
//         created_by: Set(Some(super_admin_id)),
//         is_active: Set(true),
//         last_login: Set(None),
//         created_at: Set(chrono::Utc::now().naive_utc()),
//         updated_at: Set(chrono::Utc::now().naive_utc()),
//     };
//     partner_admin.insert(db).await?;
//     // Create system roles
//     let roles = vec![
//         ("super_admin", "Super Admin", "Full system access", false),
//         (
//             "partner_portal_admin",
//             "Partner Portal Admin",
//             "Full partner portal access",
//             false,
//         ),
//         (
//             "partner_admin",
//             "Partner  Admin",
//             "Full partner access",
//             true,
//         ),
//         // (
//         //     "finance_operator",
//         //     "Finance Operator",
//         //     "Manage financial operations",
//         //     true,
//         // ),
//         // (
//         //     "finance_approver",
//         //     "Finance Approver",
//         //     "Approve financial transactions",
//         //     true,
//         // ),
//         // (
//         //     "operations_viewer",
//         //     "Operations Viewer",
//         //     "View operations",
//         //     true,
//         // ),
//         (
//             "operations_manager",
//             "Operations Manager",
//             "Manage operations",
//             true,
//         ),
//         // (
//         //     "cross_partner_operator",
//         //     "Cross Partner Operator",
//         //     "Operate across partners",
//         //     true,
//         // ),
//     ];
//
//     for (code, name, desc, is_system) in roles {
//         let role_id = Uuid::new_v4();
//         let role = role::ActiveModel {
//             id: Set(role_id),
//             partner_id: Set(Some(main_partner_id)),
//             name: Set(name.to_string()),
//             code: Set(code.to_string()),
//             description: Set(Some(desc.to_string())),
//             is_system_role: Set(is_system),
//             created_by: Set(Some(super_admin_id)),
//             created_at: Set(chrono::Utc::now().naive_utc()),
//             updated_at: Set(chrono::Utc::now().naive_utc()),
//         };
//         role.insert(db).await?;
//     }
//
//     // Create base permissions
//     let resources = vec![
//         "partners",
//         "users",
//         "roles",
//         "permissions",
//         "invoices",
//         "transactions",
//         "operations",
//         "reports",
//     ];
//     let actions = vec!["create", "read", "update", "delete", "approve", "export"];
//     // let scopes = vec![
//     //     r#enum::permission_scope::PermissionScope::Own,
//     //     r#enum::permission_scope::PermissionScope::Partner,
//     //     r#enum::permission_scope::PermissionScope::All,
//     // ];
//
//     for resource in &resources {
//         for action in &actions {
//             // Önce var mı diye kontrol et
//             let existing = permission::Entity::find()
//                 .filter(permission::Column::Resource.eq(resource.to_string()))
//                 .filter(permission::Column::Action.eq(action.to_string()))
//                 .one(db)
//                 .await?;
//
//             if existing.is_none() {
//                 // Tek bir default scope ile ekle
//                 let default_scope = if *resource == "partners" {
//                     PermissionScope::Partner
//                 } else {
//                     PermissionScope::Own
//                 };
//
//                 // ✅ permission_id tanımla
//                 let permission_id = Uuid::new_v4();
//
//                 // ✅ Tüm required field'ları ekle
//                 let permission = permission::ActiveModel {
//                     id: Set(permission_id),
//                     resource: Set(resource.to_string()),
//                     action: Set(action.to_string()),
//                     scope: Set(default_scope),
//                     conditions: Set(Some(serde_json::json!({
//                         "available_scopes": ["own", "partner", "all"]
//                     }))),
//                     description: Set(Some(format!("Permission to {} {}", action, resource))),
//                     created_at: Set(chrono::Utc::now().naive_utc()),
//                 };
//
//                 permission.insert(db).await?;
//                 println!("Created permission: {} {}", action, resource);
//             } else {
//                 println!("Permission already exists: {} {}", action, resource);
//             }
//         }
//     }
//
//     // for resource in &resources {
//     //     for action in &actions {
//     //         for scope in &scopes {
//     //             // Skip invalid combinations
//     //             if *resource == "partners" && scope == &r#enum::permission_scope::PermissionScope::Own {
//     //                 continue;
//     //             }
//     //
//     //             let permission_id = Uuid::new_v4();
//     //             let permission = permission::ActiveModel {
//     //                 id: Set(permission_id),
//     //                 resource: Set(resource.to_string()),
//     //                 action: Set(action.to_string()),
//     //                 scope: Set(scope.clone()),
//     //                 conditions: Set(None),
//     //                 description: Set(Some(format!("{} {} in {} scope", action, resource,
//     //                                               match scope {
//     //                                                   r#enum::permission_scope::PermissionScope::Own => "own",
//     //                                                   r#enum::permission_scope::PermissionScope::Partner => "partner",
//     //                                                   r#enum::permission_scope::PermissionScope::All => "all",
//     //                                               }
//     //                 ))),
//     //                 created_at: Set(chrono::Utc::now().naive_utc()),
//     //             };
//     //             permission.insert(db).await?;
//     //         }
//     //     }
//     // }
//     println!("Initial data seeded successfully!");
//     Ok(())
// }





// 
// 
// // src/seed.rs
// // ✅ DOĞRU: r#enum klasöründen import
// use crate::entities::r#enum::permission_scope::PermissionScope;
// use crate::entities::r#enum::partner_type::PartnerType;
// use crate::entities::r#enum::user_type::UserType;
// use crate::entities::r#enum::http_method::HttpMethod;
// use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set, TransactionTrait};
// use uuid::Uuid;
// use crate::entities::{partner, user, role, permission, role_permission, user_role, api_permission, partner_api_access};
// use crate::services::auth_service::AuthService;
// use std::collections::HashMap;
// use chrono::Utc;
// 
// pub struct SeedData;
// 
// impl SeedData {
//     pub async fn seed_initial_data(
//         db: &DatabaseConnection,
//         auth_service: &AuthService,
//     ) -> Result<(), Box<dyn std::error::Error>> {
// 
//         println!("🚀 Starting detailed seed process for United Payment Azerbaijan...");
// 
//         let txn = db.begin().await?;
// 
//         // Check if already seeded
//         let existing = partner::Entity::find()
//             .filter(partner::Column::Code.eq("UPA"))
//             .one(&txn)
//             .await?;
// 
//         if existing.is_some() {
//             println!("⚠️ Database already seeded. Skipping...");
//             return Ok(());
//         }
// 
//         // ==================== STEP 1: CREATE PARTNERS ====================
//         println!("\n📂 STEP 1: Creating Partners...");
//         let partner_ids = Self::create_partners(&txn).await?;
// 
//         // ==================== STEP 2: CREATE ROLES ====================
//         println!("\n🎭 STEP 2: Creating Roles...");
//         let role_ids = Self::create_roles(&txn).await?;
// 
//         // ==================== STEP 3: CREATE PERMISSIONS ====================
//         println!("\n🔑 STEP 3: Creating Permissions...");
//         let permission_ids = Self::create_permissions(&txn).await?;
// 
//         // ==================== STEP 4: ASSIGN PERMISSIONS TO ROLES ====================
//         println!("\n🔗 STEP 4: Assigning Permissions to Roles...");
//         Self::assign_permissions_to_roles(&txn, &role_ids, &permission_ids).await?;
// 
//         // ==================== STEP 5: CREATE USERS ====================
//         println!("\n👥 STEP 5: Creating Users...");
//         let user_ids = Self::create_users(&txn, &partner_ids, auth_service).await?;
// 
//         // ==================== STEP 6: ASSIGN ROLES TO USERS ====================
//         println!("\n👤 STEP 6: Assigning Roles to Users...");
//         Self::assign_roles_to_users(&txn, &user_ids, &role_ids).await?;
// 
//         // ==================== STEP 7: CREATE API PERMISSIONS ====================
//         println!("\n🌐 STEP 7: Creating API Permissions...");
//         let api_permission_ids = Self::create_api_permissions(&txn, &permission_ids).await?;
// 
//         // ==================== STEP 8: GRANT API ACCESS TO PARTNERS ====================
//         println!("\n🔐 STEP 8: Granting API Access to Partners...");
//         Self::grant_api_access_to_partners(&txn, &partner_ids, &api_permission_ids).await?;
// 
//         txn.commit().await?;
// 
//         println!("\n✅ Seed completed successfully!");
//         // Self::print_credentials();
// 
//         Ok(())
//     }
// 
//     // ==================== PARTNERS ====================
//     async fn create_partners(txn: &sea_orm::DatabaseTransaction) -> Result<HashMap<String, Uuid>, Box<dyn std::error::Error>> {
//         let mut partner_ids = HashMap::new();
// 
//         // 1. Ana Şirket: United Payment Azerbaijan
//         let upa_id = Uuid::new_v4();
//         let upa = partner::ActiveModel {
//             id: Set(upa_id),
//             name: Set("United Payment Azerbaijan".to_string()),
//             code: Set("UPA".to_string()),
//             partner_type: Set(partner::PartnerType::Main),
//             parent_partner_id: Set(None),
//             is_main_partner: Set(true),
//             is_active: Set(true),
//             settings: Set(serde_json::json!({
//                 "features": ["all"],
//                 "modules": ["partners", "users", "finance", "operations", "reports"],
//                 "country": "AZ",
//                 "currency": "AZN"
//             })),
//             created_at: Set(Utc::now().naive_utc()),
//             updated_at: Set(Utc::now().naive_utc()),
//         };
//         upa.insert(txn).await?;
//         partner_ids.insert("UPA".to_string(), upa_id);
//         println!("  ✓ Created: United Payment Azerbaijan (Ana Şirket)");
// 
//         // 2. Müşteri: Mocrypt
//         let mocrypt_id = Uuid::new_v4();
//         let mocrypt = partner::ActiveModel {
//             id: Set(mocrypt_id),
//             name: Set("Mocrypt".to_string()),
//             code: Set("MOCRYPT".to_string()),
//             partner_type: Set(partner::PartnerType::Standard),
//             parent_partner_id: Set(None),
//             is_main_partner: Set(false),
//             is_active: Set(true),
//             settings: Set(serde_json::json!({
//                 "features": ["crypto_payments", "wallet_management"],
//                 "modules": ["invoices", "transactions", "reports"],
//                 "industry": "fintech",
//                 "type": "crypto"
//             })),
//             created_at: Set(Utc::now().naive_utc()),
//             updated_at: Set(Utc::now().naive_utc()),
//         };
//         mocrypt.insert(txn).await?;
//         partner_ids.insert("MOCRYPT".to_string(), mocrypt_id);
//         println!("  ✓ Created: Mocrypt (Müşteri - Kripto Ödemeler)");
// 
//         // 3. Müşteri: Pasha Holding
//         let pasha_holding_id = Uuid::new_v4();
//         let pasha_holding = partner::ActiveModel {
//             id: Set(pasha_holding_id),
//             name: Set("Pasha Holding".to_string()),
//             code: Set("PASHA_HOLD".to_string()),
//             partner_type: Set(partner::PartnerType::Standard),
//             parent_partner_id: Set(None),
//             is_main_partner: Set(false),
//             is_active: Set(true),
//             settings: Set(serde_json::json!({
//                 "features": ["multi_company", "consolidated_reporting"],
//                 "modules": ["partners", "users", "invoices", "transactions", "reports"],
//                 "industry": "holding",
//                 "subsidiaries_count": 2
//             })),
//             created_at: Set(Utc::now().naive_utc()),
//             updated_at: Set(Utc::now().naive_utc()),
//         };
//         pasha_holding.insert(txn).await?;
//         partner_ids.insert("PASHA_HOLD".to_string(), pasha_holding_id);
//         println!("  ✓ Created: Pasha Holding (Müşteri - Holding)");
// 
//         // 4. Alt Şirket: PashaBank
//         let pashabank_id = Uuid::new_v4();
//         let pashabank = partner::ActiveModel {
//             id: Set(pashabank_id),
//             name: Set("PashaBank".to_string()),
//             code: Set("PASHABANK".to_string()),
//             partner_type: Set(partner::PartnerType::Subsidiary),
//             parent_partner_id: Set(Some(pasha_holding_id)),
//             is_main_partner: Set(false),
//             is_active: Set(true),
//             settings: Set(serde_json::json!({
//                 "features": ["banking", "loans", "cards"],
//                 "modules": ["invoices", "transactions", "reports"],
//                 "industry": "banking",
//                 "parent": "PASHA_HOLD"
//             })),
//             created_at: Set(Utc::now().naive_utc()),
//             updated_at: Set(Utc::now().naive_utc()),
//         };
//         pashabank.insert(txn).await?;
//         partner_ids.insert("PASHABANK".to_string(), pashabank_id);
//         println!("  ✓ Created: PashaBank (Pasha Holding Alt Şirketi)");
// 
//         // 5. Alt Şirket: Pasha Sigorta
//         let pasha_sigorta_id = Uuid::new_v4();
//         let pasha_sigorta = partner::ActiveModel {
//             id: Set(pasha_sigorta_id),
//             name: Set("Pasha Sigorta".to_string()),
//             code: Set("PASHA_SIG".to_string()),
//             partner_type: Set(partner::PartnerType::Subsidiary),
//             parent_partner_id: Set(Some(pasha_holding_id)),
//             is_main_partner: Set(false),
//             is_active: Set(true),
//             settings: Set(serde_json::json!({
//                 "features": ["insurance", "claims"],
//                 "modules": ["invoices", "transactions", "reports"],
//                 "industry": "insurance",
//                 "parent": "PASHA_HOLD"
//             })),
//             created_at: Set(Utc::now().naive_utc()),
//             updated_at: Set(Utc::now().naive_utc()),
//         };
//         pasha_sigorta.insert(txn).await?;
//         partner_ids.insert("PASHA_SIG".to_string(), pasha_sigorta_id);
//         println!("  ✓ Created: Pasha Sigorta (Pasha Holding Alt Şirketi)");
// 
//         Ok(partner_ids)
//     }
// 
//     // ==================== USERS ====================
//     async fn create_users(
//         txn: &sea_orm::DatabaseTransaction,
//         partner_ids: &HashMap<String, Uuid>,
//         auth_service: &AuthService
//     ) -> Result<HashMap<String, Uuid>, Box<dyn std::error::Error>> {
//         let mut user_ids = HashMap::new();
// 
//         println!("\n  Creating users for each partner...");
// 
//         // ========== United Payment Azerbaijan Users ==========
//         let upa_admin_id = Uuid::new_v4();
//         let upa_admin = user::ActiveModel {
//             id: Set(upa_admin_id),
//             partner_id: Set(partner_ids["UPA"]),
//             email: Set("admin@unitedpayment.az".to_string()),
//             username: Set("upa_admin".to_string()),
//             password_hash: Set(auth_service.hash_password("UpaAdmin@2024").await?),
//             user_type: Set(UserType::Admin),
//             is_system_user: Set(false),
//             is_admin: Set(true),
//             can_access_all_partners: Set(true),
//             created_by: Set(None),
//             is_active: Set(true),
//             last_login: Set(None),
//             created_at: Set(Utc::now().naive_utc()),
//             updated_at: Set(Utc::now().naive_utc()),
//         };
//         upa_admin.insert(txn).await?;
//         user_ids.insert("upa_admin".to_string(), upa_admin_id);
//         println!("    ✓ UPA Admin: admin@unitedpayment.az (Süper Admin)");
// 
//         let upa_api_id = Uuid::new_v4();
//         let upa_api = user::ActiveModel {
//             id: Set(upa_api_id),
//             partner_id: Set(partner_ids["UPA"]),
//             email: Set("api@unitedpayment.az".to_string()),
//             username: Set("upa_api_user".to_string()),
//             password_hash: Set(auth_service.hash_password("UpaApi@2024").await?),
//             user_type: Set(UserType::System),
//             is_system_user: Set(true),
//             is_admin: Set(false),
//             can_access_all_partners: Set(true),
//             created_by: Set(Some(upa_admin_id)),
//             is_active: Set(true),
//             last_login: Set(None),
//             created_at: Set(Utc::now().naive_utc()),
//             updated_at: Set(Utc::now().naive_utc()),
//         };
//         upa_api.insert(txn).await?;
//         user_ids.insert("upa_api".to_string(), upa_api_id);
//         println!("    ✓ UPA API: api@unitedpayment.az (Sistem Entegrasyonu)");
// 
//         let upa_portal_id = Uuid::new_v4();
//         let upa_portal = user::ActiveModel {
//             id: Set(upa_portal_id),
//             partner_id: Set(partner_ids["UPA"]),
//             email: Set("operations@unitedpayment.az".to_string()),
//             username: Set("upa_operator".to_string()),
//             password_hash: Set(auth_service.hash_password("UpaOps@2024").await?),
//             user_type: Set(UserType::Portal),
//             is_system_user: Set(false),
//             is_admin: Set(false),
//             can_access_all_partners: Set(false),
//             created_by: Set(Some(upa_admin_id)),
//             is_active: Set(true),
//             last_login: Set(None),
//             created_at: Set(Utc::now().naive_utc()),
//             updated_at: Set(Utc::now().naive_utc()),
//         };
//         upa_portal.insert(txn).await?;
//         user_ids.insert("upa_portal".to_string(), upa_portal_id);
//         println!("    ✓ UPA Portal: operations@unitedpayment.az (Operasyon)");
// 
//         // ========== Mocrypt Users ==========
//         let mocrypt_admin_id = Uuid::new_v4();
//         let mocrypt_admin = user::ActiveModel {
//             id: Set(mocrypt_admin_id),
//             partner_id: Set(partner_ids["MOCRYPT"]),
//             email: Set("admin@mocrypt.com".to_string()),
//             username: Set("mocrypt_admin".to_string()),
//             password_hash: Set(auth_service.hash_password("Mocrypt@2024").await?),
//             user_type: Set(UserType::Admin),
//             is_system_user: Set(false),
//             is_admin: Set(true),
//             can_access_all_partners: Set(false),
//             created_by: Set(Some(upa_admin_id)),
//             is_active: Set(true),
//             last_login: Set(None),
//             created_at: Set(Utc::now().naive_utc()),
//             updated_at: Set(Utc::now().naive_utc()),
//         };
//         mocrypt_admin.insert(txn).await?;
//         user_ids.insert("mocrypt_admin".to_string(), mocrypt_admin_id);
//         println!("    ✓ Mocrypt Admin: admin@mocrypt.com (Partner Admin)");
// 
//         let mocrypt_api_id = Uuid::new_v4();
//         let mocrypt_api = user::ActiveModel {
//             id: Set(mocrypt_api_id),
//             partner_id: Set(partner_ids["MOCRYPT"]),
//             email: Set("api@mocrypt.com".to_string()),
//             username: Set("mocrypt_api".to_string()),
//             password_hash: Set(auth_service.hash_password("MocApi@2024").await?),
//             user_type: Set(UserType::System),
//             is_system_user: Set(true),
//             is_admin: Set(false),
//             can_access_all_partners: Set(false),
//             created_by: Set(Some(mocrypt_admin_id)),
//             is_active: Set(true),
//             last_login: Set(None),
//             created_at: Set(Utc::now().naive_utc()),
//             updated_at: Set(Utc::now().naive_utc()),
//         };
//         mocrypt_api.insert(txn).await?;
//         user_ids.insert("mocrypt_api".to_string(), mocrypt_api_id);
//         println!("    ✓ Mocrypt API: api@mocrypt.com (Kripto API)");
// 
//         let mocrypt_portal_id = Uuid::new_v4();
//         let mocrypt_portal = user::ActiveModel {
//             id: Set(mocrypt_portal_id),
//             partner_id: Set(partner_ids["MOCRYPT"]),
//             email: Set("operations@mocrypt.com".to_string()),
//             username: Set("mocrypt_operator".to_string()),
//             password_hash: Set(auth_service.hash_password("MocOps@2024").await?),
//             user_type: Set(UserType::Portal),
//             is_system_user: Set(false),
//             is_admin: Set(false),
//             can_access_all_partners: Set(false),
//             created_by: Set(Some(mocrypt_admin_id)),
//             is_active: Set(true),
//             last_login: Set(None),
//             created_at: Set(Utc::now().naive_utc()),
//             updated_at: Set(Utc::now().naive_utc()),
//         };
//         mocrypt_portal.insert(txn).await?;
//         user_ids.insert("mocrypt_portal".to_string(), mocrypt_portal_id);
//         println!("    ✓ Mocrypt Portal: operations@mocrypt.com (Kripto Ops)");
// 
//         // ========== Pasha Holding Users ==========
//         let pasha_admin_id = Uuid::new_v4();
//         let pasha_admin = user::ActiveModel {
//             id: Set(pasha_admin_id),
//             partner_id: Set(partner_ids["PASHA_HOLD"]),
//             email: Set("admin@pashaholding.az".to_string()),
//             username: Set("pasha_admin".to_string()),
//             password_hash: Set(auth_service.hash_password("Pasha@2024").await?),
//             user_type: Set(UserType::Admin),
//             is_system_user: Set(false),
//             is_admin: Set(true),
//             can_access_all_partners: Set(false),
//             created_by: Set(Some(upa_admin_id)),
//             is_active: Set(true),
//             last_login: Set(None),
//             created_at: Set(Utc::now().naive_utc()),
//             updated_at: Set(Utc::now().naive_utc()),
//         };
//         pasha_admin.insert(txn).await?;
//         user_ids.insert("pasha_admin".to_string(), pasha_admin_id);
//         println!("    ✓ Pasha Admin: admin@pashaholding.az (Holding Admin)");
// 
//         let pasha_api_id = Uuid::new_v4();
//         let pasha_api = user::ActiveModel {
//             id: Set(pasha_api_id),
//             partner_id: Set(partner_ids["PASHA_HOLD"]),
//             email: Set("api@pashaholding.az".to_string()),
//             username: Set("pasha_api".to_string()),
//             password_hash: Set(auth_service.hash_password("PashaApi@2024").await?),
//             user_type: Set(UserType::System),
//             is_system_user: Set(true),
//             is_admin: Set(false),
//             can_access_all_partners: Set(false),
//             created_by: Set(Some(pasha_admin_id)),
//             is_active: Set(true),
//             last_login: Set(None),
//             created_at: Set(Utc::now().naive_utc()),
//             updated_at: Set(Utc::now().naive_utc()),
//         };
//         pasha_api.insert(txn).await?;
//         user_ids.insert("pasha_api".to_string(), pasha_api_id);
//         println!("    ✓ Pasha API: api@pashaholding.az (Holding API)");
// 
//         // ========== PashaBank Users ==========
//         let pashabank_portal_id = Uuid::new_v4();
//         let pashabank_portal = user::ActiveModel {
//             id: Set(pashabank_portal_id),
//             partner_id: Set(partner_ids["PASHABANK"]),
//             email: Set("operations@pashabank.az".to_string()),
//             username: Set("pashabank_operator".to_string()),
//             password_hash: Set(auth_service.hash_password("BankOps@2024").await?),
//             user_type: Set(UserType::Portal),
//             is_system_user: Set(false),
//             is_admin: Set(false),
//             can_access_all_partners: Set(false),
//             created_by: Set(Some(pasha_admin_id)),
//             is_active: Set(true),
//             last_login: Set(None),
//             created_at: Set(Utc::now().naive_utc()),
//             updated_at: Set(Utc::now().naive_utc()),
//         };
//         pashabank_portal.insert(txn).await?;
//         user_ids.insert("pashabank_portal".to_string(), pashabank_portal_id);
//         println!("    ✓ PashaBank Portal: operations@pashabank.az (Banka Ops)");
// 
//         let pashabank_api_id = Uuid::new_v4();
//         let pashabank_api = user::ActiveModel {
//             id: Set(pashabank_api_id),
//             partner_id: Set(partner_ids["PASHABANK"]),
//             email: Set("api@pashabank.az".to_string()),
//             username: Set("pashabank_api".to_string()),
//             password_hash: Set(auth_service.hash_password("BankApi@2024").await?),
//             user_type: Set(UserType::System),
//             is_system_user: Set(true),
//             is_admin: Set(false),
//             can_access_all_partners: Set(false),
//             created_by: Set(Some(pasha_admin_id)),
//             is_active: Set(true),
//             last_login: Set(None),
//             created_at: Set(Utc::now().naive_utc()),
//             updated_at: Set(Utc::now().naive_utc()),
//         };
//         pashabank_api.insert(txn).await?;
//         user_ids.insert("pashabank_api".to_string(), pashabank_api_id);
//         println!("    ✓ PashaBank API: api@pashabank.az (Banka API)");
// 
//         // ========== Pasha Sigorta Users ==========
//         let sigorta_portal_id = Uuid::new_v4();
//         let sigorta_portal = user::ActiveModel {
//             id: Set(sigorta_portal_id),
//             partner_id: Set(partner_ids["PASHA_SIG"]),
//             email: Set("operations@pashasigorta.az".to_string()),
//             username: Set("sigorta_operator".to_string()),
//             password_hash: Set(auth_service.hash_password("SigOps@2024").await?),
//             user_type: Set(UserType::Portal),
//             is_system_user: Set(false),
//             is_admin: Set(false),
//             can_access_all_partners: Set(false),
//             created_by: Set(Some(pasha_admin_id)),
//             is_active: Set(true),
//             last_login: Set(None),
//             created_at: Set(Utc::now().naive_utc()),
//             updated_at: Set(Utc::now().naive_utc()),
//         };
//         sigorta_portal.insert(txn).await?;
//         user_ids.insert("sigorta_portal".to_string(), sigorta_portal_id);
//         println!("    ✓ Pasha Sigorta Portal: operations@pashasigorta.az (Sigorta Ops)");
// 
//         let sigorta_api_id = Uuid::new_v4();
//         let sigorta_api = user::ActiveModel {
//             id: Set(sigorta_api_id),
//             partner_id: Set(partner_ids["PASHA_SIG"]),
//             email: Set("api@pashasigorta.az".to_string()),
//             username: Set("sigorta_api".to_string()),
//             password_hash: Set(auth_service.hash_password("SigApi@2024").await?),
//             user_type: Set(UserType::System),
//             is_system_user: Set(true),
//             is_admin: Set(false),
//             can_access_all_partners: Set(false),
//             created_by: Set(Some(pasha_admin_id)),
//             is_active: Set(true),
//             last_login: Set(None),
//             created_at: Set(Utc::now().naive_utc()),
//             updated_at: Set(Utc::now().naive_utc()),
//         };
//         sigorta_api.insert(txn).await?;
//         user_ids.insert("sigorta_api".to_string(), sigorta_api_id);
//         println!("    ✓ Pasha Sigorta API: api@pashasigorta.az (Sigorta API)");
// 
//         Ok(user_ids)
//     }
// 
//     // ==================== ROLES ====================
//     async fn create_roles(txn: &sea_orm::DatabaseTransaction) -> Result<HashMap<String, Uuid>, Box<dyn std::error::Error>> {
//         let mut role_ids = HashMap::new();
// 
//         let roles = vec![
//             ("super_admin", "Super Admin", "Full system access - only for UPA", true),
//             ("partner_admin", "Partner Admin", "Full partner management access", true),
//             ("api_user", "API User", "System integration user", true),
//             ("finance_manager", "Finance Manager", "Manage financial operations", true),
//             ("finance_viewer", "Finance Viewer", "View financial data only", true),
//             ("operations_manager", "Operations Manager", "Manage daily operations", true),
//             ("operations_viewer", "Operations Viewer", "View operations data", true),
//             ("report_manager", "Report Manager", "Create and export reports", true),
//             ("report_viewer", "Report Viewer", "View reports only", true),
//             ("user_manager", "User Manager", "Manage users and roles", true),
//             ("cross_partner_operator", "Cross Partner Operator", "Operate across child partners", true),
//         ];
// 
//         for (code, name, desc, is_system) in roles {
//             let role_id = Uuid::new_v4();
//             let role = role::ActiveModel {
//                 id: Set(role_id),
//                 partner_id: Set(None),
//                 name: Set(name.to_string()),
//                 code: Set(code.to_string()),
//                 description: Set(Some(desc.to_string())),
//                 is_system_role: Set(is_system),
//                 created_by: Set(None),
//                 created_at: Set(Utc::now().naive_utc()),
//                 updated_at: Set(Utc::now().naive_utc()),
//             };
//             role.insert(txn).await?;
//             role_ids.insert(code.to_string(), role_id);
//             println!("  ✓ Created role: {} - {}", code, desc);
//         }
// 
//         Ok(role_ids)
//     }
// 
//     // ==================== PERMISSIONS ====================
//     async fn create_permissions(txn: &sea_orm::DatabaseTransaction) -> Result<HashMap<String, Uuid>, Box<dyn std::error::Error>> {
//         let mut permission_ids = HashMap::new();
// 
//         let permissions = vec![
//             // PARTNER İZİNLERİ
//             ("partners:create", "partners", "create", "all", "Yeni partner oluşturabilir - sadece UPA"),
//             ("partners:read", "partners", "read", "partner", "Partner bilgilerini görüntüleyebilir"),
//             ("partners:update", "partners", "update", "partner", "Partner bilgilerini güncelleyebilir"),
//             ("partners:delete", "partners", "delete", "all", "Partner silebilir - sadece UPA"),
// 
//             // USER İZİNLERİ
//             ("users:create", "users", "create", "partner", "Kendi partnerına kullanıcı ekleyebilir"),
//             ("users:read", "users", "read", "partner", "Kullanıcıları görüntüleyebilir"),
//             ("users:update", "users", "update", "own", "Kullanıcı bilgilerini güncelleyebilir"),
//             ("users:delete", "users", "delete", "partner", "Kullanıcı silebilir"),
// 
//             // ROL İZİNLERİ
//             ("roles:read", "roles", "read", "partner", "Rolleri görüntüleyebilir"),
//             ("roles:assign", "roles", "assign", "partner", "Kullanıcılara rol atayabilir"),
//             ("roles:create", "roles", "create", "partner", "Yeni rol oluşturabilir"),
//             ("roles:update", "roles", "update", "partner", "Rol güncelleyebilir"),
//             ("roles:delete", "roles", "delete", "partner", "Rol silebilir"),
// 
//             // FATURA İZİNLERİ
//             ("invoices:create", "invoices", "create", "partner", "Fatura oluşturabilir"),
//             ("invoices:read", "invoices", "read", "partner", "Faturaları görüntüleyebilir"),
//             ("invoices:update", "invoices", "update", "partner", "Fatura düzenleyebilir"),
//             ("invoices:delete", "invoices", "delete", "partner", "Fatura silebilir"),
//             ("invoices:approve", "invoices", "approve", "partner", "Fatura onaylayabilir"),
//             ("invoices:export", "invoices", "export", "partner", "Faturaları dışa aktarabilir"),
// 
//             // İŞLEM İZİNLERİ
//             ("transactions:create", "transactions", "create", "partner", "İşlem oluşturabilir"),
//             ("transactions:read", "transactions", "read", "partner", "İşlemleri görüntüleyebilir"),
//             ("transactions:update", "transactions", "update", "partner", "İşlem güncelleyebilir"),
//             ("transactions:approve", "transactions", "approve", "partner", "İşlem onaylayabilir"),
//             ("transactions:export", "transactions", "export", "partner", "İşlemleri dışa aktarabilir"),
// 
//             // OPERASYON İZİNLERİ
//             ("operations:create", "operations", "create", "partner", "Operasyon oluşturabilir"),
//             ("operations:read", "operations", "read", "partner", "Operasyonları görüntüleyebilir"),
//             ("operations:update", "operations", "update", "partner", "Operasyon güncelleyebilir"),
//             ("operations:delete", "operations", "delete", "partner", "Operasyon silebilir"),
//             ("operations:approve", "operations", "approve", "partner", "Operasyon onaylayabilir"),
// 
//             // RAPOR İZİNLERİ
//             ("reports:read", "reports", "read", "partner", "Raporları görüntüleyebilir"),
//             ("reports:create", "reports", "create", "partner", "Rapor oluşturabilir"),
//             ("reports:export", "reports", "export", "partner", "Rapor dışa aktarabilir"),
//         ];
// 
//         for (key, resource, action, scope_str, description) in permissions {
//             let scope = match scope_str {
//                 "own" => PermissionScope::Own,
//                 "partner" => PermissionScope::Partner,
//                 "all" => PermissionScope::All,
//                 _ => PermissionScope::Partner,
//             };
// 
//             let permission_id = Uuid::new_v4();
//             let permission = permission::ActiveModel {
//                 id: Set(permission_id),
//                 resource: Set(resource.to_string()),
//                 action: Set(action.to_string()),
//                 scope: Set(scope),
//                 conditions: Set(None),
//                 description: Set(Some(description.to_string())),
//                 created_at: Set(Utc::now().naive_utc()),
//             };
//             permission.insert(txn).await?;
//             permission_ids.insert(key.to_string(), permission_id);
//             println!("  ✓ Created permission: {} - {}", key, description);
//         }
// 
//         Ok(permission_ids)
//     }
// 
//     // ==================== ASSIGN PERMISSIONS TO ROLES ====================
//     async fn assign_permissions_to_roles(
//         txn: &sea_orm::DatabaseTransaction,
//         role_ids: &HashMap<String, Uuid>,
//         permission_ids: &HashMap<String, Uuid>
//     ) -> Result<(), Box<dyn std::error::Error>> {
// 
//         let role_permissions = vec![
//             // SUPER ADMIN - HER ŞEYE ERİŞİM
//             ("super_admin", vec![
//                 "partners:create", "partners:read", "partners:update", "partners:delete",
//                 "users:create", "users:read", "users:update", "users:delete",
//                 "roles:read", "roles:assign", "roles:create", "roles:update", "roles:delete",
//                 "invoices:create", "invoices:read", "invoices:update", "invoices:delete", "invoices:approve", "invoices:export",
//                 "transactions:create", "transactions:read", "transactions:update", "transactions:approve", "transactions:export",
//                 "operations:create", "operations:read", "operations:update", "operations:delete", "operations:approve",
//                 "reports:read", "reports:create", "reports:export",
//             ]),
// 
//             // PARTNER ADMIN - KENDİ PARTNERİNDE HER ŞEY
//             ("partner_admin", vec![
//                 "partners:read", "partners:update",
//                 "users:create", "users:read", "users:update", "users:delete",
//                 "roles:read", "roles:assign",
//                 "invoices:create", "invoices:read", "invoices:update", "invoices:delete", "invoices:approve", "invoices:export",
//                 "transactions:create", "transactions:read", "transactions:update", "transactions:approve", "transactions:export",
//                 "operations:create", "operations:read", "operations:update", "operations:delete", "operations:approve",
//                 "reports:read", "reports:create", "reports:export",
//             ]),
// 
//             // API USER - SİSTEM ENTEGRASYONU
//             ("api_user", vec![
//                 "partners:read",
//                 "users:read",
//                 "invoices:create", "invoices:read", "invoices:update",
//                 "transactions:create", "transactions:read",
//                 "operations:read",
//                 "reports:read",
//             ]),
// 
//             // FINANCE MANAGER - MALİ İŞLEMLER YÖNETİMİ
//             ("finance_manager", vec![
//                 "invoices:create", "invoices:read", "invoices:update", "invoices:approve", "invoices:export",
//                 "transactions:create", "transactions:read", "transactions:update", "transactions:approve", "transactions:export",
//                 "reports:read", "reports:create", "reports:export",
//             ]),
// 
//             // FINANCE VIEWER - MALİ VERİLERİ SADECE GÖRME
//             ("finance_viewer", vec![
//                 "invoices:read",
//                 "transactions:read",
//                 "reports:read",
//             ]),
// 
//             // OPERATIONS MANAGER - OPERASYON YÖNETİMİ
//             ("operations_manager", vec![
//                 "operations:create", "operations:read", "operations:update", "operations:delete", "operations:approve",
//                 "reports:read", "reports:create",
//             ]),
// 
//             // OPERATIONS VIEWER - OPERASYON GÖRÜNTÜLEME
//             ("operations_viewer", vec![
//                 "operations:read",
//                 "reports:read",
//             ]),
// 
//             // USER MANAGER - KULLANICI YÖNETİMİ
//             ("user_manager", vec![
//                 "users:create", "users:read", "users:update", "users:delete",
//                 "roles:read", "roles:assign",
//             ]),
// 
//             // REPORT MANAGER - RAPOR YÖNETİMİ
//             ("report_manager", vec![
//                 "reports:read", "reports:create", "reports:export",
//             ]),
// 
//             // REPORT VIEWER - RAPOR GÖRÜNTÜLEME
//             ("report_viewer", vec![
//                 "reports:read",
//             ]),
// 
//             // CROSS PARTNER OPERATOR - ALT ŞİRKETLERDE İŞLEM
//             ("cross_partner_operator", vec![
//                 "partners:read",
//                 "users:read",
//                 "invoices:read", "invoices:create",
//                 "transactions:read", "transactions:create",
//                 "operations:read",
//                 "reports:read",
//             ]),
//         ];
// 
//         for (role_code, permissions) in role_permissions {
//             if let Some(&role_id) = role_ids.get(role_code) {
//                 for permission_key in permissions {
//                     if let Some(&permission_id) = permission_ids.get(permission_key) {
//                         let assignment = role_permission::ActiveModel {
//                             id: Set(Uuid::new_v4()),
//                             role_id: Set(role_id),
//                             permission_id: Set(permission_id),
//                             granted_at: Set(Utc::now().naive_utc()),
//                         };
//                         assignment.insert(txn).await?;
//                     }
//                 }
//                 println!("  ✓ Assigned permissions to role: {}", role_code);
//             }
//         }
// 
//         Ok(())
//     }
// 
//     // ==================== ASSIGN ROLES TO USERS ====================
//     async fn assign_roles_to_users(
//         txn: &sea_orm::DatabaseTransaction,
//         user_ids: &HashMap<String, Uuid>,
//         role_ids: &HashMap<String, Uuid>,
//     ) -> Result<(), Box<dyn std::error::Error>> {
// 
//         let user_role_assignments = vec![
//             // UPA USERS
//             ("upa_admin", "super_admin", "UPA Admin'e süper admin rolü"),
//             ("upa_api", "api_user", "UPA API user'a sistem entegrasyon rolü"),
//             ("upa_portal", "operations_manager", "UPA Portal user'a operasyon yönetim rolü"),
// 
//             // MOCRYPT USERS
//             ("mocrypt_admin", "partner_admin", "Mocrypt Admin'e partner admin rolü"),
//             ("mocrypt_api", "api_user", "Mocrypt API user'a sistem entegrasyon rolü"),
//             ("mocrypt_portal", "finance_manager", "Mocrypt Portal user'a finans yönetim rolü (kripto işlemler için)"),
// 
//             // PASHA HOLDING USERS
//             ("pasha_admin", "partner_admin", "Pasha Admin'e partner admin rolü"),
//             ("pasha_admin", "cross_partner_operator", "Pasha Admin'e alt şirket erişim rolü"),
//             ("pasha_api", "api_user", "Pasha API user'a sistem entegrasyon rolü"),
// 
//             // PASHABANK USERS
//             ("pashabank_portal", "finance_manager", "PashaBank Portal user'a finans yönetim rolü"),
//             ("pashabank_api", "api_user", "PashaBank API user'a sistem entegrasyon rolü"),
// 
//             // PASHA SIGORTA USERS
//             ("sigorta_portal", "operations_manager", "Pasha Sigorta Portal user'a operasyon yönetim rolü"),
//             ("sigorta_api", "api_user", "Pasha Sigorta API user'a sistem entegrasyon rolü"),
//         ];
// 
//         for (user_key, role_key, description) in user_role_assignments {
//             if let (Some(&user_id), Some(&role_id)) = (user_ids.get(user_key), role_ids.get(role_key)) {
//                 let assignment = user_role::ActiveModel {
//                     id: Set(Uuid::new_v4()),
//                     user_id: Set(user_id),
//                     role_id: Set(role_id),
//                     assigned_by: Set(None),
//                     assigned_at: Set(Utc::now().naive_utc()),
//                 };
//                 assignment.insert(txn).await?;
//                 println!("  ✓ {}: {} → {}", description, user_key, role_key);
//             }
//         }
// 
//         Ok(())
//     }
// 
//     // ==================== API PERMISSIONS ====================
//     async fn create_api_permissions(
//         txn: &sea_orm::DatabaseTransaction,
//         permission_ids: &HashMap<String, Uuid>
//     ) -> Result<HashMap<String, Uuid>, Box<dyn std::error::Error>> {
//         let mut api_permission_ids = HashMap::new();
// 
//         let api_permissions = vec![
//             // PARTNER API'LARI
//             ("partner_list", "/api/partners", "GET", "partners", vec!["partners:read"], "Partner listesini görüntüleme"),
//             ("partner_create", "/api/partners", "POST", "partners", vec!["partners:create"], "Yeni partner oluşturma"),
//             ("partner_get", "/api/partners/*", "GET", "partners", vec!["partners:read"], "Partner detayını görüntüleme"),
//             ("partner_update", "/api/partners/*", "PUT", "partners", vec!["partners:update"], "Partner güncelleme"),
//             ("partner_delete", "/api/partners/*", "DELETE", "partners", vec!["partners:delete"], "Partner silme"),
//             ("partner_users", "/api/partners/*/users", "GET", "partners", vec!["users:read"], "Partner kullanıcılarını listeleme"),
//             ("partner_stats", "/api/partners/*/stats", "GET", "partners", vec!["partners:read"], "Partner istatistiklerini görme"),
// 
//             // USER API'LARI
//             ("user_list", "/api/users", "GET", "users", vec!["users:read"], "Kullanıcı listesini görüntüleme"),
//             ("user_create", "/api/users", "POST", "users", vec!["users:create"], "Yeni kullanıcı oluşturma"),
//             ("user_get", "/api/users/*", "GET", "users", vec!["users:read"], "Kullanıcı detayını görüntüleme"),
//             ("user_update", "/api/users/*", "PUT", "users", vec!["users:update"], "Kullanıcı güncelleme"),
//             ("user_delete", "/api/users/*", "DELETE", "users", vec!["users:delete"], "Kullanıcı silme"),
//             ("user_permissions", "/api/users/*/permissions", "GET", "users", vec!["users:read"], "Kullanıcı yetkilerini görme"),
// 
//             // ROLE API'LARI
//             ("role_list", "/api/roles", "GET", "roles", vec!["roles:read"], "Rol listesini görüntüleme"),
//             ("role_create", "/api/roles", "POST", "roles", vec!["roles:create"], "Yeni rol oluşturma"),
//             ("role_assign", "/api/users/*/roles/*", "POST", "roles", vec!["roles:assign"], "Kullanıcıya rol atama"),
//             ("role_permissions", "/api/roles/*/permissions", "GET", "roles", vec!["roles:read"], "Rol yetkilerini görüntüleme"),
// 
//             // PERMISSION API'LARI
//             ("permission_list", "/api/permissions", "GET", "permissions", vec!["roles:read"], "Yetki listesini görüntüleme"),
// 
//             // AUTH API'LARI (Public - no permission needed)
//             ("auth_login", "/auth/login", "POST", "auth", vec![], "Kullanıcı girişi"),
//             ("auth_refresh", "/auth/refresh", "POST", "auth", vec![], "Token yenileme"),
// 
//             // HEALTH CHECK (Public)
//             ("health", "/health", "GET", "health", vec![], "Sistem sağlık kontrolü"),
//         ];
// 
//         for (key, endpoint, method_str, module, required_perms, description) in api_permissions {
//             let method = match method_str {
//                 "GET" => HttpMethod::Get,
//                 "POST" => HttpMethod::Post,
//                 "PUT" => HttpMethod::Put,
//                 "DELETE" => HttpMethod::Delete,
//                 "PATCH" => HttpMethod::Patch,
//                 _ => HttpMethod::Get,
//             };
// 
//             let mut perm_ids = Vec::new();
//             for perm_key in &required_perms {
//                 if let Some(perm_id) = permission_ids.get(*perm_key) {
//                     perm_ids.push(*perm_id);
//                 }
//             }
// 
//             let api_perm_id = Uuid::new_v4();
//             let api_permission = api_permission::ActiveModel {
//                 id: Set(api_perm_id),
//                 endpoint: Set(endpoint.to_string()),
//                 method: Set(method),
//                 module: Set(module.to_string()),
//                 required_permissions: Set(serde_json::json!(perm_ids)),
//                 description: Set(Some(description.to_string())),
//                 created_at: Set(Utc::now().naive_utc()),
//             };
//             api_permission.insert(txn).await?;
//             api_permission_ids.insert(key.to_string(), api_perm_id);
//             println!("  ✓ Created API permission: {} {} - {}", method_str, endpoint, description);
//         }
// 
//         Ok(api_permission_ids)
//     }
// 
//     // ==================== GRANT API ACCESS TO PARTNERS ====================
//     async fn grant_api_access_to_partners(
//         txn: &sea_orm::DatabaseTransaction,
//         partner_ids: &HashMap<String, Uuid>,
//         api_permission_ids: &HashMap<String, Uuid>
//     ) -> Result<(), Box<dyn std::error::Error>> {
// 
//         let partner_api_access = vec![
//             // UPA - HER ŞEYİ KULLANABİLİR
//             ("UPA", vec![
//                 "partner_list", "partner_create", "partner_get", "partner_update", "partner_delete", "partner_users", "partner_stats",
//                 "user_list", "user_create", "user_get", "user_update", "user_delete", "user_permissions",
//                 "role_list", "role_create", "role_assign", "role_permissions",
//                 "permission_list",
//             ], "United Payment tüm API'lara erişebilir"),
// 
//             // MOCRYPT - STANDART MÜŞTERİ ERİŞİMİ
//             ("MOCRYPT", vec![
//                 "partner_get", "partner_users", "partner_stats",
//                 "user_list", "user_create", "user_get", "user_update", "user_delete", "user_permissions",
//                 "role_list", "role_assign", "role_permissions",
//                 "permission_list",
//             ], "Mocrypt standart müşteri API erişimi"),
// 
//             // PASHA HOLDING - HOLDİNG ERİŞİMİ (ALT ŞİRKETLER DAHİL)
//             ("PASHA_HOLD", vec![
//                 "partner_list", "partner_get", "partner_users", "partner_stats",
//                 "user_list", "user_create", "user_get", "user_update", "user_delete", "user_permissions",
//                 "role_list", "role_assign", "role_permissions",
//                 "permission_list",
//             ], "Pasha Holding ve alt şirket API erişimi"),
// 
//             // PASHABANK - BANKA ERİŞİMİ
//             ("PASHABANK", vec![
//                 "partner_get", "partner_stats",
//                 "user_list", "user_get", "user_permissions",
//                 "role_list", "role_permissions",
//                 "permission_list",
//             ], "PashaBank sınırlı API erişimi"),
// 
//             // PASHA SIGORTA - SİGORTA ERİŞİMİ
//             ("PASHA_SIG", vec![
//                 "partner_get", "partner_stats",
//                 "user_list", "user_get", "user_permissions",
//                 "role_list", "role_permissions",
//                 "permission_list",
//             ], "Pasha Sigorta sınırlı API erişimi"),
//         ];
// 
//         for (partner_key, api_keys, description) in partner_api_access {
//             if let Some(&partner_id) = partner_ids.get(partner_key) {
//                 for api_key in api_keys {
//                     if let Some(&api_perm_id) = api_permission_ids.get(api_key) {
//                         let access = partner_api_access::ActiveModel {
//                             id: Set(Uuid::new_v4()),
//                             partner_id: Set(partner_id),
//                             api_permission_id: Set(api_perm_id),
//                             is_granted: Set(true),
//                             granted_by: Set(None),
//                             granted_at: Set(Utc::now().naive_utc()),
//                         };
//                         access.insert(txn).await?;
//                     }
//                 }
//                 // println!("  ✓ {}: {} → {} API endpoints", description, partner_key, api_keys.len());
//             }
//         }
// 
//         Ok(())
//     }
// 
//     // ==================== PRINT CREDENTIALS ====================
//     // fn print_credentials() {
//     //     println!("\n" + "="*80);
//     //     println!("📋 KULLANICI BİLGİLERİ");
//     //     println!("="*80);
//     //
//     //     println!("\n🏢 UNITED PAYMENT AZERBAIJAN (Ana Şirket):");
//     //     println!("  Admin    : admin@unitedpayment.az / UpaAdmin@2024");
//     //     println!("  API User : api@unitedpayment.az / UpaApi@2024");
//     //     println!("  Portal   : operations@unitedpayment.az / UpaOps@2024");
//     //
//     //     println!("\n💎 MOCRYPT (Müşteri):");
//     //     println!("  Admin    : admin@mocrypt.com / Mocrypt@2024");
//     //     println!("  API User : api@mocrypt.com / MocApi@2024");
//     //     println!("  Portal   : operations@mocrypt.com / MocOps@2024");
//     //
//     //     println!("\n🏛️ PASHA HOLDING (Müşteri):");
//     //     println!("  Admin    : admin@pashaholding.az / Pasha@2024");
//     //     println!("  API User : api@pashaholding.az / PashaApi@2024");
//     //
//     //     println!("\n🏦 PASHABANK (Alt Şirket):");
//     //     println!("  Portal   : operations@pashabank.az / BankOps@2024");
//     //     println!("  API User : api@pashabank.az / BankApi@2024");
//     //
//     //     println!("\n🛡️ PASHA SIGORTA (Alt Şirket):");
//     //     println!("  Portal   : operations@pashasigorta.az / SigOps@2024");
//     //     println!("  API User : api@pashasigorta.az / SigApi@2024");
//     //
//     //     println!("\n" + "="*80);
//     // }
// }