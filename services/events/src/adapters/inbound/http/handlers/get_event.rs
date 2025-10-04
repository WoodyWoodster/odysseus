use actix_web::{web, HttpResponse, Responder};
use shared::UseCase;
use uuid::Uuid;

use crate::domain::{GetEventUseCase, GetEventUseCaseError};

use super::super::dtos::{GetEventError, GetEventResponse};

#[utoipa::path(
    get,
    path = "/events/api/events/{id}",
    params(
        ("id" = Uuid, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "Event found", body = GetEventResponse),
        (status = 404, description = "Event not found", body = GetEventError),
        (status = 500, description = "Internal server error", body = GetEventError)
    ),
    tag = "events"
)]
pub async fn get_event(
    path: web::Path<Uuid>,
    use_case: web::Data<GetEventUseCase>,
) -> impl Responder {
    let id = path.into_inner();

    match use_case.execute(id).await {
        Ok(event) => {
            let response: GetEventResponse = event.into();
            HttpResponse::Ok().json(response)
        }
        Err(GetEventUseCaseError::NotFound) => {
            HttpResponse::NotFound().json(GetEventError {
                error: "Event not found".to_string(),
            })
        }
        Err(GetEventUseCaseError::DatabaseError(msg)) => {
            HttpResponse::InternalServerError().json(GetEventError { error: msg })
        }
    }
}
