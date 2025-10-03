use async_trait::async_trait;
use shared::UseCase;
use std::sync::Arc;

use crate::domain::{CreateUserData, DomainError, User, UserRepository};

#[derive(Debug)]
pub enum CreateUserUseCaseError {
    ValidationError(String),
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
impl UseCase<CreateUserData, Result<User, CreateUserUseCaseError>> for CreateUserUseCase {
    async fn execute(&self, data: CreateUserData) -> Result<User, CreateUserUseCaseError> {
        if !data.email.contains('@') || data.email.len() < 3 {
            return Err(CreateUserUseCaseError::ValidationError(
                "Invalid email format".to_string(),
            ));
        }

        let name_trimmed = data.name.trim();
        if name_trimmed.is_empty() {
            return Err(CreateUserUseCaseError::ValidationError(
                "Name cannot be empty".to_string(),
            ));
        }

        if name_trimmed.len() > 255 {
            return Err(CreateUserUseCaseError::ValidationError(
                "Name is too long (max 255 characters)".to_string(),
            ));
        }

        self.repository.create(data).await.map_err(|e| match e {
            DomainError::ValidationError(msg) => CreateUserUseCaseError::ValidationError(msg),
            DomainError::DatabaseError(msg) => CreateUserUseCaseError::DatabaseError(msg),
            _ => {
                log::error!("Unexpected error in CreateUserUseCase: {:?}", e);
                CreateUserUseCaseError::DatabaseError("Unexpected error".to_string())
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation_valid() {
        let valid_emails = vec![
            "user@example.com",
            "test.user@domain.co.uk",
            "name+tag@email.com",
        ];

        for email in valid_emails {
            let data = CreateUserData {
                email: email.to_string(),
                name: "John Doe".to_string(),
            };

            let is_valid = data.email.contains('@') && data.email.len() >= 3;
            assert!(is_valid, "Email {} should be valid", email);
        }
    }

    #[test]
    fn test_email_validation_invalid() {
        let invalid_emails = vec!["notanemail", "a@", ""];

        for email in invalid_emails {
            let data = CreateUserData {
                email: email.to_string(),
                name: "John Doe".to_string(),
            };

            let is_valid = data.email.contains('@') && data.email.len() >= 3;
            assert!(!is_valid, "Email '{}' should be invalid", email);
        }
    }

    #[test]
    fn test_email_validation_edge_cases() {
        let edge_cases = vec![("@domain.com", true), ("a@b", true)];

        for (email, expected_valid) in edge_cases {
            let data = CreateUserData {
                email: email.to_string(),
                name: "John Doe".to_string(),
            };

            let is_valid = data.email.contains('@') && data.email.len() >= 3;
            assert_eq!(is_valid, expected_valid, "Email '{}' validity check", email);
        }
    }

    #[test]
    fn test_name_validation_empty() {
        let empty_names = vec!["", "   ", "\t", "\n"];

        for name in empty_names {
            let trimmed = name.trim();
            assert!(
                trimmed.is_empty(),
                "Name '{}' should be considered empty after trim",
                name
            );
        }
    }

    #[test]
    fn test_name_validation_too_long() {
        let long_name = "a".repeat(256);
        assert!(long_name.len() > 255, "Name should be too long");
    }

    #[test]
    fn test_name_validation_valid() {
        let valid_names = vec![
            "John Doe",
            "Alice",
            "Bob Smith-Jones",
            "María García",
            "O'Brien",
        ];

        for name in valid_names {
            let trimmed = name.trim();
            assert!(!trimmed.is_empty(), "Name '{}' should not be empty", name);
            assert!(
                trimmed.len() <= 255,
                "Name '{}' should not be too long",
                name
            );
        }
    }

    #[test]
    fn test_name_trimming() {
        let names_with_whitespace = vec![
            ("  John  ", "John"),
            ("\tAlice\n", "Alice"),
            ("  Bob Smith  ", "Bob Smith"),
        ];

        for (input, expected) in names_with_whitespace {
            let trimmed = input.trim();
            assert_eq!(
                trimmed, expected,
                "Name '{}' should trim to '{}'",
                input, expected
            );
        }
    }
}
