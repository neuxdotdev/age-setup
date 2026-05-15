use crate::errors::Result;
use crate::keypair::KeyPair;
use crate::public_key::PublicKey;
use crate::secret_key::SecretKey;
use age::secrecy::ExposeSecret;
use age::x25519::Identity;

/// Generates a new age X25519 key pair.
///
/// Creates a fresh [`Identity`] using the age library, extracts the public
/// and secret components, validates both, and returns them as a [`KeyPair`].
///
/// # Performance
///
/// This operation involves cryptographic key generation and is relatively
/// expensive. The result is marked `#[must_use]` to discourage discarding
/// generated keys accidentally. Consider caching the `KeyPair` when possible.
///
/// # Errors
///
/// Returns [`Error::Generation`](crate::Error::Generation) if the underlying
/// age identity generation fails.
///
/// # Examples
///
/// ```no_run
/// use age_setup::build_keypair;
///
/// let kp = build_keypair()?;
/// println!("Public key: {}", kp.public);
/// // Secret key is not displayed: the Debug impl redacts it.
/// println!("KeyPair: {:?}", kp);
/// # Ok::<(), age_setup::Error>(())
/// ```
///
/// # See Also
///
/// * [`KeyPair`](crate::KeyPair) – Container for the generated keys.
/// * [`Identity::generate`](age::x25519::Identity::generate) – Underlying generation.
#[must_use = "generating a key pair is an expensive operation; consider reusing the result"]
pub fn build_keypair() -> Result<KeyPair> {
    let identity = Identity::generate();
    let recipient = identity.to_public();
    let public_raw = recipient.to_string();
    let secret_raw = identity.to_string().expose_secret().to_string();
    let public = PublicKey::new(public_raw)?;
    let secret = SecretKey::new(secret_raw)?;
    Ok(KeyPair::new(public, secret))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_keypair_has_valid_format() {
        let kp = build_keypair().unwrap();
        assert!(kp.public.expose().starts_with("age1"));
        assert!(kp.secret.expose_secret().starts_with("AGE-SECRET-KEY-1"));
    }

    #[test]
    fn generated_keypairs_are_random() {
        let kp1 = build_keypair().unwrap();
        let kp2 = build_keypair().unwrap();
        assert_ne!(kp1.public.expose(), kp2.public.expose());
        assert_ne!(kp1.secret.expose_secret(), kp2.secret.expose_secret());
    }

    #[test]
    fn secret_is_not_leaked() {
        let kp = build_keypair().unwrap();
        let debug = format!("{:?}", kp);
        assert!(!debug.contains(kp.secret.expose_secret()));
    }

    #[test]
    fn keys_have_body_after_prefix() {
        let kp = build_keypair().unwrap();
        assert!(kp.public.expose().len() > "age1".len());
        assert!(kp.secret.expose_secret().len() > "AGE-SECRET-KEY-1".len());
    }
}
