use async_trait::async_trait;
use shared::UseCase;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{DomainError, Event, EventRepository};

#[derive(Debug, thiserror::Error)]
pub enum GetEventUseCaseError {
    #[error("Event not found")]
    NotFound,
    #[error("Database error: {0}")]
    DatabaseError(String),
}

pub struct GetEventUseCase {
    repository: Arc<dyn EventRepository>,
}

impl GetEventUseCase {
    pub fn new(repository: Arc<dyn EventRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<Uuid, Result<Event, GetEventUseCaseError>> for GetEventUseCase {
    async fn execute(&self, id: Uuid) -> Result<Event, GetEventUseCaseError> {
        self.repository.find_by_id(id).await.map_err(|e| match e {
            DomainError::NotFound(_) => GetEventUseCaseError::NotFound,
            DomainError::DatabaseError(msg) => GetEventUseCaseError::DatabaseError(msg),
        })
    }
}
