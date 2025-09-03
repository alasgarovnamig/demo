// src/api/permissions.rs
use actix_web::{get, post, web, HttpResponse, HttpRequest, HttpMessage};
use crate::{AppState, error::AppError};
use crate::services::permission_service::{CreatePermissionRequest, CreateApiPermissionRequest};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_permissions)
        .service(create_permission)
        .service(list_api_permissions)
        .service(create_api_permission)
        .service(grant_api_access);
}

#[get("")]
pub async fn list_permissions(
    data: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    use sea_orm::EntityTrait;
    let permissions = crate::entities::permission::Entity::find()
        .all(&data.db)
        .await?;

    Ok(HttpResponse::Ok().json(permissions))
}

#[post("")]
pub async fn create_permission(
    data: web::Data<AppState>,
    req: web::Json<CreatePermissionRequest>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // let claims = http_req.extensions().get::<crate::services::auth_service::Claims>().unwrap();
    let extensions = http_req.extensions();
    let claims = extensions.get::<crate::services::auth_service::Claims>().unwrap();

    // Only main partner can create permissions
    if !claims.is_main_partner {
        return Err(AppError::Forbidden);
    }

    let permission = data.permission_service.create_permission(req.into_inner()).await?;

    Ok(HttpResponse::Created().json(permission))
}

#[get("/api")]
pub async fn list_api_permissions(
    data: web::Data<AppState>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, AppError> {
    let module = query.get("module").map(|s| s.as_str());

    let permissions = if let Some(module) = module {
        data.permission_service.get_module_permissions(module).await?
    } else {
        use sea_orm::EntityTrait;
        crate::entities::api_permission::Entity::find()
            .all(&data.db)
            .await?
    };

    Ok(HttpResponse::Ok().json(permissions))
}

#[post("/api")]
pub async fn create_api_permission(
    data: web::Data<AppState>,
    req: web::Json<CreateApiPermissionRequest>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // let claims = http_req.extensions().get::<crate::services::auth_service::Claims>().unwrap();
    let extensions = http_req.extensions();
    let claims = extensions.get::<crate::services::auth_service::Claims>().unwrap();
    // Only main partner can create API permissions
    if !claims.is_main_partner {
        return Err(AppError::Forbidden);
    }

    let permission = data.permission_service.create_api_permission(req.into_inner()).await?;

    Ok(HttpResponse::Created().json(permission))
}

#[post("/api/{id}/grant/{partner_id}")]
pub async fn grant_api_access(
    data: web::Data<AppState>,
    path: web::Path<(uuid::Uuid, uuid::Uuid)>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // let claims = http_req.extensions().get::<crate::services::auth_service::Claims>().unwrap();
    let extensions = http_req.extensions();
    let claims = extensions.get::<crate::services::auth_service::Claims>().unwrap();

    let (api_permission_id, partner_id) = path.into_inner();

    // Only main partner can grant API access
    if !claims.is_main_partner {
        return Err(AppError::Forbidden);
    }

    data.permission_service.grant_api_access_to_partner(
        partner_id,
        api_permission_id,
        claims.sub
    ).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "API access granted successfully"
    })))
}