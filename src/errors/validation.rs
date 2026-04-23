#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Invalid public key format: {reason}")]
    InvalidPublicKeyFormat { reason: String },
}
impl ValidationError {
    pub(crate) fn invalid_public_key(reason: impl Into<String>) -> Self {
        Self::InvalidPublicKeyFormat {
            reason: reason.into(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::invalid_public_key("test reason");
        let msg = format!("{}", err);
        assert_eq!(msg, "Invalid public key format: test reason");
    }
}
