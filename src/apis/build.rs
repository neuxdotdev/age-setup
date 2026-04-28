use crate::build::identity::create_identity;
use crate::build::recipient::extract_recipient;
use crate::errors::Result;
use crate::types::{KeyPair, PublicKey, SecretKey};
use age::secrecy::ExposeSecret;                         
pub fn build_keypair() -> Result<KeyPair> {
    let identity = create_identity()?;
    let recipient = extract_recipient(&identity);
    let public_raw = recipient.to_string();                     
    let secret_raw = identity.to_string().expose_secret().to_string();  
    let public = PublicKey::new(public_raw)?;
    let secret = SecretKey::new(secret_raw)?;
    Ok(KeyPair::new(public, secret))
}