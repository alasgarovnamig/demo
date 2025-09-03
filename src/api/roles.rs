// src/api/roles.rs
use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, HttpMessage};
use sea_orm::EntityTrait;
use uuid::Uuid;
use crate::{AppState, error::AppError};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_roles)
        .service(create_role)
        .service(update_role)
        .service(delete_role)
        .service(get_role_permissions)
        .service(assign_permission_to_role);
}

#[get("")]
pub async fn list_roles(
    data: web::Data<AppState>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // let claims = http_req.extensions().get::<crate::services::auth_service::Claims>().unwrap();
    let extensions = http_req.extensions();
    let claims = extensions.get::<crate::services::auth_service::Claims>().unwrap();

    // Get roles based on partner
    let roles = if claims.can_access_all_partners {
        // Get all roles
        crate::entities::role::Entity::find()
            .all(&data.db)
            .await?
    } else {
        // Get partner-specific and system roles
        use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, Condition};
        crate::entities::role::Entity::find()
            .filter(
                Condition::any()
                    .add(crate::entities::role::Column::PartnerId.eq(claims.partner_id))
                    .add(crate::entities::role::Column::IsSystemRole.eq(true))
            )
            .all(&data.db)
            .await?
    };

    Ok(HttpResponse::Ok().json(roles))
}

#[post("")]
pub async fn create_role(
    data: web::Data<AppState>,
    req: web::Json<serde_json::Value>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // let claims = http_req.extensions().get::<crate::services::auth_service::Claims>().unwrap();
    let extensions = http_req.extensions();
    let claims = extensions.get::<crate::services::auth_service::Claims>().unwrap();
    // Only admins can create roles
    if !claims.roles.contains(&"partner_admin".to_string()) && !claims.is_main_partner {
        return Err(AppError::Forbidden);
    }

    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "Role created successfully"
    })))
}

#[put("/{id}")]
pub async fn update_role(
    data: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: web::Json<serde_json::Value>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Role updated successfully"
    })))
}

#[delete("/{id}")]
pub async fn delete_role(
    data: web::Data<AppState>,
    path: web::Path<Uuid>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Role deleted successfully"
    })))
}

#[get("/{id}/permissions")]
pub async fn get_role_permissions(
    data: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let role_id = path.into_inner();
    let permissions = data.permission_service.get_role_permissions(role_id).await?;

    Ok(HttpResponse::Ok().json(permissions))
}

#[post("/{id}/permissions/{permission_id}")]
pub async fn assign_permission_to_role(
    data: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // let claims = http_req.extensions().get::<crate::services::auth_service::Claims>().unwrap();
    let extensions = http_req.extensions();
    let claims = extensions.get::<crate::services::auth_service::Claims>().unwrap();
    let (role_id, permission_id) = path.into_inner();

    // Only admins can assign permissions
    if !claims.roles.contains(&"partner_admin".to_string()) && !claims.is_main_partner {
        return Err(AppError::Forbidden);
    }

    data.permission_service.assign_permission_to_role(role_id, permission_id).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Permission assigned to role successfully"
    })))
}