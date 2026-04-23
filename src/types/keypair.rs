//! Keypair structure combining public and secret keys.

use crate::types::{PublicKey, SecretKey};

/// A keypair consisting of an age public key and its corresponding secret key.
///
/// This is the main output of [`build_keypair`](crate::build_keypair).
/// Both fields are public for direct access.
#[derive(Debug)]
pub struct KeyPair {
    /// The public key (starts with "age1").
    pub public: PublicKey,
    /// The secret key (zeroized on drop).
    pub secret: SecretKey,
}

impl KeyPair {
    /// Creates a new keypair (internal use only).
    pub(crate) fn new(public: PublicKey, secret: SecretKey) -> Self {
        Self { public, secret }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{PublicKey, SecretKey};

    #[test]
    fn test_keypair_new() {
        let pub_key = PublicKey::new("age1pub".to_string()).unwrap();
        let secret_key = SecretKey::new("secret".to_string());
        let kp = KeyPair::new(pub_key, secret_key);
        assert_eq!(kp.public.expose(), "age1pub");
        assert_eq!(kp.secret.expose(), "secret");
    }
}
