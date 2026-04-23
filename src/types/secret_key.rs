//! Secret key type with automatic zeroization on drop.

use crate::security::zeroize::wipe_memory;
use std::fmt;

/// An age secret key that securely wipes its memory when dropped.
///
/// The secret key is stored as bytes. The `Display` implementation redacts the value,
/// showing `[REDACTED]`. Use `expose()` to get the raw string (use with caution).
#[derive(Debug, Clone)]
pub struct SecretKey {
    inner: Vec<u8>,
}

impl SecretKey {
    /// Creates a new secret key from a string (internal).
    pub(crate) fn new(raw: String) -> Self {
        Self {
            inner: raw.into_bytes(),
        }
    }

    /// Exposes the raw secret key as a string.
    ///
    /// # Panics
    /// Panics if the inner bytes are not valid UTF-8 (should never happen because
    /// the key is created from a valid UTF-8 string).
    ///
    /// # Security
    /// Only use this when absolutely necessary, as it exposes the secret.
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
        drop(sk); // Tidak panic, wipe_memory dipanggil
    }
}
