//! Keypair building API.

use crate::build::identity::create_identity;
use crate::build::recipient::extract_recipient;
use crate::errors::Result;
use crate::types::{KeyPair, PublicKey, SecretKey};
use age::secrecy::ExposeSecret;

/// Generates a new age X25519 keypair.
///
/// This function creates a random identity using `age::x25519::Identity::generate()`,
/// extracts the corresponding recipient (public key), and wraps them into a [`KeyPair`].
/// The public key is validated to start with "age1".
///
/// # Returns
/// Returns a [`Result`] containing a [`KeyPair`] on success, or an [`crate::Error`] if validation fails.
///
/// # Example
/// ```
/// # use age_setup::build_keypair;
/// let kp = build_keypair().unwrap();
/// assert!(kp.public.expose().starts_with("age1"));
/// ```
pub fn build_keypair() -> Result<KeyPair> {
    let identity = create_identity()?;
    let recipient = extract_recipient(&identity);
    let public_raw = recipient.to_string();
    let secret_raw = identity.to_string().expose_secret().to_string();
    let public = PublicKey::new(public_raw)?;
    let secret = SecretKey::new(secret_raw);
    Ok(KeyPair::new(public, secret))
}
