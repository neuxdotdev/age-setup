//! Key generation error types.

/// Errors that occur during identity generation.
#[derive(Debug, thiserror::Error)]
pub enum GenerationError {
    /// Failed to generate an age identity (internal library issue).
    #[error("Age identity generation failed: internal library error")]
    IdentityCreationFailed,
}
