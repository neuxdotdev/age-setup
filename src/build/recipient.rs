use age::x25519::{Identity, Recipient};
pub(crate) fn extract_recipient(identity: &Identity) -> Recipient {
    identity.to_public()
}