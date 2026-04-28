use crate::public_key::PublicKey;
use crate::secret_key::SecretKey;
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
