use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::use_cases::CreateEventParams;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateEventRequest {
    #[schema(example = "Summer Music Festival 2024")]
    pub title: String,
    #[schema(example = "Join us for an amazing outdoor music festival featuring top artists")]
    pub description: String,
    #[schema(example = "2024-07-15T18:00:00Z")]
    pub event_date: DateTime<Utc>,
    #[schema(example = "Central Park Amphitheater")]
    pub venue_name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateEventRequestError {
    #[error("Title is required")]
    TitleEmpty,
    #[error("Title is too long (maximum 200 characters)")]
    TitleTooLong,
    #[error("Description is too long (maximum 2000 characters)")]
    DescriptionTooLong,
    #[error("Event date must be in the future")]
    EventDateInPast,
    #[error("Venue name is required")]
    VenueNameEmpty,
    #[error("Venue name is too long (maximum 100 characters)")]
    VenueNameTooLong,
}

fn validate_title(title: &str) -> Result<String, CreateEventRequestError> {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return Err(CreateEventRequestError::TitleEmpty);
    }
    if trimmed.len() > 200 {
        return Err(CreateEventRequestError::TitleTooLong);
    }
    Ok(trimmed.to_string())
}

fn validate_description(description: &str) -> Result<String, CreateEventRequestError> {
    let trimmed = description.trim();
    if trimmed.len() > 2000 {
        return Err(CreateEventRequestError::DescriptionTooLong);
    }
    Ok(trimmed.to_string())
}

fn validate_event_date(event_date: &DateTime<Utc>) -> Result<(), CreateEventRequestError> {
    if event_date < &Utc::now() {
        return Err(CreateEventRequestError::EventDateInPast);
    }
    Ok(())
}

fn validate_venue_name(venue_name: &str) -> Result<String, CreateEventRequestError> {
    let trimmed = venue_name.trim();
    if trimmed.is_empty() {
        return Err(CreateEventRequestError::VenueNameEmpty);
    }
    if trimmed.len() > 100 {
        return Err(CreateEventRequestError::VenueNameTooLong);
    }
    Ok(trimmed.to_string())
}

impl TryFrom<CreateEventRequest> for CreateEventParams {
    type Error = CreateEventRequestError;

    fn try_from(req: CreateEventRequest) -> Result<Self, Self::Error> {
        let title = validate_title(&req.title)?;
        let description = validate_description(&req.description)?;
        validate_event_date(&req.event_date)?;
        let venue_name = validate_venue_name(&req.venue_name)?;

        Ok(Self {
            title,
            description,
            event_date: req.event_date,
            venue_name,
        })
    }
}
