use crate::errors::{Error, Result, ValidationError};
pub(crate) fn validate_age_prefix(key: &str) -> Result<()> {
    if key.is_empty() {
        return Err(Error::from(ValidationError::invalid_public_key(
            "Key is empty",
        )));
    }
    if !key.starts_with("age1") {
        return Err(Error::from(ValidationError::invalid_public_key(format!(
            "Key must start with 'age1', got: {}",
            &key[..key.len().min(10)]
        ))));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::Error;
    #[test]
    fn test_validate_age_prefix_empty() {
        let result = validate_age_prefix("");
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(e) => {
                let msg = format!("{}", e);
                assert!(msg.contains("Key is empty"));
            }
            _ => panic!(),
        }
    }
    #[test]
    fn test_validate_age_prefix_wrong_prefix() {
        let result = validate_age_prefix("xyz123");
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(e) => {
                let msg = format!("{}", e);
                assert!(msg.contains("must start with 'age1'"));
                assert!(msg.contains("xyz123"));
            }
            _ => panic!(),
        }
    }
    #[test]
    fn test_validate_age_prefix_short_prefix() {
        let result = validate_age_prefix("age");
        assert!(result.is_err());
    }
    #[test]
    fn test_validate_age_prefix_valid() {
        let result = validate_age_prefix("age1abcdef");
        assert!(result.is_ok());
    }
}
