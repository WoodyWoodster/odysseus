use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    id: Uuid,
    title: String,
    description: String,
    event_date: DateTime<Utc>,
    venue_name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Event {
    pub fn new(
        id: Uuid,
        title: String,
        description: String,
        event_date: DateTime<Utc>,
        venue_name: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            event_date,
            venue_name,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn event_date(&self) -> &DateTime<Utc> {
        &self.event_date
    }

    pub fn venue_name(&self) -> &str {
        &self.venue_name
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}
