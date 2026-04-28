use crate::errors::Result;
use crate::types::validation::validate_age_prefix;
use std::fmt;
#[derive(Debug, Clone)]
pub struct PublicKey(String);
impl PublicKey {
    pub(crate) fn new(raw: String) -> Result<Self> {
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
    fn test_public_key_new_valid() {
        let pk = PublicKey::new("age1valid".to_string());
        assert!(pk.is_ok());
        assert_eq!(pk.unwrap().expose(), "age1valid");
    }
    #[test]
    fn test_public_key_new_invalid() {
        let pk = PublicKey::new("invalid".to_string());
        assert!(pk.is_err());
    }
    #[test]
    fn test_public_key_display() {
        let pk = PublicKey::new("age1test".to_string()).unwrap();
        assert_eq!(format!("{}", pk), "age1test");
    }
    #[test]
    fn test_public_key_asref() {
        let pk = PublicKey::new("age1asref".to_string()).unwrap();
        let s: &str = pk.as_ref();
        assert_eq!(s, "age1asref");
    }
    #[test]
    fn test_public_key_clone() {
        let pk1 = PublicKey::new("age1clone".to_string()).unwrap();
        let pk2 = pk1.clone();
        assert_eq!(pk1.expose(), pk2.expose());
    }
}
