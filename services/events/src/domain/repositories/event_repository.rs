use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{entities::Event, errors::DomainResult, use_cases::CreateEventParams};

#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Event>;
    async fn create(&self, data: CreateEventParams) -> DomainResult<Event>;
}
