//! Age secret key type.
//!
//! This module provides the [`SecretKey`] type, a validated, memory‑safe
//! wrapper around an age secret key string (starting with `AGE-SECRET-KEY-1`).
//! The type is the private half of an [`KeyPair`](crate::KeyPair) and is
//! designed to keep the secret confidential:
//!
//! - It uses [`zeroize::Zeroizing`] internally to overwrite memory on drop.
//! - Its [`Display`] and [`Debug`] implementations intentionally redact the
//!   key material, printing `[REDACTED]` instead.
//!
//! # Accessing the secret
//!
//! To obtain the raw key string (e.g., for writing to a file or passing to
//! an age encryption function), call [`expose_secret`](SecretKey::expose_secret).
//! Do so **only when necessary** and ensure the returned reference is not
//! copied, logged, or leaked accidentally.

use crate::errors::{Error, Result, ValidationError};
use std::fmt;
use zeroize::Zeroizing;

/// A validated age secret key protected by memory zeroization.
///
/// `SecretKey` wraps the raw key string inside [`Zeroizing`], which guarantees
/// that the memory is securely erased when the value is dropped. This prevents
/// secrets from lingering in memory dumps or swap files.
///
/// # Validation
///
/// The key is validated at construction time via [`new`](SecretKey::new):
/// - It must be non‑empty.
/// - It must start with the string `AGE-SECRET-KEY-1` (case‑sensitive).
///
/// # Security properties
///
/// - **Redacted display** – `Display` and `Debug` print `[REDACTED]`, never
///   the actual key.
/// - **Zeroization on drop** – memory is overwritten with zeros when the
///   `SecretKey` (or any clone) is dropped.
/// - **Cloneable** – cloning creates a new independent `Zeroizing` copy that
///   is also zeroized separately.
///
/// # Examples
///
/// ```rust
/// use age_setup::SecretKey;
///
/// let sk = SecretKey::new("AGE-SECRET-KEY-1mytestkey".into())?;
/// println!("{}", sk);                       // prints: [REDACTED]
/// println!("{:?}", sk);                     // prints: SecretKey { ... [REDACTED] ... }
/// let raw = sk.expose_secret();             // careful: raw secret exposed
/// # Ok::<(), age_setup::Error>(())
/// ```
#[derive(Clone)]
pub struct SecretKey {
    inner: Zeroizing<String>,
}

impl SecretKey {
    /// Creates a new `SecretKey` after validating the raw string.
    ///
    /// # Validation checks
    ///
    /// 1. The key must not be empty.
    /// 2. The key must start with `"AGE-SECRET-KEY-1"`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Validation`](crate::Error::Validation) with a
    /// descriptive reason if any check fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use age_setup::SecretKey;
    /// let valid = SecretKey::new("AGE-SECRET-KEY-1abc".into()).unwrap();
    ///
    /// let empty = SecretKey::new("".into());
    /// assert!(empty.is_err());
    ///
    /// let wrong_prefix = SecretKey::new("ssh-rsa ...".into());
    /// assert!(wrong_prefix.is_err());
    /// ```
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

    /// Exposes the raw secret key string.
    ///
    /// ⚠️ **Security Warning** – this method returns the actual secret material
    /// as a `&str`. Only use it when absolutely necessary (e.g., to pass the
    /// key to an age decryption function or to write it to a securely
    /// permissioned file). Avoid logging, printing, or storing the returned
    /// string in an unsecured location.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use age_setup::SecretKey;
    /// let sk = SecretKey::new("AGE-SECRET-KEY-1test".into()).unwrap();
    /// let raw = sk.expose_secret();
    /// assert_eq!(raw, "AGE-SECRET-KEY-1test");
    /// ```
    #[must_use]
    pub fn expose_secret(&self) -> &str {
        &self.inner
    }
}

/// The `Debug` implementation records the secret value as `[REDACTED]` to
/// prevent accidental leakage through debug output.
impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SecretKey")
            .field("value", &"[REDACTED]")
            .finish()
    }
}

/// The `Display` implementation always writes `[REDACTED]`, never the actual
/// key. Use [`expose_secret`](SecretKey::expose_secret) if you need the raw
/// string.
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

    /// Confirm that `Debug` and `Display` never contain the secret.
    #[test]
    fn debug_redacted() {
        let sk = SecretKey::new("AGE-SECRET-KEY-1TEST".into()).unwrap();
        let d = format!("{:?}", sk);
        assert!(d.contains("[REDACTED]"));
        assert!(!d.contains("TEST"));
    }
}
