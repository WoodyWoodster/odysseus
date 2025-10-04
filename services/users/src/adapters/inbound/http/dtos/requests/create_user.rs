use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::use_cases::CreateUserParams;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    #[schema(example = "user@example.com")]
    pub email: String,
    #[schema(example = "John Doe")]
    pub name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateUserRequestError {
    #[error("Email must contain an @ symbol")]
    EmailMissingAtSymbol,
    #[error("Email is too short (minimum 3 characters)")]
    EmailTooShort,
    #[error("Email is too long (maximum 254 characters)")]
    EmailTooLong,
    #[error("Name cannot be empty")]
    NameEmpty,
    #[error("Name is too long (maximum 255 characters)")]
    NameTooLong,
}

fn validate_email(email: &str) -> Result<String, CreateUserRequestError> {
    if !email.contains('@') {
        return Err(CreateUserRequestError::EmailMissingAtSymbol);
    }
    if email.len() < 3 {
        return Err(CreateUserRequestError::EmailTooShort);
    }
    if email.len() > 254 {
        return Err(CreateUserRequestError::EmailTooLong);
    }
    Ok(email.to_string())
}

fn validate_name(name: &str) -> Result<String, CreateUserRequestError> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(CreateUserRequestError::NameEmpty);
    }
    if trimmed.len() > 255 {
        return Err(CreateUserRequestError::NameTooLong);
    }
    Ok(trimmed.to_string())
}

impl TryFrom<CreateUserRequest> for CreateUserParams {
    type Error = CreateUserRequestError;

    fn try_from(req: CreateUserRequest) -> Result<Self, Self::Error> {
        let email = validate_email(&req.email)?;
        let name = validate_name(&req.name)?;

        Ok(Self { email, name })
    }
}
