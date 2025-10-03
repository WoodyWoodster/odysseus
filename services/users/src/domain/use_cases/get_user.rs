use async_trait::async_trait;
use shared::UseCase;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{DomainResult, User, UserRepository};

pub struct GetUserUseCase {
    repository: Arc<dyn UserRepository>,
}

impl GetUserUseCase {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<Uuid, DomainResult<User>> for GetUserUseCase {
    async fn execute(&self, user_id: Uuid) -> DomainResult<User> {
        self.repository.find_by_id(user_id).await
    }
}
