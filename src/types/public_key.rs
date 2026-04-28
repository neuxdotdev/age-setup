use crate::errors::Result;
use std::fmt;
use super::validation::validate_age_prefix;
#[derive(Debug, Clone)]
pub struct PublicKey(String);
impl PublicKey {
    pub fn new(raw: String) -> Result<Self> {
        validate_age_prefix(&raw)?;
        Ok(Self(raw))
    }
    #[must_use]
    pub fn expose(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl AsRef<str> for PublicKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_public_key() {
        let key = "age1validkey";
        let pk = PublicKey::new(key.to_string()).unwrap();
        assert_eq!(pk.expose(), key);
    }
    #[test]
    fn invalid_public_key_prefix() {
        let key = "invalidkey";
        let result = PublicKey::new(key.to_string());
        assert!(result.is_err());
    }
    #[test]
    fn empty_public_key() {
        let result = PublicKey::new("".to_string());
        assert!(result.is_err());
    }
}