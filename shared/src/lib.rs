use async_trait::async_trait;

/// Generic use case trait that all use cases should implement.
///
/// # Type Parameters
/// * `Input` - The input type for the use case (e.g., CreateUserData, Uuid)
/// * `Output` - The output type for the use case (e.g., User, Order)
#[async_trait]
pub trait UseCase<Input, Output> {
    /// Executes the use case with the given input.
    async fn execute(&self, input: Input) -> Output;
}
