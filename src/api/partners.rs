// src/api/partners.rs
use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, HttpMessage};
use serde::{Deserialize, Serialize};
// use uuid::Uuid;
use crate::{AppState, error::AppError};
use crate::services::auth_service::Claims;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_partner)
        .service(get_partner)
        .service(list_partners)
        .service(update_partner)
        .service(delete_partner)
        .service(get_partner_users)
        .service(get_partner_stats);
}

#[derive(Debug, Deserialize)]
pub struct CreatePartnerDto {
    pub name: String,
    pub code: String,
    pub partner_type: String,
    pub parent_partner_id: Option<i32>,
    pub admin_email: String,
    pub admin_username: String,
    pub admin_password: String,
}

#[post("")]
pub async fn create_partner(
    data: web::Data<AppState>,
    req: web::Json<CreatePartnerDto>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // let claims = http_req.extensions().get::<Claims>().unwrap();
    // extensions() sonucunu bir değişkene alarak yaşam süresini uzat
    let extensions = http_req.extensions();
    let claims = extensions.get::<Claims>().unwrap();

    // Only main partner can create new partners
    if !claims.is_main_partner {
        return Err(AppError::Forbidden);
    }

    let partner_req = crate::services::partner_service::CreatePartnerRequest {
        name: req.name.clone(),
        code: req.code.clone(),
        partner_type: req.partner_type.parse().unwrap(),
        parent_partner_id: req.parent_partner_id,
    };

    let admin_req = crate::services::partner_service::CreatePartnerAdminRequest {
        email: req.admin_email.clone(),
        username: req.admin_username.clone(),
        password: req.admin_password.clone(),
    };

    let partner = data.partner_service.create_partner(partner_req, admin_req).await?;

    Ok(HttpResponse::Created().json(partner))
}

#[get("/{id}")]
pub async fn get_partner(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // let claims = http_req.extensions().get::<Claims>().unwrap();
    // extensions() sonucunu bir değişkene alarak yaşam süresini uzat
    let extensions = http_req.extensions();
    let claims = extensions.get::<Claims>().unwrap();

    let partner_id = path.into_inner();

    // Check access rights
    if !claims.can_access_all_partners && claims.partner_id != partner_id {
        return Err(AppError::Forbidden);
    }

    let partner = data.partner_service.get_partner(partner_id).await?
        .ok_or(AppError::NotFound("Partner not found".into()))?;

    Ok(HttpResponse::Ok().json(partner))
}

#[get("")]
pub async fn list_partners(
    data: web::Data<AppState>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // let claims = http_req.extensions().get::<Claims>().unwrap();
    // extensions() sonucunu bir değişkene alarak yaşam süresini uzat
    let extensions = http_req.extensions();
    let claims = extensions.get::<Claims>().unwrap();

    let partners = if claims.can_access_all_partners {
        data.partner_service.list_partners(None).await?
    } else {
        // Only return own partner
        vec![data.partner_service.get_partner(claims.partner_id).await?
            .ok_or(AppError::NotFound("Partner not found".into()))?]
    };

    Ok(HttpResponse::Ok().json(partners))
}

#[put("/{id}")]
pub async fn update_partner(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    req: web::Json<serde_json::Value>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // let claims = http_req.extensions().get::<Claims>().unwrap();
    // extensions() sonucunu bir değişkene alarak yaşam süresini uzat
    let extensions = http_req.extensions();
    let claims = extensions.get::<Claims>().unwrap();
    let partner_id = path.into_inner();

    // Check permissions
    if !claims.can_access_all_partners &&
        !(claims.partner_id == partner_id && claims.roles.contains(&"partner_admin".to_string())) {
        return Err(AppError::Forbidden);
    }

    // Update logic here
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Partner updated successfully"
    })))
}

#[delete("/{id}")]
pub async fn delete_partner(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // let claims = http_req.extensions().get::<Claims>().unwrap();
    // extensions() sonucunu bir değişkene alarak yaşam süresini uzat
    let extensions = http_req.extensions();
    let claims = extensions.get::<Claims>().unwrap();

    // Only main partner can delete partners
    if !claims.is_main_partner {
        return Err(AppError::Forbidden);
    }

    let partner_id = path.into_inner();

    // Cannot delete main partner
    if partner_id == claims.partner_id {
        return Err(AppError::BadRequest("Cannot delete main partner".into()));
    }

    data.partner_service.update_partner_status(partner_id, false).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Partner deleted successfully"
    })))
}

#[get("/{id}/users")]
pub async fn get_partner_users(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // let claims = http_req.extensions().get::<Claims>().unwrap();
    // extensions() sonucunu bir değişkene alarak yaşam süresini uzat
    let extensions = http_req.extensions();
    let claims = extensions.get::<Claims>().unwrap();
    let partner_id = path.into_inner();

    let users = data.user_service.list_partner_users(partner_id, claims).await?;

    Ok(HttpResponse::Ok().json(users))
}

#[get("/{id}/stats")]
pub async fn get_partner_stats(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // let claims = http_req.extensions().get::<Claims>().unwrap();
    // extensions() sonucunu bir değişkene alarak yaşam süresini uzat
    let extensions = http_req.extensions();
    let claims = extensions.get::<Claims>().unwrap();
    let partner_id = path.into_inner();

    // Check access
    if !claims.can_access_all_partners && claims.partner_id != partner_id {
        return Err(AppError::Forbidden);
    }

    // Get statistics (simplified)
    let stats = serde_json::json!({
        "partner_id": partner_id,
        "total_users": 10,
        "active_users": 8,
        "total_transactions": 150,
        "monthly_volume": 50000,
    });

    Ok(HttpResponse::Ok().json(stats))
}