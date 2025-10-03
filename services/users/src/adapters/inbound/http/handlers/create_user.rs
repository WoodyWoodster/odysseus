use actix_web::{web, HttpResponse, Responder};
use shared::UseCase;

use crate::domain::{CreateUserData, CreateUserUseCase, DomainError, User};

use super::super::dtos::{CreateUserError, CreateUserRequest};

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 400, description = "Invalid request", body = CreateUserError),
        (status = 500, description = "Internal server error", body = CreateUserError)
    ),
    tag = "users"
)]
pub async fn create_user(
    body: web::Json<CreateUserRequest>,
    use_case: web::Data<CreateUserUseCase>,
) -> impl Responder {
    let data = CreateUserData {
        email: body.email.clone(),
        name: body.name.clone(),
    };

    match use_case.execute(data).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(DomainError::DatabaseError(msg)) => {
            HttpResponse::InternalServerError().json(CreateUserError { error: msg })
        }
        Err(DomainError::ValidationError(msg)) => {
            HttpResponse::BadRequest().json(CreateUserError { error: msg })
        }
        Err(DomainError::NotFound(msg)) => {
            HttpResponse::NotFound().json(CreateUserError { error: msg })
        }
    }
}
