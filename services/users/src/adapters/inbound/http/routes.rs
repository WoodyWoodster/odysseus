use actix_web::web;

use super::handlers::{create_user, get_user};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .route("", web::post().to(create_user))
            .route("/{id}", web::get().to(get_user)),
    );
}
