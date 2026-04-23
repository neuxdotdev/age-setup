//! # age-setup
//!
//! A library for generating age keypairs (X25519) with validation and secure memory handling.
//!
//! This crate provides a simple way to create a keypair for age encryption,
//! returning a [`KeyPair`] containing a public key (starting with "age1") and a secret key.
//! The secret key is automatically zeroized on drop for security.
//!
//! # Example
//!
//! ```
//! use age_setup::build_keypair;
//!
//! let keypair = build_keypair().expect("failed to generate keypair");
//! println!("Public key: {}", keypair.public);
//! println!("Secret key: [REDACTED]"); // Display redacts secret
//! // Access raw secret with .expose() – use with caution!
//! # let _ = keypair.secret.expose();
//! ```
//!
//! # Features
//! - Generate X25519 keypairs compatible with age.
//! - Public key validation (must start with "age1").
//! - Secret key zeroization on drop.
//! - Redacted `Display` for secret key.
//! - Error handling with detailed error types.

pub mod apis;
pub mod build;
pub mod errors;
pub mod security;
pub mod types;

pub use apis::build::build_keypair;
pub use errors::{Error, Result};
pub use types::KeyPair;
