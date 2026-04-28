#[derive(Debug, thiserror::Error)]
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_validation_error() {
        let err = ValidationError::invalid_public_key("test");
        assert_eq!(format!("{}", err), "Invalid public key format: test");
    }
}