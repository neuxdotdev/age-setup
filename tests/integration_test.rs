use age_setup::errors::{GenerationError, ValidationError};
use age_setup::types::{PublicKey, SecretKey};
use age_setup::*;
#[test]
fn test_build_keypair_returns_valid_keypair() {
    let kp = build_keypair().expect("Failed to generate keypair");
    assert!(kp.public.expose().starts_with("age1"));
    assert!(!kp.public.expose().is_empty());
    assert!(!kp.secret.expose().is_empty());
}
#[test]
fn test_public_key_display_and_asref_and_clone() {
    let kp = build_keypair().unwrap();
    let pub_key = kp.public;
    assert_eq!(format!("{}", pub_key), pub_key.expose());
    assert_eq!(pub_key.as_ref(), pub_key.expose());
    let pub_key2 = pub_key.clone();
    assert_eq!(pub_key.expose(), pub_key2.expose());
}
#[test]
fn test_secret_key_display_redacts_and_clone() {
    let kp = build_keypair().unwrap();
    let secret = kp.secret;
    assert_eq!(format!("{}", secret), "[REDACTED]");
    let exposed = secret.expose();
    assert!(!exposed.is_empty());
    let secret2 = secret.clone();
    assert_eq!(secret.expose(), secret2.expose());
}
#[test]
fn test_keypair_fields_accessible() {
    let kp = build_keypair().unwrap();
    let _public: &PublicKey = &kp.public;
    let _secret: &SecretKey = &kp.secret;
}
#[test]
fn test_error_enums_convert_to_error() {
    let validation_err = ValidationError::InvalidPublicKeyFormat {
        reason: "test reason".to_string(),
    };
    let err1: Error = validation_err.into();
    assert!(matches!(err1, Error::Validation(_)));
    assert!(format!("{}", err1).contains("Invalid public key format"));
    let gen_err = GenerationError::IdentityCreationFailed;
    let err2: Error = gen_err.into();
    assert!(matches!(err2, Error::Generation(_)));
}
#[test]
fn test_result_type_alias_works() {
    fn returns_ok() -> Result<()> {
        Ok(())
    }
    fn returns_err() -> Result<()> {
        Err(Error::Generation(GenerationError::IdentityCreationFailed))
    }
    assert!(returns_ok().is_ok());
    assert!(returns_err().is_err());
}
#[test]
fn test_build_keypair_always_produces_different_keys() {
    let kp1 = build_keypair().unwrap();
    let kp2 = build_keypair().unwrap();
    assert_ne!(kp1.public.expose(), kp2.public.expose());
    assert_ne!(kp1.secret.expose(), kp2.secret.expose());
}
#[test]
fn test_public_key_string_starts_with_age1() {
    for _ in 0..10 {
        let kp = build_keypair().unwrap();
        assert!(kp.public.expose().starts_with("age1"));
    }
}
