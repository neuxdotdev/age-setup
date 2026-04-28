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
    fn valid() {
        let sk = SecretKey::new("AGE-SECRET-KEY-1TEST".into()).unwrap();
        assert_eq!(sk.expose_secret(), "AGE-SECRET-KEY-1TEST");
    }
    #[test]
    fn debug_redacted() {
        let sk = SecretKey::new("AGE-SECRET-KEY-1TEST".into()).unwrap();
        let d = format!("{:?}", sk);
        assert!(d.contains("[REDACTED]"));
        assert!(!d.contains("TEST"));
    }
}
