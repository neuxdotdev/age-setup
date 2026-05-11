//! # age‑setup – Simple, Secure X25519 Key Pair Generation for Age
//!
//! This crate provides a one‑function API for generating X25519 key pairs
//! compatible with the [**age** encryption tool](https://age-encryption.org/).
//! Every generated key pair is automatically validated and protected by
//! memory‑zeroisation, so you can focus on encrypting data without worrying
//! about cryptographic details or secret leakage.
//!
//! ## Design
//!
//! The crate is organised into small, focused modules:
//!
//! | Module | Responsibility |
//! |--------|----------------|
//! | [`generator`] | The main entry point – generates a fresh [`KeyPair`]. |
//! | [`keypair`]   | The [`KeyPair`] struct that holds a [`PublicKey`] and a [`SecretKey`]. |
//! | [`public_key`] | A validated wrapper for age public keys (prefix `age1`). |
//! | [`secret_key`] | A validated wrapper for age secret keys, with zeroisation on drop. |
//! | [`security`]   | Low‑level memory‑zeroisation helpers (public, reusable). |
//! | [`validation`] | Internal sanity checks used by [`PublicKey`] and [`SecretKey`]. |
//! | [`errors`]     | All error types returned by the crate. |
//!
//! ## Quick start
//!
//! ```rust
//! use age_setup::build_keypair;
//!
//! fn main() -> age_setup::Result<()> {
//!     let kp = build_keypair()?;
//!     println!("Public key: {}", kp.public);    // age1...
//!     println!("Secret key: {}", kp.secret);    // [REDACTED]
//!     Ok(())
//! }
//! ```
//!
//! ## Feature flags
//!
//! This crate does not expose any feature flags itself; it inherits the
//! default TLS backend from the `age` crate (`rustls`). You can switch to
//! native‑TLS by enabling the corresponding feature in `age`.

pub mod errors;
pub mod generator;
pub mod keypair;
pub mod public_key;
pub mod secret_key;
pub mod security;
pub mod validation;

// ---------------------------------------------------------------------------
// Public re‑exports – everything you need to generate and work with key pairs
// ---------------------------------------------------------------------------

/// All error types used by the crate.
///
/// Re‑exports:
/// - [`errors::Error`] – the main error enum (generation + validation).
/// - [`errors::GenerationError`] – errors that can occur during key creation.
/// - [`errors::ValidationError`] – errors for invalid public/secret key formats.
/// - [`errors::Result`] – a convenience alias for `std::result::Result<T, Error>`.
pub use errors::{Error, GenerationError, Result, ValidationError};

/// The core function that generates a new X25519 key pair.
///
/// See [`generator::build_keypair`] for detailed documentation.
pub use generator::build_keypair;

/// A validated, memory‑safe age key pair.
///
/// Contains a [`PublicKey`] and a [`SecretKey`]. Obtained exclusively via
/// [`build_keypair`].
pub use keypair::KeyPair;

/// A validated age public key that always starts with `age1`.
///
/// Safe to display, share, and clone.
pub use public_key::PublicKey;

/// A validated age secret key that is automatically zeroised when dropped.
///
/// Its [`Display`] and [`Debug`] implementations print `[REDACTED]` to prevent
/// accidental exposure.
pub use secret_key::SecretKey;
