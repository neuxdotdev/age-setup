//! Key pair generation.
//!
//! This module provides [`build_keypair`], the primary function for generating
//! a fresh X25519 key pair suitable for use with the `age` encryption tool.
//! The generation uses cryptographically secure randomness provided by the
//! operating system.

use crate::errors::Result;
use crate::keypair::KeyPair;
use crate::public_key::PublicKey;
use crate::secret_key::SecretKey;
use age::secrecy::ExposeSecret;
use age::x25519::Identity;

/// Generates a new age X25519 key pair.
///
/// This is the recommended way to create a [`KeyPair`]. It performs the following
/// steps **securely and automatically**:
///
/// 1. **Generate a fresh identity** using the `age` crate. The identity is
///    created with randomness sourced from the operating system's secure
///    random number generator (e.g. `/dev/urandom` on Linux, `getrandom`).
/// 2. **Extract the public and secret halves** from the identity. The secret
///    is temporarily exposed in a local variable which is immediately moved
///    into a [`SecretKey`] that guarantees zeroization on drop.
/// 3. **Validate both keys** – the public key is checked for the `"age1"`
///    prefix, the secret key for `"AGE-SECRET-KEY-1"`. This step acts as a
///    safety net; because the strings originate from the `age` crate they are
///    expected to be valid, but the check catches potential internal bugs
///    early.
/// 4. **Assemble the [`KeyPair`]** and return it to the caller.
///
/// The entire operation is **infallible** in practice – [`Identity::generate`]
/// does not return a `Result`. The only possible failures are the validation
/// steps, which would indicate a serious bug in the `age` library or this
/// crate.
///
/// # Returns
///
/// * `Ok(KeyPair)` – a newly generated key pair ready for encryption and
///   decryption.
/// * `Err(Error::Validation(...))` – if the generated key strings fail the
///   prefix checks (should never happen in practice).
///
/// # Security properties
///
/// * The secret key is automatically **zeroized** when the `KeyPair` (or its
///   `SecretKey` field) is dropped. No additional cleanup is needed.
/// * The function itself holds the raw secret string for a minimal amount of
///   time; it is moved directly into a [`Zeroizing`]-backed container.
/// * The randomness source is the same as the one used by the `age` CLI tool
///   and is suitable for production use.
///
/// # Examples
///
/// ```rust
/// use age_setup::build_keypair;
///
/// let kp = build_keypair()?;
/// assert!(kp.public.expose().starts_with("age1"));
/// assert!(kp.secret.expose_secret().starts_with("AGE-SECRET-KEY-1"));
/// # Ok::<(), age_setup::Error>(())
/// ```
#[must_use = "generating a key pair is an expensive operation; consider reusing the result"]
pub fn build_keypair() -> Result<KeyPair> {
    // 1. Generate a fresh X25519 identity
    let identity = Identity::generate();

    // 2. Obtain the public recipient string (age1...)
    let recipient = identity.to_public();
    let public_raw = recipient.to_string();

    // 3. Obtain the secret key string (AGE-SECRET-KEY-1...)
    //    `expose_secret()` returns a reference; we clone it into a new String.
    let secret_raw = identity.to_string().expose_secret().to_string();

    // 4. Validate and wrap in our secure types
    let public = PublicKey::new(public_raw)?;
    let secret = SecretKey::new(secret_raw)?;

    // 5. Return the key pair (secret_raw has been moved into `secret` and will be zeroized)
    Ok(KeyPair::new(public, secret))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A freshly generated key pair must pass our prefix checks.
    #[test]
    fn generated_keypair_has_valid_format() {
        let kp = build_keypair().unwrap();
        assert!(kp.public.expose().starts_with("age1"));
        assert!(kp.secret.expose_secret().starts_with("AGE-SECRET-KEY-1"));
    }

    /// Two consecutive calls must produce distinct key pairs
    /// (i.e., randomness is actually random).
    #[test]
    fn generated_keypairs_are_random() {
        let kp1 = build_keypair().unwrap();
        let kp2 = build_keypair().unwrap();
        assert_ne!(kp1.public.expose(), kp2.public.expose());
        assert_ne!(kp1.secret.expose_secret(), kp2.secret.expose_secret());
    }

    /// The secret key must be redacted in Debug output.
    #[test]
    fn secret_is_not_leaked() {
        let kp = build_keypair().unwrap();
        let debug = format!("{:?}", kp);
        assert!(!debug.contains(kp.secret.expose_secret()));
    }

    /// The public and secret keys must have more than just the prefix.
    #[test]
    fn keys_have_body_after_prefix() {
        let kp = build_keypair().unwrap();
        assert!(kp.public.expose().len() > "age1".len());
        assert!(kp.secret.expose_secret().len() > "AGE-SECRET-KEY-1".len());
    }
}
