use crate::errors::Result;
use std::fmt;

/// A validated age public key.
///
/// Wraps a string that is guaranteed to start with `"age1"` (the standard age
/// recipient prefix). Construction fails if the provided string does not meet
/// this requirement.
///
/// # Invariants
///
/// * The inner string is never empty.
/// * The inner string always starts with `"age1"`.
///
/// # Examples
///
/// ```rust
/// use age_setup::PublicKey;
///
/// let pk = PublicKey::new("age1abcdef".into())?;
/// assert_eq!(pk.expose(), "age1abcdef");
/// # Ok::<(), age_setup::Error>(())
/// ```
///
/// Invalid input:
///
/// ```compile_fail
/// use age_setup::PublicKey;
///
/// // This will not compile because `new` returns a Result.
/// let pk: PublicKey = PublicKey::new("bad".into());
/// ```
#[derive(Debug, Clone)]
pub struct PublicKey(String);

impl PublicKey {
    /// Creates a new `PublicKey` after validating the age prefix.
    ///
    /// The provided `raw` string must start with `"age1"` and must not be empty.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Validation`](crate::Error::Validation) with
    /// [`ValidationError::InvalidPublicKeyFormat`](crate::ValidationError::InvalidPublicKeyFormat)
    /// if the key is empty or does not start with `"age1"`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use age_setup::PublicKey;
    ///
    /// assert!(PublicKey::new("age1valid".into()).is_ok());
    /// assert!(PublicKey::new("invalid".into()).is_err());
    /// assert!(PublicKey::new("".into()).is_err());
    /// ```
    pub fn new(raw: String) -> Result<Self> {
        crate::validation::validate_age_prefix(&raw)?;
        Ok(Self(raw))
    }

    /// Returns a reference to the underlying public key string.
    ///
    /// This intentionally does **not** implement [`AsRef<str>`] directly
    /// (though it is provided via a separate impl) to discourage accidental
    /// logging. Use this method explicitly when you need the raw value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use age_setup::PublicKey;
    ///
    /// let pk = PublicKey::new("age1secret".into())?;
    /// assert_eq!(pk.expose(), "age1secret");
    /// # Ok::<(), age_setup::Error>(())
    /// ```
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
