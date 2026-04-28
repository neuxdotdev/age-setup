use crate::errors::Result;
use crate::keypair::KeyPair;
use crate::public_key::PublicKey;
use crate::secret_key::SecretKey;
use age::secrecy::ExposeSecret;
use age::x25519::Identity;
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
    fn build_valid() {
        let kp = build_keypair().unwrap();
        assert!(kp.public.expose().starts_with("age1"));
        assert!(kp.secret.expose_secret().starts_with("AGE-SECRET-KEY-1"));
    }
}
