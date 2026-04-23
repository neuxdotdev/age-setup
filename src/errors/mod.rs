//! Error handling types.

pub mod buildings;
pub mod security;
pub mod validation;

pub use buildings::GenerationError;
pub use security::SecurityError;
pub use validation::ValidationError;

/// Main error type for the crate.
///
/// Encompasses all possible errors during key generation, validation, or security operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error during key generation (e.g., internal library failure).
    #[error("Key generation failed: {0}")]
    Generation(#[from] GenerationError),
    /// Error validating public key format.
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationError),
    /// Error during security operations (e.g., memory wipe).
    #[error("Security operation failed: {0}")]
    Security(#[from] SecurityError),
}

/// Result type alias using `Error` from this crate.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_from_generation() {
        let gen_err = GenerationError::IdentityCreationFailed;
        let err: Error = gen_err.into();
        assert!(matches!(err, Error::Generation(_)));
    }

    #[test]
    fn test_error_from_validation() {
        let val_err = ValidationError::invalid_public_key("test");
        let err: Error = val_err.into();
        assert!(matches!(err, Error::Validation(_)));
    }

    #[test]
    fn test_error_from_security() {
        let sec_err = SecurityError::MemoryWipeFailed;
        let err: Error = sec_err.into();
        assert!(matches!(err, Error::Security(_)));
    }

    #[test]
    fn test_error_display() {
        let err = Error::Generation(GenerationError::IdentityCreationFailed);
        assert_eq!(
            format!("{}", err),
            "Key generation failed: Age identity generation failed: internal library error"
        );
    }
}
