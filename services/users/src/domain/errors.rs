pub type DomainResult<T> = Result<T, DomainError>;

#[derive(Debug)]
pub enum DomainError {
    NotFound(String),
    DatabaseError(String),
    ValidationError(String),
}
