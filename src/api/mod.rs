// src/api/mod.rs
pub mod auth;
pub mod partners;
pub mod users;
pub mod roles;
pub mod permissions;
pub mod health;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/partners")
            .configure(partners::config)
    )
        .service(
            actix_web::web::scope("/users")
                .configure(users::config)
        )
        .service(
            actix_web::web::scope("/roles")
                .configure(roles::config)
        )
        .service(
            actix_web::web::scope("/permissions")
                .configure(permissions::config)
        );
}