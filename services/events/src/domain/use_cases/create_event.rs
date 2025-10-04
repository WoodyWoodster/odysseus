use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::UseCase;
use std::sync::Arc;

use crate::domain::{DomainError, Event, EventRepository};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEventParams {
    pub title: String,
    pub description: String,
    pub event_date: DateTime<Utc>,
    pub venue_name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateEventUseCaseError {
    #[error("Database error: {0}")]
    DatabaseError(String),
}

pub struct CreateEventUseCase {
    repository: Arc<dyn EventRepository>,
}

impl CreateEventUseCase {
    pub fn new(repository: Arc<dyn EventRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<CreateEventParams, Result<Event, CreateEventUseCaseError>> for CreateEventUseCase {
    async fn execute(&self, data: CreateEventParams) -> Result<Event, CreateEventUseCaseError> {
        self.repository.create(data).await.map_err(|e| match e {
            DomainError::DatabaseError(msg) => CreateEventUseCaseError::DatabaseError(msg),
            _ => {
                log::error!("Unexpected error in CreateEventUseCase: {:?}", e);
                CreateEventUseCaseError::DatabaseError("Unexpected error".to_string())
            }
        })
    }
}
