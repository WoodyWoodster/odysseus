use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateUserError {
    #[schema(example = "Email already exists")]
    pub error: String,
}
