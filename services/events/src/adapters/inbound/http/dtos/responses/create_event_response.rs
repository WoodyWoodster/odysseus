use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::Event;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateEventResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "Summer Music Festival 2024")]
    pub title: String,
    #[schema(example = "Join us for an amazing outdoor music festival")]
    pub description: String,
    #[schema(example = "2024-07-15T18:00:00Z")]
    pub event_date: DateTime<Utc>,
    #[schema(example = "Central Park Amphitheater")]
    pub venue_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Event> for CreateEventResponse {
    fn from(event: Event) -> Self {
        Self {
            id: event.id(),
            title: event.title().to_string(),
            description: event.description().to_string(),
            event_date: *event.event_date(),
            venue_name: event.venue_name().to_string(),
            created_at: *event.created_at(),
            updated_at: *event.updated_at(),
        }
    }
}
