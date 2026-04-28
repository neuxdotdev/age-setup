use crate::security::zeroize::wipe_memory;
use std::fmt;
#[derive(Debug, Clone)]
pub struct SecretKey {
    inner: Vec<u8>,
}
impl SecretKey {
    pub(crate) fn new(raw: String) -> Self {
        Self {
            inner: raw.into_bytes(),
        }
    }
    #[must_use]
    pub fn expose(&self) -> &str {
        std::str::from_utf8(&self.inner).expect("SecretKey inner buffer must be valid UTF-8")
    }
}
impl Drop for SecretKey {
    fn drop(&mut self) {
        let _ = wipe_memory(&mut self.inner);
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
    fn test_secret_key_expose() {
        let sk = SecretKey::new("test".to_string());
        assert_eq!(sk.expose(), "test");
    }
    #[test]
    fn test_secret_key_display() {
        let sk = SecretKey::new("test".to_string());
        assert_eq!(format!("{}", sk), "[REDACTED]");
    }
    #[test]
    fn test_secret_key_clone() {
        let sk1 = SecretKey::new("secret".to_string());
        let sk2 = sk1.clone();
        assert_eq!(sk1.expose(), sk2.expose());
    }
    #[test]
    fn test_secret_key_drop_calls_wipe() {
        let sk = SecretKey::new("secret".to_string());
        drop(sk);
    }
}
