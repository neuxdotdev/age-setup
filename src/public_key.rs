use crate::errors::Result;
use std::fmt;
#[derive(Debug, Clone)]
pub struct PublicKey(String);
impl PublicKey {
    pub fn new(raw: String) -> Result<Self> {
        crate::validation::validate_age_prefix(&raw)?;
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
    fn valid() {
        let pk = PublicKey::new("age1test".into()).unwrap();
        assert_eq!(pk.expose(), "age1test");
    }
    #[test]
    fn invalid() {
        assert!(PublicKey::new("bad".into()).is_err());
    }
}
