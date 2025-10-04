use actix_web::{web, HttpResponse, Responder};
use shared::UseCase;

use crate::domain::{CreateUserUseCase, CreateUserUseCaseError};

use super::super::dtos::{CreateUserError, CreateUserRequest, CreateUserResponse};

#[utoipa::path(
    post,
    path = "/users/api/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = CreateUserResponse),
        (status = 400, description = "Invalid request", body = CreateUserError),
        (status = 500, description = "Internal server error", body = CreateUserError)
    ),
    tag = "users"
)]
pub async fn create_user(
    body: web::Json<CreateUserRequest>,
    use_case: web::Data<CreateUserUseCase>,
) -> impl Responder {
    use crate::domain::use_cases::CreateUserParams;

    let data: CreateUserParams = match body.into_inner().try_into() {
        Ok(data) => data,
        Err(e) => {
            return HttpResponse::BadRequest().json(CreateUserError {
                error: e.to_string(),
            });
        }
    };

    match use_case.execute(data).await {
        Ok(user) => {
            let response: CreateUserResponse = user.into();
            HttpResponse::Created().json(response)
        }
        Err(CreateUserUseCaseError::DatabaseError(msg)) => {
            HttpResponse::InternalServerError().json(CreateUserError { error: msg })
        }
    }
}
