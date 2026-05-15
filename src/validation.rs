use crate::errors::{Error, Result, ValidationError};

/// Validates that a string has the standard age public key prefix.
///
/// Checks that `key` is non-empty and starts with `"age1"`. This function
/// is used internally by [`PublicKey::new`](crate::PublicKey::new).
///
/// # Errors
///
/// Returns [`Error::Validation`] with:
///
/// * [`ValidationError::InvalidPublicKeyFormat`](crate::ValidationError::InvalidPublicKeyFormat)
///   if the key is empty.
/// * [`ValidationError::InvalidPublicKeyFormat`](crate::ValidationError::InvalidPublicKeyFormat)
///   if the key does not start with `"age1"`. The error message includes the
///   first 10 characters of the offending input.
///
/// # Examples
///
/// ```rust
/// use age_setup::validation::validate_age_prefix;
///
/// assert!(validate_age_prefix("age1abc").is_ok());
/// assert!(validate_age_prefix("bad").is_err());
/// assert!(validate_age_prefix("").is_err());
/// ```
pub fn validate_age_prefix(key: &str) -> Result<()> {
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
        let msg = format!("{}", e);
        assert!(msg.contains("must start with 'age1'"));
    }

    #[test]
    fn valid() {
        assert!(validate_age_prefix("age1abc").is_ok());
    }
}
