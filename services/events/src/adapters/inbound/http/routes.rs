use actix_web::web;

use super::handlers::{create_event, get_event};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/events")
            .route("", web::post().to(create_event::create_event))
            .route("/{id}", web::get().to(get_event::get_event)),
    );
}
