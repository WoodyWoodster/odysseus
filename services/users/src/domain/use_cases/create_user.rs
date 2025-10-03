use std::sync::Arc;

use crate::domain::{CreateUserData, DomainError, DomainResult, User, UserRepository};

pub struct CreateUserUseCase {
    repository: Arc<dyn UserRepository>,
}

impl CreateUserUseCase {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, data: CreateUserData) -> DomainResult<User> {
        if !data.email.contains('@') || data.email.len() < 3 {
            return Err(DomainError::ValidationError(
                "Invalid email format".to_string(),
            ));
        }

        let name_trimmed = data.name.trim();
        if name_trimmed.is_empty() {
            return Err(DomainError::ValidationError(
                "Name cannot be empty".to_string(),
            ));
        }

        if name_trimmed.len() > 255 {
            return Err(DomainError::ValidationError(
                "Name is too long (max 255 characters)".to_string(),
            ));
        }

        self.repository.create(data).await
    }
}
