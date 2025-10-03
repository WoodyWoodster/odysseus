use async_trait::async_trait;
use shared::UseCase;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{DomainError, User, UserRepository};

#[derive(Debug)]
pub enum GetUserUseCaseError {
    NotFound(String),
    DatabaseError(String),
}

pub struct GetUserUseCase {
    repository: Arc<dyn UserRepository>,
}

impl GetUserUseCase {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<Uuid, Result<User, GetUserUseCaseError>> for GetUserUseCase {
    async fn execute(&self, user_id: Uuid) -> Result<User, GetUserUseCaseError> {
        self.repository
            .find_by_id(user_id)
            .await
            .map_err(|e| match e {
                DomainError::NotFound(msg) => GetUserUseCaseError::NotFound(msg),
                DomainError::DatabaseError(msg) => GetUserUseCaseError::DatabaseError(msg),
                _ => {
                    log::error!("Unexpected error in GetUserUseCase: {:?}", e);
                    GetUserUseCaseError::DatabaseError("Unexpected error".to_string())
                }
            })
    }
}
