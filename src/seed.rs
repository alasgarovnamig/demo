// src/seed.rs
use crate::entities::{r#enum, partner, permission, role, role_permission, user};
use crate::services::auth_service::AuthService;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use sea_orm::{ColumnTrait, QueryFilter};
// ✅ DOĞRU: r#enum klasöründen import
use crate::entities::r#enum::permission_scope::PermissionScope;
pub async fn seed_initial_data(
    db: &DatabaseConnection,
    auth_service: &AuthService,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Seeding initial data...");

    // Create main partner (United Payment)
    let main_partner_id = Uuid::new_v4();
    let main_partner = partner::ActiveModel {
        id: Set(main_partner_id),
        name: Set("Moka United Azerbaijan".to_string()),
        code: Set("MUA".to_string()),
        partner_type: Set(r#enum::partner_type::PartnerType::Main),
        parent_partner_id: Set(None),
        is_main_partner: Set(true),
        is_active: Set(true),
        // TODO: settingsin ne ise yaradigini arasdir
        settings: Set(serde_json::json!({
            "features": ["all"],
            "limits": {}
        })),
        created_at: Set(chrono::Utc::now().naive_utc()),
        updated_at: Set(chrono::Utc::now().naive_utc()),
    };
    main_partner.insert(db).await?;

    // Create main partner (United Payment)
    let standard_partner_id = Uuid::new_v4();
    let standard_partner = partner::ActiveModel {
        id: Set(standard_partner_id),
        name: Set("Mocrypt".to_string()),
        code: Set("MCR".to_string()),
        partner_type: Set(r#enum::partner_type::PartnerType::Standard),
        parent_partner_id: Set(Some(main_partner_id)),
        is_main_partner: Set(false),
        is_active: Set(true),
        // TODO: settingsin ne ise yaradigini arasdir
        settings: Set(serde_json::json!({
            "features": ["all"],
            "limits": {}
        })),
        created_at: Set(chrono::Utc::now().naive_utc()),
        updated_at: Set(chrono::Utc::now().naive_utc()),
    };
    standard_partner.insert(db).await?;

    // For the main partner, super administrator user portal management
    let super_admin_id = Uuid::new_v4();
    let password_hash = auth_service
        .hash_password("UpAz@?!%12345F79512=84@!")
        .await?;
    let super_admin = user::ActiveModel {
        id: Set(super_admin_id),
        partner_id: Set(main_partner_id),
        email: Set("admin@unitedpayment.com".to_string()),
        //TODO: buna ehtiyac yoxdu
        username: Set("up_az_super_admin".to_string()),
        password_hash: Set(password_hash),
        user_type: Set(r#enum::user_type::UserType::Admin),
        is_system_user: Set(false),
        is_admin: Set(true),
        can_access_all_partners: Set(true),
        created_by: Set(None),
        is_active: Set(true),
        last_login: Set(None),
        created_at: Set(chrono::Utc::now().naive_utc()),
        updated_at: Set(chrono::Utc::now().naive_utc()),
    };
    super_admin.insert(db).await?;

    // For the partner admin , partner administrator user portal management
    let partner_admin_id = Uuid::new_v4();
    let partner_admin_password_hash = auth_service
        .hash_password("UpAz@?!%12345F79512=85@!")
        .await?;
    let partner_admin = user::ActiveModel {
        id: Set(partner_admin_id),
        partner_id: Set(standard_partner_id),
        email: Set("mocrypt_admin@unitedpayment.com".to_string()),
        //TODO: buna ehtiyac yoxdu
        username: Set("mocrypt_admin".to_string()),
        password_hash: Set(partner_admin_password_hash),
        user_type: Set(r#enum::user_type::UserType::Portal),
        is_system_user: Set(false),
        is_admin: Set(true),
        can_access_all_partners: Set(false),
        created_by: Set(Some(super_admin_id)),
        is_active: Set(true),
        last_login: Set(None),
        created_at: Set(chrono::Utc::now().naive_utc()),
        updated_at: Set(chrono::Utc::now().naive_utc()),
    };
    partner_admin.insert(db).await?;
    // Create system roles
    let roles = vec![
        ("super_admin", "Super Admin", "Full system access", false),
        (
            "partner_portal_admin",
            "Partner Portal Admin",
            "Full partner portal access",
            false,
        ),
        (
            "partner_admin",
            "Partner  Admin",
            "Full partner access",
            true,
        ),
        // (
        //     "finance_operator",
        //     "Finance Operator",
        //     "Manage financial operations",
        //     true,
        // ),
        // (
        //     "finance_approver",
        //     "Finance Approver",
        //     "Approve financial transactions",
        //     true,
        // ),
        // (
        //     "operations_viewer",
        //     "Operations Viewer",
        //     "View operations",
        //     true,
        // ),
        (
            "operations_manager",
            "Operations Manager",
            "Manage operations",
            true,
        ),
        // (
        //     "cross_partner_operator",
        //     "Cross Partner Operator",
        //     "Operate across partners",
        //     true,
        // ),
    ];

    for (code, name, desc, is_system) in roles {
        let role_id = Uuid::new_v4();
        let role = role::ActiveModel {
            id: Set(role_id),
            partner_id: Set(Some(main_partner_id)),
            name: Set(name.to_string()),
            code: Set(code.to_string()),
            description: Set(Some(desc.to_string())),
            is_system_role: Set(is_system),
            created_by: Set(Some(super_admin_id)),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
        };
        role.insert(db).await?;
    }

    // Create base permissions
    let resources = vec![
        "partners",
        "users",
        "roles",
        "permissions",
        "invoices",
        "transactions",
        "operations",
        "reports",
    ];
    let actions = vec!["create", "read", "update", "delete", "approve", "export"];
    // let scopes = vec![
    //     r#enum::permission_scope::PermissionScope::Own,
    //     r#enum::permission_scope::PermissionScope::Partner,
    //     r#enum::permission_scope::PermissionScope::All,
    // ];

    for resource in &resources {
        for action in &actions {
            // Önce var mı diye kontrol et
            let existing = permission::Entity::find()
                .filter(permission::Column::Resource.eq(resource.to_string()))
                .filter(permission::Column::Action.eq(action.to_string()))
                .one(db)
                .await?;

            if existing.is_none() {
                // Tek bir default scope ile ekle
                let default_scope = if *resource == "partners" {
                    PermissionScope::Partner
                } else {
                    PermissionScope::Own
                };

                // ✅ permission_id tanımla
                let permission_id = Uuid::new_v4();

                // ✅ Tüm required field'ları ekle
                let permission = permission::ActiveModel {
                    id: Set(permission_id),
                    resource: Set(resource.to_string()),
                    action: Set(action.to_string()),
                    scope: Set(default_scope),
                    conditions: Set(Some(serde_json::json!({
                        "available_scopes": ["own", "partner", "all"]
                    }))),
                    description: Set(Some(format!("Permission to {} {}", action, resource))),
                    created_at: Set(chrono::Utc::now().naive_utc()),
                };

                permission.insert(db).await?;
                println!("Created permission: {} {}", action, resource);
            } else {
                println!("Permission already exists: {} {}", action, resource);
            }
        }
    }

    // for resource in &resources {
    //     for action in &actions {
    //         for scope in &scopes {
    //             // Skip invalid combinations
    //             if *resource == "partners" && scope == &r#enum::permission_scope::PermissionScope::Own {
    //                 continue;
    //             }
    //
    //             let permission_id = Uuid::new_v4();
    //             let permission = permission::ActiveModel {
    //                 id: Set(permission_id),
    //                 resource: Set(resource.to_string()),
    //                 action: Set(action.to_string()),
    //                 scope: Set(scope.clone()),
    //                 conditions: Set(None),
    //                 description: Set(Some(format!("{} {} in {} scope", action, resource,
    //                                               match scope {
    //                                                   r#enum::permission_scope::PermissionScope::Own => "own",
    //                                                   r#enum::permission_scope::PermissionScope::Partner => "partner",
    //                                                   r#enum::permission_scope::PermissionScope::All => "all",
    //                                               }
    //                 ))),
    //                 created_at: Set(chrono::Utc::now().naive_utc()),
    //             };
    //             permission.insert(db).await?;
    //         }
    //     }
    // }
    println!("Initial data seeded successfully!");
    Ok(())
}
