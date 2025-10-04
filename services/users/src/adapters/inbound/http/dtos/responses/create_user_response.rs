use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::User;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "user@example.com")]
    pub email: String,
    #[schema(example = "John Doe")]
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for CreateUserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id(),
            email: user.email().to_string(),
            name: user.name().to_string(),
            created_at: *user.created_at(),
            updated_at: *user.updated_at(),
        }
    }
}
