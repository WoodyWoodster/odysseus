use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use shared::UseCase;
use std::sync::Arc;

use crate::domain::{DomainError, User, UserRepository};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserParams {
    pub email: String,
    pub name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateUserUseCaseError {
    #[error("Database error: {0}")]
    DatabaseError(String),
}

pub struct CreateUserUseCase {
    repository: Arc<dyn UserRepository>,
}

impl CreateUserUseCase {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<CreateUserParams, Result<User, CreateUserUseCaseError>> for CreateUserUseCase {
    async fn execute(&self, data: CreateUserParams) -> Result<User, CreateUserUseCaseError> {
        self.repository.create(data).await.map_err(|e| match e {
            DomainError::DatabaseError(msg) => CreateUserUseCaseError::DatabaseError(msg),
            _ => {
                log::error!("Unexpected error in CreateUserUseCase: {:?}", e);
                CreateUserUseCaseError::DatabaseError("Unexpected error".to_string())
            }
        })
    }
}
