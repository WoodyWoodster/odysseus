mod adapters;
mod domain;

use actix_web::{middleware::Logger, web, App, HttpServer};
use adapters::inbound::http::configure_routes;
use adapters::outbound::persistence::UserRepositoryImpl;
use domain::{CreateUserUseCase, GetUserUseCase};
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::prelude::*;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use adapters::inbound::http::dtos::{
    CreateUserError, CreateUserRequest, CreateUserResponse, GetUserError, GetUserResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        adapters::inbound::http::handlers::get_user::get_user,
        adapters::inbound::http::handlers::create_user::create_user,
    ),
    components(
        schemas(
            CreateUserRequest,
            CreateUserResponse,
            CreateUserError,
            GetUserResponse,
            GetUserError
        )
    ),
    tags(
        (name = "users", description = "User management endpoints")
    ),
    info(
        title = "Users Service API",
        version = "0.1.0",
        description = "RESTful API for user management using hexagonal architecture"
    )
)]
struct ApiDoc;

async fn create_database_connection() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    let db = Database::connect(&database_url).await?;
    Ok(db)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    println!("🔌 Connecting to database...");
    let db = match create_database_connection().await {
        Ok(db) => {
            println!("✅ Database connection established");
            db
        }
        Err(e) => {
            eprintln!("❌ Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    println!("🔄 Running database migrations...");
    if let Err(e) = migration::Migrator::up(&db, None).await {
        eprintln!("❌ Failed to run migrations: {}", e);
        std::process::exit(1);
    }
    println!("✅ Database migrations completed");

    let user_repository = Arc::new(UserRepositoryImpl::new(db));

    let get_user_use_case = web::Data::new(GetUserUseCase::new(user_repository.clone()));
    let create_user_use_case = web::Data::new(CreateUserUseCase::new(user_repository.clone()));

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", host, port);

    println!("🚀 Server starting on http://{}", bind_address);
    println!(
        "📚 Swagger UI available at http://{}/swagger-ui/",
        bind_address
    );

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(get_user_use_case.clone())
            .app_data(create_user_use_case.clone())
            .configure(configure_routes)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(&bind_address)?
    .run()
    .await
}
