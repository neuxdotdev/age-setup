//! Lightweight validation for age public keys.
//!
//! This module provides a **non‑cryptographic** sanity check for age public key
//! strings. It is used internally (`pub(crate)`) to catch obvious programmer
//! mistakes early, before the key is actually used in cryptographic operations.

use crate::errors::{Error, Result, ValidationError};

/// Verifies that a string is non‑empty and starts with the age public key prefix `"age1"`.
///
/// This is a **lightweight, internal sanity check** that ensures a candidate public
/// key adheres to the most basic requirement of the [age specification]. It is called
/// before a raw string is wrapped inside [`PublicKey`].
///
/// # Checks performed
///
/// 1. **Non‑emptiness** – an empty string is rejected immediately.
/// 2. **Prefix** – the string must begin with `"age1"`. The comparison is
///    case‑sensitive, so `"AGE1..."` is rejected.
///
/// No further validation (length, Bech32 character set, checksum, or curve point)
/// is performed here. Full parsing is the responsibility of the upstream `age` crate.
///
/// # Why only a prefix check?
///
/// Parsing a real Bech32‑encoded age key is complex and already handled by the `age`
/// library. This minimal check catches common programmer errors (e.g., passing a
/// filename instead of a key) early and with a clear error message.
///
/// # Parameters
///
/// * `key` – A string slice (`&str`) representing the raw public key to validate.
///
/// # Returns
///
/// * `Ok(())` if the key is non‑empty and starts with `"age1"`.
/// * `Err(Error::Validation(ValidationError::InvalidPublicKeyFormat { ... }))` if
///   either condition is violated.
///
/// # Errors
///
/// | Condition | Error message |
/// |-----------|---------------|
/// | `key.is_empty()` | `"Key is empty"` |
/// | `key` does not start with `"age1"` | `"Key must start with 'age1', got: <prefix>…"` |
///
/// When the prefix is wrong, the error message includes the first few characters of
/// the supplied key (up to 10). The truncation is safe for short strings and will not
/// panic.
///
/// # Security note
///
/// **This function does not guarantee that the key is a valid age recipient.**
/// Malicious or malformed keys that still begin with `"age1"` will pass this check.
/// The actual cryptographic validation happens inside the `age` crate when the key is
/// used for encryption.
///
/// # Examples
///
/// ```rust,ignore
/// use age_setup::validation::validate_age_prefix;
///
/// // ✅ Valid keys
/// assert!(validate_age_prefix("age1abcdef").is_ok());
/// assert!(validate_age_prefix("age1").is_ok());   // minimal valid input
///
/// // ❌ Invalid keys
/// let empty = validate_age_prefix("");
/// assert!(empty.is_err());
///
/// let wrong = validate_age_prefix("ssh-rsa AAAA...");
/// let err_msg = wrong.unwrap_err().to_string();
/// assert_eq!(
///     err_msg,
///     "Key must start with 'age1', got: ssh-rsa AA"
/// );
/// ```
///
/// # See also
///
/// * [`PublicKey::new`](crate::public_key::PublicKey::new) – uses this validator.
/// * [age specification](https://github.com/FiloSottile/age) – formal documentation.
pub(crate) fn validate_age_prefix(key: &str) -> Result<()> {
    if key.is_empty() {
        return Err(Error::from(ValidationError::invalid_public_key(
            "Key is empty",
        )));
    }
    if !key.starts_with("age1") {
        return Err(Error::from(ValidationError::invalid_public_key(format!(
            "Key must start with 'age1', got: {}",
            &key[..key.len().min(10)]
        ))));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::Error;

    #[test]
    fn empty() {
        let e = validate_age_prefix("").unwrap_err();
        assert!(matches!(e, Error::Validation(_)));
    }

    #[test]
    fn wrong_prefix() {
        let e = validate_age_prefix("xxx").unwrap_err();
        let msg = format!("{}", e);
        assert!(msg.contains("must start with 'age1'"));
    }

    #[test]
    fn valid() {
        assert!(validate_age_prefix("age1abc").is_ok());
    }
}
