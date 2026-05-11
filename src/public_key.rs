//! Age public key type.
//!
//! This module provides the [`PublicKey`] type, a validated wrapper around a
//! string that is guaranteed to start with the age public key prefix `"age1"`.
//! The type is the public half of an [`KeyPair`](crate::KeyPair) and can be
//! freely shared, displayed, and cloned.

use crate::errors::Result;
use std::fmt;

/// A validated age public key.
///
/// `PublicKey` is a thin wrapper around a [`String`] that ensures the contained
/// key starts with `"age1"`. The validation is performed at construction time
/// via [`new`](PublicKey::new), which delegates to
/// [`validate_age_prefix`](crate::validation::validate_age_prefix).
///
/// # Security
///
/// While this type guarantees the `"age1"` prefix, it does **not** perform
/// full Bech32 decoding or curve validation. The actual cryptographic checks
/// are left to the `age` crate when the key is used for encryption.
///
/// # Examples
///
/// ```rust
/// use age_setup::PublicKey;
///
/// let pk = PublicKey::new("age1mykey".into())?;
/// println!("Public key: {}", pk);          // uses Display
/// println!("Exposed value: {}", pk.expose());
/// # Ok::<(), age_setup::Error>(())
/// ```
#[derive(Debug, Clone)]
pub struct PublicKey(String);

impl PublicKey {
    /// Creates a new `PublicKey` after validating the raw string.
    ///
    /// The string must be non‑empty and start with `"age1"` (case‑sensitive).
    /// Validation is performed by [`validate_age_prefix`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::Validation`](crate::Error::Validation) if the key
    /// does not meet the prefix requirement.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use age_setup::PublicKey;
    /// let valid = PublicKey::new("age1abcdef".into()).unwrap();
    /// assert_eq!(valid.expose(), "age1abcdef");
    ///
    /// let invalid = PublicKey::new("not-a-key".into());
    /// assert!(invalid.is_err());
    /// ```
    pub fn new(raw: String) -> Result<Self> {
        crate::validation::validate_age_prefix(&raw)?;
        Ok(Self(raw))
    }

    /// Returns the raw string representation of the public key.
    ///
    /// The returned `&str` is safe to display, share, or use as an age
    /// recipient.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use age_setup::PublicKey;
    /// let pk = PublicKey::new("age1test".into()).unwrap();
    /// assert_eq!(pk.expose(), "age1test");
    /// ```
    #[must_use]
    pub fn expose(&self) -> &str {
        &self.0
    }
}

/// The public key is printed in the format `age1...`.
impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Allows `PublicKey` to be used where a `&str` is expected.
///
/// ```rust
/// # use age_setup::PublicKey;
/// fn print_key(key: &impl AsRef<str>) {
///     println!("{}", key.as_ref());
/// }
/// let pk = PublicKey::new("age1foo".into()).unwrap();
/// print_key(&pk);
/// ```
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
