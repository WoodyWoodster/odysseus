use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{commands::CreateUserData, entities::User, errors::DomainResult};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<User>;
    async fn create(&self, data: CreateUserData) -> DomainResult<User>;
}
