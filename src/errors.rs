use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
    #[error("Key generation failed: {0}")]
    Generation(#[from] GenerationError),
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationError),
    #[error("Security operation failed: {0}")]
    Security(#[from] SecurityError),
}
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Error)]
pub enum GenerationError {
    #[error("Identity generation failed")]
    IdentityCreationFailed,
}
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid public key format: {reason}")]
    InvalidPublicKeyFormat { reason: String },
    #[error("Invalid secret key format: {reason}")]
    InvalidSecretKeyFormat { reason: String },
}
impl ValidationError {
    pub(crate) fn invalid_public_key(reason: impl Into<String>) -> Self {
        Self::InvalidPublicKeyFormat {
            reason: reason.into(),
        }
    }
    pub(crate) fn invalid_secret_key(reason: impl Into<String>) -> Self {
        Self::InvalidSecretKeyFormat {
            reason: reason.into(),
        }
    }
}
#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("Memory wipe failed")]
    MemoryWipeFailed,
}
