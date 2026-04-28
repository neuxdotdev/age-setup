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
    fn empty() {
        let e = validate_age_prefix("").unwrap_err();
        assert!(matches!(e, Error::Validation(_)));
    }
    #[test]
    fn wrong_prefix() {
        let e = validate_age_prefix("xxx").unwrap_err();
        assert!(format!("{}", e).contains("must start with 'age1'"));
    }
    #[test]
    fn valid() {
        assert!(validate_age_prefix("age1abc").is_ok());
    }
}
