use actix_web::{App, HttpServer, middleware as actix_middleware, web};
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::env;

mod entities;
mod middleware;
mod services;

mod api;
mod error;
mod seed;
mod startup;

pub struct AppState {
    db: DatabaseConnection,
    auth_service: services::auth_service::AuthService,
    partner_service: services::partner_service::PartnerService,
    user_service: services::user_service::UserService,
    permission_service: services::permission_service::PermissionService,
    opa_service: services::opa_service::OpaService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // Database connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    // Initialize services
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let opa_url = env::var("OPA_URL").unwrap_or_else(|_| "http://localhost:8181".to_string());

    let auth_service = services::auth_service::AuthService::new(db.clone(), jwt_secret);
    let partner_service =
        services::partner_service::PartnerService::new(db.clone(), auth_service.clone());
    let user_service = services::user_service::UserService::new(db.clone(), auth_service.clone());
    let permission_service = services::permission_service::PermissionService::new(db.clone());


    // let opa_service = services::opa_service::OpaService::new(opa_url);
    // OPA servisini oluşturmak için opa_url'i clone'la
    let opa_url_clone = opa_url.clone();
    let opa_service = services::opa_service::OpaService::new(opa_url_clone);

    // Seed initial data if needed
    if env::var("SEED_DATA").unwrap_or_else(|_| "false".to_string()) == "true" {
        seed::seed_initial_data(&db, &auth_service)
            .await
            .expect("Failed to seed data");
    }

    // ✅ OPA'ya verileri yükle
    startup::initialize_opa_data(&db, opa_url)
        .await
        .expect("Failed to initialize OPA data");

    let app_state = web::Data::new(AppState {
        db: db.clone(),
        auth_service: auth_service.clone(),
        partner_service,
        user_service,
        permission_service,
        opa_service: opa_service.clone(),
    });

    println!("Starting server on http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(actix_middleware::Logger::default())
            .wrap(actix_middleware::NormalizePath::trim())
            .wrap(
                actix_middleware::DefaultHeaders::new()
                    .header("X-Version", "1.0")
                    .header("X-Content-Type-Options", "nosniff"),
            )
            .service(
                web::scope("/api")
                    .wrap(middleware::auth::AuthMiddleware::new(
                        auth_service.clone(),
                        opa_service.clone(),
                    ))
                    .configure(api::config),
            )
            .service(
                web::scope("/auth")
                    .service(api::auth::login)
                    .service(api::auth::refresh),
            )
            .service(api::health::health_check)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
