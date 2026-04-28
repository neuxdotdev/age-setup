use crate::errors::{Error, Result, ValidationError};
use std::fmt;
use zeroize::Zeroizing;
#[derive(Clone)]
pub struct SecretKey {
    inner: Zeroizing<String>,
}
impl SecretKey {
    pub fn new(raw: String) -> Result<Self> {
        if raw.is_empty() {
            return Err(Error::from(ValidationError::invalid_secret_key(
                "Secret key is empty",
            )));
        }
        if !raw.starts_with("AGE-SECRET-KEY-1") {
            return Err(Error::from(ValidationError::invalid_secret_key(
                "Secret key must start with 'AGE-SECRET-KEY-1'",
            )));
        }
        Ok(Self {
            inner: Zeroizing::new(raw),
        })
    }
    #[must_use]
    pub fn expose_secret(&self) -> &str {
        &self.inner
    }
}
impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SecretKey")
            .field("value", &"[REDACTED]")
            .finish()
    }
}
impl fmt::Display for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED]")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_secret_key() {
        let key = "AGE-SECRET-KEY-1TESTKEY";
        let sk = SecretKey::new(key.to_string()).unwrap();
        assert_eq!(sk.expose_secret(), key);
    }
    #[test]
    fn invalid_secret_key_prefix() {
        let key = "INVALID-KEY";
        let result = SecretKey::new(key.to_string());
        assert!(result.is_err());
    }
    #[test]
    fn debug_redacted() {
        let key = "AGE-SECRET-KEY-1TESTKEY";
        let sk = SecretKey::new(key.to_string()).unwrap();
        let debug_output = format!("{:?}", sk);
        assert!(debug_output.contains("[REDACTED]"));
        assert!(!debug_output.contains(key));
    }
}