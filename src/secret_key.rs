use crate::errors::{Error, Result, ValidationError};
use std::fmt;
use zeroize::Zeroizing;

/// A zeroizing age secret key.
///
/// Wraps a secret key string inside [`Zeroizing`], guaranteeing that the
/// underlying memory is cleared when the `SecretKey` is dropped. The key must
/// start with the standard age secret key prefix `"AGE-SECRET-KEY-1"`.
///
/// The [`Debug`] and [`Display`] implementations intentionally redact the
/// actual value to prevent accidental leakage in logs or error messages.
///
/// # Invariants
///
/// * The inner string is never empty.
/// * The inner string always starts with `"AGE-SECRET-KEY-1"`.
/// * Memory is zeroized on drop via [`Zeroizing`].
///
/// # Examples
///
/// ```rust
/// use age_setup::SecretKey;
///
/// let sk = SecretKey::new("AGE-SECRET-KEY-1ABCDEF".into())?;
/// // The debug representation hides the actual value.
/// assert_eq!(format!("{:?}", sk), "SecretKey { value: \"[REDACTED]\" }");
/// # Ok::<(), age_setup::Error>(())
/// ```
///
/// # See Also
///
/// * [`PublicKey`](crate::PublicKey) – The corresponding public key wrapper.
/// * [`KeyPair`](crate::KeyPair) – Container holding both keys.
#[derive(Clone)]
pub struct SecretKey {
    inner: Zeroizing<String>,
}

impl SecretKey {
    /// Creates a new `SecretKey` after validating the age secret key prefix.
    ///
    /// The provided `raw` string must start with `"AGE-SECRET-KEY-1"` and must
    /// not be empty.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Validation`](crate::Error::Validation) with
    /// [`ValidationError::InvalidSecretKeyFormat`](crate::ValidationError::InvalidSecretKeyFormat)
    /// if the key is empty or does not start with the required prefix.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use age_setup::SecretKey;
    ///
    /// assert!(SecretKey::new("AGE-SECRET-KEY-1VALID".into()).is_ok());
    /// assert!(SecretKey::new("bad".into()).is_err());
    /// assert!(SecretKey::new("".into()).is_err());
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

    /// Returns a reference to the underlying secret key string.
    ///
    /// Use this only when the secret must be passed to another API. Prefer
    /// to keep the `SecretKey` in scope and avoid unnecessary copies.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use age_setup::SecretKey;
    ///
    /// let sk = SecretKey::new("AGE-SECRET-KEY-1SECRET".into())?;
    /// assert_eq!(sk.expose_secret(), "AGE-SECRET-KEY-1SECRET");
    /// # Ok::<(), age_setup::Error>(())
    /// ```
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
