pub type DomainResult<T> = Result<T, DomainError>;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[allow(dead_code)]
    #[error("Validation error: {0}")]
    ValidationError(String),
}
