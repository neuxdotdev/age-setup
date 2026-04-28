pub mod buildings;
pub mod security;
pub mod validation;
pub use buildings::GenerationError;
pub use security::SecurityError;
pub use validation::ValidationError;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Key generation failed: {0}")]
    Generation(#[from] GenerationError),
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationError),
    #[error("Security operation failed: {0}")]
    Security(#[from] SecurityError),
}
pub type Result<T> = std::result::Result<T, Error>;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn error_from_generation() {
        let err = Error::from(GenerationError::IdentityCreationFailed);
        assert!(matches!(err, Error::Generation(_)));
    }
    #[test]
    fn result_type_works() {
        let ok: Result<i32> = Ok(42);
        let err: Result<i32> = Err(Error::from(ValidationError::invalid_public_key("test")));
        assert_eq!(ok.unwrap(), 42);
        assert!(err.is_err());
    }
}