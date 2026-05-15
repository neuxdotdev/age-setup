use crate::public_key::PublicKey;
use crate::secret_key::SecretKey;

/// A cryptographic key pair for the age protocol.
///
/// Contains a [`PublicKey`] and a [`SecretKey`], both guaranteed to be valid
/// age keys. The [`Debug`] implementation redacts the secret key value while
/// displaying the public key in full.
///
/// # Obtaining a KeyPair
///
/// Use [`build_keypair`](crate::build_keypair) to generate a fresh key pair:
///
/// ```no_run
/// use age_setup::build_keypair;
///
/// let kp = build_keypair()?;
/// println!("Public: {}", kp.public);
/// # Ok::<(), age_setup::Error>(())
/// ```
///
/// # Debug Safety
///
/// The debug representation does **not** leak the secret key:
///
/// ```rust
/// use age_setup::build_keypair;
///
/// let kp = build_keypair()?;
/// let debug_str = format!("{:?}", kp);
/// assert!(debug_str.contains(kp.public.expose()));
/// assert!(!debug_str.contains(kp.secret.expose_secret()));
/// # Ok::<(), age_setup::Error>(())
/// ```
///
/// # See Also
///
/// * [`build_keypair`](crate::build_keypair) – Generates a new `KeyPair`.
/// * [`SecretKey`](crate::SecretKey) – Zeroizing secret key wrapper.
/// * [`PublicKey`](crate::PublicKey) – Validated public key wrapper.
#[derive(Debug)]
pub struct KeyPair {
    /// The public key component.
    pub public: PublicKey,
    /// The secret key component (redacted in debug output).
    pub secret: SecretKey,
}

impl KeyPair {
    /// Creates a new `KeyPair` from existing keys.
    ///
    /// This constructor is crate-internal. External users should call
    /// [`build_keypair`](crate::build_keypair) to generate a new pair.
    ///
    /// # Parameters
    ///
    /// * `public` – A validated [`PublicKey`].
    /// * `secret` – A validated [`SecretKey`].
    pub(crate) fn new(public: PublicKey, secret: SecretKey) -> Self {
        Self { public, secret }
    }
}

#[cfg(test)]
mod tests {
    use crate::build_keypair;

    #[test]
    fn generated_keypair_fields_are_valid() {
        let kp = build_keypair().unwrap();
        assert!(kp.public.expose().starts_with("age1"));
        assert!(kp.secret.expose_secret().starts_with("AGE-SECRET-KEY-1"));
    }

    #[test]
    fn generated_keypair_fields_are_non_empty() {
        let kp = build_keypair().unwrap();
        assert!(!kp.public.expose().is_empty());
        assert!(!kp.secret.expose_secret().is_empty());
    }

    #[test]
    fn debug_does_not_leak_secret() {
        let kp = build_keypair().unwrap();
        let debug_str = format!("{:?}", kp);
        assert!(debug_str.contains(kp.public.expose()));
        assert!(!debug_str.contains(kp.secret.expose_secret()));
        assert!(debug_str.contains("[REDACTED]"));
    }
}
