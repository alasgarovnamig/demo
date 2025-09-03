// src/api/users.rs
use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, HttpMessage};
use uuid::Uuid;
use crate::{AppState, error::AppError};
use crate::services::auth_service::Claims;
use crate::services::user_service::CreateUserRequest;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        // .service(create_user)
        // .service(get_user)
        .service(update_user)
        .service(delete_user)
        .service(assign_role_to_user)
        .service(remove_role_from_user);
        // .service(get_user_permissions);
}

// #[post("")]
// pub async fn create_user(
//     data: web::Data<AppState>,
//     req: web::Json<CreateUserRequest>,
//     http_req: HttpRequest,
// ) -> Result<HttpResponse, AppError> {
//     let claims = http_req.extensions().get::<Claims>().unwrap();
//
//     // Determine partner_id
//     let partner_id = if claims.can_access_all_partners {
//         req.partner_id.unwrap_or(claims.partner_id)
//     } else {
//         claims.partner_id
//     };
//
//     let user = data.user_service.create_user(
//         partner_id,
//         req.into_inner(),
//         claims.sub,
//         claims
//     ).await?;
//
//     Ok(HttpResponse::Created().json(user))
// }

// #[get("/{id}")]
// pub async fn get_user(
//     data: web::Data<AppState>,
//     path: web::Path<Uuid>,
//     http_req: HttpRequest,
// ) -> Result<HttpResponse, AppError> {
//     let claims = http_req.extensions().get::<Claims>().unwrap();
//     let user_id = path.into_inner();
//
//     // Get user from database
//     let user = crate::entities::user::Entity::find_by_id(user_id)
//         .one(&data.db)
//         .await?
//         .ok_or(AppError::NotFound("User not found".into()))?;
//
//     // Check access
//     if !claims.can_access_all_partners &&
//         claims.partner_id != user.partner_id &&
//         claims.sub != user_id {
//         return Err(AppError::Forbidden);
//     }
//
//     Ok(HttpResponse::Ok().json(user))
// }

#[put("/{id}")]
pub async fn update_user(
    data: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: web::Json<serde_json::Value>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = http_req.extensions().get::<Claims>().unwrap();
    let user_id = path.into_inner();

    // Implementation for user update
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "User updated successfully"
    })))
}

#[delete("/{id}")]
pub async fn delete_user(
    data: web::Data<AppState>,
    path: web::Path<Uuid>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // let claims = http_req.extensions().get::<Claims>().unwrap();
    // extensions() sonucunu bir değişkene alarak yaşam süresini uzat
    let extensions = http_req.extensions();
    let claims = extensions.get::<Claims>().unwrap();
    let user_id = path.into_inner();

    // Cannot delete self
    if claims.sub == user_id {
        return Err(AppError::BadRequest("Cannot delete yourself".into()));
    }

    // Implementation for user deletion
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "User deleted successfully"
    })))
}

#[post("/{id}/roles/{role_id}")]
pub async fn assign_role_to_user(
    data: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // let claims = http_req.extensions().get::<Claims>().unwrap();
    // extensions() sonucunu bir değişkene alarak yaşam süresini uzat
    let extensions = http_req.extensions();
    let claims = extensions.get::<Claims>().unwrap();

    let (user_id, role_id) = path.into_inner();

    data.user_service.assign_role(user_id, role_id, claims.sub, claims).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Role assigned successfully"
    })))
}

#[delete("/{id}/roles/{role_id}")]
pub async fn remove_role_from_user(
    data: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = http_req.extensions().get::<Claims>().unwrap();
    let (user_id, role_id) = path.into_inner();

    // Implementation for removing role
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Role removed successfully"
    })))
}

// #[get("/{id}/permissions")]
// pub async fn get_user_permissions(
//     data: web::Data<AppState>,
//     path: web::Path<Uuid>,
//     http_req: HttpRequest,
// ) -> Result<HttpResponse, AppError> {
//     let claims = http_req.extensions().get::<Claims>().unwrap();
//     let user_id = path.into_inner();
//
//     // Check if user can view permissions
//     if claims.sub != user_id && !claims.is_main_partner {
//         return Err(AppError::Forbidden);
//     }
//
//     // Get permissions through user service
//     let permissions = data.auth_service.get_user_permissions(user_id).await?;
//
//     Ok(HttpResponse::Ok().json(permissions))
// }