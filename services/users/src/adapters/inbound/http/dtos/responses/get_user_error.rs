use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct GetUserError {
    #[schema(example = "User not found")]
    pub error: String,
}
