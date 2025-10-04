use actix_web::{web, HttpResponse, Responder};
use shared::UseCase;

use crate::domain::{CreateEventUseCase, CreateEventUseCaseError};

use super::super::dtos::{CreateEventError, CreateEventRequest, CreateEventResponse};

#[utoipa::path(
    post,
    path = "/events/api/events",
    request_body = CreateEventRequest,
    responses(
        (status = 201, description = "Event created successfully", body = CreateEventResponse),
        (status = 400, description = "Invalid request", body = CreateEventError),
        (status = 500, description = "Internal server error", body = CreateEventError)
    ),
    tag = "events"
)]
pub async fn create_event(
    body: web::Json<CreateEventRequest>,
    use_case: web::Data<CreateEventUseCase>,
) -> impl Responder {
    use crate::domain::use_cases::CreateEventParams;

    let data: CreateEventParams = match body.into_inner().try_into() {
        Ok(data) => data,
        Err(e) => {
            return HttpResponse::BadRequest().json(CreateEventError {
                error: e.to_string(),
            });
        }
    };

    match use_case.execute(data).await {
        Ok(event) => {
            let response: CreateEventResponse = event.into();
            HttpResponse::Created().json(response)
        }
        Err(CreateEventUseCaseError::DatabaseError(msg)) => {
            HttpResponse::InternalServerError().json(CreateEventError { error: msg })
        }
    }
}
