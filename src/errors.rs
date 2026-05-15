use thiserror::Error;

/// Top-level error type for the `age-authenticator` library.
///
/// All fallible operations return this error type, which categorizes failures
/// into two groups:
///
/// * [`Generation`](Error::Generation) – Errors during key pair generation.
/// * [`Validation`](Error::Validation) – Errors during key format validation.
///
/// # Conversions
///
/// Both [`GenerationError`] and [`ValidationError`] automatically convert into
/// `Error` via their respective [`From`] implementations, allowing the `?`
/// operator to be used seamlessly.
///
/// # Examples
///
/// ```rust
/// use age_setup::Error;
///
/// let err = Error::Generation(
///     age_setup::GenerationError::IdentityCreationFailed,
/// );
/// assert!(matches!(err, Error::Generation(_)));
/// ```
#[derive(Debug, Error)]
pub enum Error {
    /// A key generation operation failed.
    #[error("Key generation failed: {0}")]
    Generation(#[from] GenerationError),

    /// A validation check failed.
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationError),
}

/// Convenience type alias for `Result<T, Error>`.
///
/// # Examples
///
/// ```rust
/// use age_setup::Result;
///
/// fn fallible() -> Result<()> {
///     Ok(())
/// }
/// assert!(fallible().is_ok());
/// ```
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that occur during key pair generation.
///
/// # Variants
///
/// * [`IdentityCreationFailed`](GenerationError::IdentityCreationFailed) –
///   The underlying age identity could not be generated.
#[derive(Debug, Error)]
pub enum GenerationError {
    /// The age X25519 identity generation failed.
    #[error("Identity generation failed")]
    IdentityCreationFailed,
}

/// Errors that occur during key format validation.
///
/// # Variants
///
/// * [`InvalidPublicKeyFormat`](ValidationError::InvalidPublicKeyFormat) –
///   A public key did not meet the required format.
/// * [`InvalidSecretKeyFormat`](ValidationError::InvalidSecretKeyFormat) –
///   A secret key did not meet the required format.
#[derive(Debug, Error)]
pub enum ValidationError {
    /// The public key format is invalid.
    #[error("Invalid public key format: {reason}")]
    InvalidPublicKeyFormat {
        /// Human-readable reason for the validation failure.
        reason: String,
    },

    /// The secret key format is invalid.
    #[error("Invalid secret key format: {reason}")]
    InvalidSecretKeyFormat {
        /// Human-readable reason for the validation failure.
        reason: String,
    },
}

impl ValidationError {
    /// Creates a public key validation error with the given reason.
    ///
    /// This is a crate-internal convenience constructor.
    pub(crate) fn invalid_public_key(reason: impl Into<String>) -> Self {
        Self::InvalidPublicKeyFormat {
            reason: reason.into(),
        }
    }

    /// Creates a secret key validation error with the given reason.
    ///
    /// This is a crate-internal convenience constructor.
    pub(crate) fn invalid_secret_key(reason: impl Into<String>) -> Self {
        Self::InvalidSecretKeyFormat {
            reason: reason.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_error_conversion() {
        let gen_err = GenerationError::IdentityCreationFailed;
        let err: Error = gen_err.into();
        assert!(matches!(err, Error::Generation(_)));
    }

    #[test]
    fn validation_error_conversion() {
        let val_err = ValidationError::invalid_public_key("test");
        let err: Error = val_err.into();
        assert!(matches!(err, Error::Validation(_)));
    }

    #[test]
    fn generation_error_display() {
        let e = GenerationError::IdentityCreationFailed;
        assert_eq!(format!("{}", e), "Identity generation failed");
    }

    #[test]
    fn validation_error_display_public() {
        let e = ValidationError::invalid_public_key("too short");
        assert_eq!(format!("{}", e), "Invalid public key format: too short");
    }

    #[test]
    fn validation_error_display_secret() {
        let e = ValidationError::invalid_secret_key("empty");
        assert_eq!(format!("{}", e), "Invalid secret key format: empty");
    }

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
