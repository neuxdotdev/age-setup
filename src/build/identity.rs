use crate::errors::Result;
use age::x25519::Identity;
pub(crate) fn create_identity() -> Result<Identity> {
    let identity = Identity::generate();
    Ok(identity)
}
