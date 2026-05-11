//! Age key pair.
//!
//! This module defines the [`KeyPair`] type, which bundles a validated
//! [`PublicKey`] with its corresponding [`SecretKey`]. Key pairs are created
//! via the [`build_keypair`](crate::build_keypair) function, which handles
//! generation and validation, ensuring that the two keys are mathematically
//! related and conform to the age specification.

use crate::public_key::PublicKey;
use crate::secret_key::SecretKey;

/// An age X25519 key pair consisting of a public key and a secret key.
///
/// `KeyPair` is the central type of this crate. It is produced by
/// [`build_keypair`](crate::build_keypair) and provides access to both the
/// public identity (safe to share) and the secret identity (must be kept
/// confidential).
///
/// # Fields
///
/// * `public: PublicKey` – The public key. It is guaranteed to start with
///   `"age1"` and can be safely displayed, cloned, and shared.
/// * `secret: SecretKey` – The secret key. It is automatically zeroized when
///   dropped, and its `Display` and `Debug` implementations redact the actual
///   key material.
///
/// # Creation
///
/// `KeyPair` cannot be constructed directly from outside the crate because
/// its constructor is `pub(crate)`. This ensures that all key pairs are
/// produced by [`build_keypair`], which properly generates a fresh identity
/// and validates both keys.
///
/// # Examples
///
/// ```rust
/// use age_setup::build_keypair;
///
/// let kp = build_keypair()?;
/// println!("Public key: {}", kp.public);
/// // Secret key is redacted when printed:
/// println!("Secret key: {}", kp.secret);   // prints [REDACTED]
/// # Ok::<(), age_setup::Error>(())
/// ```
#[derive(Debug)]
pub struct KeyPair {
    /// The public half of the key pair.
    pub public: PublicKey,
    /// The secret half of the key pair (zeroized on drop).
    pub secret: SecretKey,
}

impl KeyPair {
    /// Creates a new `KeyPair` from an already validated public and secret key.
    ///
    /// This constructor is `pub(crate)` – only accessible within the crate.
    /// External users obtain a `KeyPair` exclusively through
    /// [`build_keypair`](crate::build_keypair), which performs the necessary
    /// generation and validation steps.
    pub(crate) fn new(public: PublicKey, secret: SecretKey) -> Self {
        Self { public, secret }
    }
}

#[cfg(test)]
mod tests {
    use crate::build_keypair;

    /// Verifies that a freshly generated key pair has fields that are
    /// accessible and conform to the expected format.
    #[test]
    fn generated_keypair_fields_are_valid() {
        let kp = build_keypair().unwrap();
        assert!(kp.public.expose().starts_with("age1"));
        assert!(kp.secret.expose_secret().starts_with("AGE-SECRET-KEY-1"));
    }

    /// Ensures that the public and secret keys of a newly generated
    /// `KeyPair` are non‑empty.
    #[test]
    fn generated_keypair_fields_are_non_empty() {
        let kp = build_keypair().unwrap();
        assert!(!kp.public.expose().is_empty());
        assert!(!kp.secret.expose_secret().is_empty());
    }

    /// The `Debug` representation of a `KeyPair` must not expose the
    /// secret key (it should be redacted by `SecretKey`'s `Debug` impl).
    #[test]
    fn debug_does_not_leak_secret() {
        let kp = build_keypair().unwrap();
        let debug_str = format!("{:?}", kp);
        // The debug output should contain the public key (it is safe) ...
        assert!(debug_str.contains(kp.public.expose()));
        // ... but NOT the raw secret key material.
        assert!(!debug_str.contains(kp.secret.expose_secret()));
        // It should mention the redacted marker instead.
        assert!(debug_str.contains("[REDACTED]"));
    }
}
