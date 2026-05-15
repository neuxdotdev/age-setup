//! # age-authenticator
//!
//! A secure authentication library built on the [age](https://age-encryption.org) encryption
//! protocol. Provides key generation, validation, memory-safe secret storage, and persistent
//! configuration management via [`neuxcfg`](https://crates.io/crates/neuxcfg).
//!
//! ## Quick Start
//!
//! ```no_run
//! use age_setup::{build_keypair, init_config};
//!
//! // Initialize configuration store
//! init_config()?;
//!
//! // Generate a new key pair
//! let keypair = build_keypair()?;
//! println!("Public key: {}", keypair.public);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Modules
//!
//! | Module | Purpose |
//! |--------|---------|
//! | [`keypair`] | Key pair container with redacted debug |
//! | [`public_key`] | Age public key wrapper with validation |
//! | [`secret_key`] | Zeroizing secret key wrapper |
//! | [`generator`] | Age X25519 identity generation |
//! | [`validation`] | Prefix validation for age keys |
//! | [`security`] | Memory wiping utilities |
//! | [`config`] | Persistent configuration via neuxcfg |
//! | [`errors`] | Structured error types |

pub mod config;
pub mod errors;
pub mod generator;
pub mod keypair;
pub mod public_key;
pub mod secret_key;
pub mod security;
pub mod validation;

pub use config::init as init_config;
pub use errors::{Error, GenerationError, Result, ValidationError};
pub use generator::build_keypair;
pub use keypair::KeyPair;
pub use public_key::PublicKey;
pub use secret_key::SecretKey;
