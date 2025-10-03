use actix_web::{web, HttpResponse, Responder};
use shared::UseCase;
use uuid::Uuid;

use crate::domain::{DomainError, GetUserUseCase, User};

use super::super::dtos::GetUserError;

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found successfully", body = User),
        (status = 404, description = "User not found", body = GetUserError),
        (status = 500, description = "Internal server error", body = GetUserError)
    ),
    tag = "users"
)]
pub async fn get_user(
    path: web::Path<Uuid>,
    use_case: web::Data<GetUserUseCase>,
) -> impl Responder {
    let user_id = path.into_inner();

    match use_case.execute(user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(DomainError::NotFound(msg)) => {
            HttpResponse::NotFound().json(GetUserError { error: msg })
        }
        Err(DomainError::DatabaseError(msg)) => {
            HttpResponse::InternalServerError().json(GetUserError { error: msg })
        }
        Err(DomainError::ValidationError(msg)) => {
            HttpResponse::BadRequest().json(GetUserError { error: msg })
        }
    }
}
