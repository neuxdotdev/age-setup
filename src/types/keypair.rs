use super::{PublicKey, SecretKey};
#[derive(Debug)]
pub struct KeyPair {
    pub public: PublicKey,
    pub secret: SecretKey,
}
impl KeyPair {
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
        let secret_key = SecretKey::new("AGE-SECRET-KEY-1SECRET".to_string()).unwrap();
        let kp = KeyPair::new(pub_key, secret_key);
        assert_eq!(kp.public.expose(), "age1pub");
        assert_eq!(kp.secret.expose_secret(), "AGE-SECRET-KEY-1SECRET");
    }
}