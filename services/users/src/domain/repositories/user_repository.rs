use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{entities::User, errors::DomainResult, use_cases::CreateUserParams};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<User>;
    async fn create(&self, data: CreateUserParams) -> DomainResult<User>;
}
