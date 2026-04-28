#[derive(Debug, thiserror::Error)]
pub enum GenerationError {
    #[error("Age identity generation failed: internal library error")]
    IdentityCreationFailed,
}