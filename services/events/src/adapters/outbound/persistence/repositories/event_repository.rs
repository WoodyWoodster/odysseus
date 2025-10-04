use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use crate::domain::{use_cases::CreateEventParams, DomainError, DomainResult, Event, EventRepository};

use super::super::models::event::{ActiveModel, Entity as EventEntity};

pub struct EventRepositoryImpl {
    db: DatabaseConnection,
}

impl EventRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl EventRepository for EventRepositoryImpl {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Event> {
        let event = EventEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound(format!("Event with id {} not found", id)))?;

        Ok(event.into())
    }

    async fn create(&self, data: CreateEventParams) -> DomainResult<Event> {
        let now = Utc::now();
        let event_id = Uuid::new_v4();

        let event = ActiveModel {
            id: Set(event_id),
            title: Set(data.title),
            description: Set(data.description),
            event_date: Set(data.event_date),
            venue_name: Set(data.venue_name),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let inserted = event
            .insert(&self.db)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(inserted.into())
    }
}
