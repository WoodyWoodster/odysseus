mod adapters;
mod domain;

use actix_web::{middleware::Logger, web, App, HttpServer};
use adapters::inbound::http::configure_routes;
use adapters::outbound::persistence::EventRepositoryImpl;
use domain::{CreateEventUseCase, GetEventUseCase};
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use adapters::inbound::http::dtos::{
    CreateEventError, CreateEventRequest, CreateEventResponse, GetEventError, GetEventResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        adapters::inbound::http::handlers::get_event::get_event,
        adapters::inbound::http::handlers::create_event::create_event,
    ),
    components(
        schemas(
            CreateEventRequest,
            CreateEventResponse,
            CreateEventError,
            GetEventResponse,
            GetEventError
        )
    ),
    tags(
        (name = "events", description = "Event management endpoints")
    ),
    info(
        title = "Events Service API",
        version = "0.1.0",
        description = "RESTful API for event management using hexagonal architecture"
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

    let event_repository = Arc::new(EventRepositoryImpl::new(db));

    let get_event_use_case = web::Data::new(GetEventUseCase::new(event_repository.clone()));
    let create_event_use_case = web::Data::new(CreateEventUseCase::new(event_repository.clone()));

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8081".to_string());
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
            .app_data(get_event_use_case.clone())
            .app_data(create_event_use_case.clone())
            .service(
                web::scope("/events")
                    .configure(configure_routes)
                    .route(
                        "/api-docs/openapi.json",
                        web::get().to({
                            let openapi = openapi.clone();
                            move || {
                                let openapi = openapi.clone();
                                async move {
                                    actix_web::HttpResponse::Ok()
                                        .content_type("application/json")
                                        .json(openapi)
                                }
                            }
                        }),
                    )
                    .service(
                        SwaggerUi::new("/swagger-ui/{_:.*}")
                            .url("/events/api-docs/openapi.json", openapi.clone()),
                    ),
            )
    })
    .bind(&bind_address)?
    .run()
    .await
}
