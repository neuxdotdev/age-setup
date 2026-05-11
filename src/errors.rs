//! Error types for the `age-setup` crate.
//!
//! This module defines the central [`Error`] enum that encompasses all possible
//! failures in key generation and validation. It is designed to provide clear,
//! actionable error messages through the [`thiserror`] crate.

use thiserror::Error;

/// The main error type for the `age-setup` crate.
///
/// All fallible operations in the library return a [`Result<T>`] where the error
/// is of this type. It is deliberately kept small and covers only the failure
/// modes that can occur:
///
/// - **Generation failures** – when the underlying `age` library fails to create
///   a new identity.
/// - **Validation failures** – when a public or secret key does not meet the
///   expected format.
///
/// # Example
///
/// ```rust
/// use age_setup::Error;
///
/// fn handle_error(e: Error) {
///     match e {
///         Error::Generation(err) => eprintln!("Generation error: {}", err),
///         Error::Validation(err) => eprintln!("Validation error: {}", err),
///     }
/// }
/// ```
#[derive(Debug, Error)]
pub enum Error {
    /// An error that occurred during key generation.
    ///
    /// This variant wraps [`GenerationError`] and is produced by
    /// [`build_keypair`](crate::build_keypair) if the internal identity generation
    /// fails.
    #[error("Key generation failed: {0}")]
    Generation(#[from] GenerationError),

    /// An error that occurred while validating a public or secret key.
    ///
    /// This variant wraps [`ValidationError`] and is produced by constructors like
    /// [`PublicKey::new`](crate::PublicKey::new) or
    /// [`SecretKey::new`](crate::SecretKey::new) when the provided string does
    /// not conform to the expected format.
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationError),
}

/// The standard `Result` type used throughout the crate.
///
/// This is a convenience alias for `std::result::Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during key generation.
///
/// Currently, the only possible failure is if the `age` library fails to create
/// a new X25519 identity. This should be extremely rare and typically indicates
/// a system-level issue (e.g., lack of entropy).
#[derive(Debug, Error)]
pub enum GenerationError {
    /// The identity could not be created.
    ///
    /// This error is returned when the underlying `age` crate encounters an
    /// internal failure while generating a new X25519 keypair.
    #[error("Identity generation failed")]
    IdentityCreationFailed,
}

/// Errors that can occur while validating a public or secret key.
///
/// Both public and secret keys are validated for basic format correctness before
/// being wrapped. This enum captures the specific reason for the validation
/// failure.
#[derive(Debug, Error)]
pub enum ValidationError {
    /// The provided public key string is not in a valid format.
    ///
    /// This error is returned when a public key is empty or does not start with
    /// the `"age1"` prefix.
    #[error("Invalid public key format: {reason}")]
    InvalidPublicKeyFormat { reason: String },

    /// The provided secret key string is not in a valid format.
    ///
    /// This error is returned when a secret key is empty or does not start with
    /// `"AGE-SECRET-KEY-1"`.
    #[error("Invalid secret key format: {reason}")]
    InvalidSecretKeyFormat { reason: String },
}

impl ValidationError {
    /// Creates a new [`ValidationError::InvalidPublicKeyFormat`] with the given reason.
    ///
    /// This is a convenience constructor used internally by the public key validation
    /// logic.
    pub(crate) fn invalid_public_key(reason: impl Into<String>) -> Self {
        Self::InvalidPublicKeyFormat {
            reason: reason.into(),
        }
    }

    /// Creates a new [`ValidationError::InvalidSecretKeyFormat`] with the given reason.
    ///
    /// This is a convenience constructor used internally by the secret key validation
    /// logic.
    pub(crate) fn invalid_secret_key(reason: impl Into<String>) -> Self {
        Self::InvalidSecretKeyFormat {
            reason: reason.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensure `GenerationError` can be converted into `Error`.
    #[test]
    fn generation_error_conversion() {
        let gen_err = GenerationError::IdentityCreationFailed;
        let err: Error = gen_err.into();
        assert!(matches!(err, Error::Generation(_)));
    }

    /// Ensure `ValidationError` can be converted into `Error`.
    #[test]
    fn validation_error_conversion() {
        let val_err = ValidationError::invalid_public_key("test");
        let err: Error = val_err.into();
        assert!(matches!(err, Error::Validation(_)));
    }

    /// Test the `Display` implementation of `GenerationError`.
    #[test]
    fn generation_error_display() {
        let e = GenerationError::IdentityCreationFailed;
        assert_eq!(format!("{}", e), "Identity generation failed");
    }

    /// Test the `Display` implementation of `ValidationError` for public key.
    #[test]
    fn validation_error_display_public() {
        let e = ValidationError::invalid_public_key("too short");
        assert_eq!(format!("{}", e), "Invalid public key format: too short");
    }

    /// Test the `Display` implementation of `ValidationError` for secret key.
    #[test]
    fn validation_error_display_secret() {
        let e = ValidationError::invalid_secret_key("empty");
        assert_eq!(format!("{}", e), "Invalid secret key format: empty");
    }

    /// Test that the `Result` alias works properly.
    #[test]
    fn result_type_alias() {
        fn returns_ok() -> Result<()> {
            Ok(())
        }
        fn returns_err() -> Result<()> {
            Err(Error::Generation(GenerationError::IdentityCreationFailed))
        }
        assert!(returns_ok().is_ok());
        assert!(returns_err().is_err());
    }
}
