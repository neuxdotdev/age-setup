# age-setup

> **Simple, secure X25519 keypair generation for age encryption — with validation and automatic memory zeroization.**

<p align="center">
  <a href="https://crates.io/crates/age-setup"><img src="https://img.shields.io/crates/v/age-setup.svg?style=flat-square&logo=rust" alt="crates.io version"></a>
  <a href="https://docs.rs/age-setup"><img src="https://img.shields.io/docsrs/age-setup?style=flat-square&logo=docs.rs" alt="docs.rs"></a>
  <a href="https://crates.io/crates/age-setup"><img src="https://img.shields.io/crates/d/age-setup?style=flat-square&logo=rust" alt="crates.io downloads"></a>
  <a href="https://github.com/neuxdotdev/age-setup/blob/main/LICENSE"><img src="https://img.shields.io/crates/l/age-setup?style=flat-square" alt="license"></a>
  <a href="https://github.com/neuxdotdev/age-setup/actions"><img src="https://img.shields.io/github/actions/workflow/status/neuxdotdev/age-setup/ci.yml?branch=main&style=flat-square&logo=github" alt="build status"></a>
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/Rust-2021-orange?style=flat-square&logo=rust" alt="Rust 2021"></a>
</p>

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
    - [Basic Key Generation](#basic-key-generation)
    - [Error Handling](#error-handling)
    - [Accessing Keys](#accessing-keys)
- [API Reference](#api-reference)
    - [Functions](#functions)
    - [Types](#types)
    - [Error Types](#error-types)
- [Security Features](#security-features)
    - [Memory Zeroization](#memory-zeroization)
    - [Display Redaction](#display-redaction)
    - [Public Key Validation](#public-key-validation)
- [Development](#development)
    - [Project Structure](#project-structure)
    - [Building](#building)
    - [Testing](#testing)
    - [Benchmarking](#benchmarking)
- [Contributing](#contributing)
- [License](#license)
- [Credits](#credits)

---

## Features

✨ **Simple API** — One function call generates a complete keypair  
🔒 **Secure by Default** — Automatic memory zeroization on drop  
✅ **Validated Keys** — Public keys guaranteed to start with "age1"  
🛡️ **Privacy-First** — Secret keys redacted in `Display` output  
📦 **Zero Config** — Works out of the box with sensible defaults  
🧪 **Well-Tested** — Comprehensive test coverage across all modules  
⚡ **Fast** — Built on the battle-tested `age` crate  
📖 **Documented** — Full API documentation with examples

---

## Installation

Add `age-setup` to your `Cargo.toml`:

```toml
[dependencies]
age-setup = "0.1"
```

Or use `cargo add`:

```bash
cargo add age-setup
```

**Minimum Supported Rust Version (MSRV):** 1.70.0

---

## Quick Start

Generate an age keypair in just two lines:

```rust
use age_setup::build_keypair;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keypair = build_keypair()?;

    println!("Public key: {}", keypair.public);
    println!("Secret key: {}", keypair.secret); // Prints: [REDACTED]

    // Access raw secret (use with caution!)
    let secret_str = keypair.secret.expose();

    Ok(())
}
```

That's it! You now have a cryptographically secure X25519 keypair ready for age encryption.

---

## Usage Examples

### Basic Key Generation

The simplest way to generate a keypair:

```rust
use age_setup::build_keypair;

let keypair = build_keypair().expect("failed to generate keypair");

// Public key is validated and guaranteed to start with "age1"
assert!(keypair.public.expose().starts_with("age1"));

// Use the keys with age encryption
println!("Share this public key: {}", keypair.public);
```

### Error Handling

Handle different error types explicitly:

```rust
use age_setup::{build_keypair, Error};

match build_keypair() {
    Ok(keypair) => {
        println!("✓ Generated keypair successfully");
        println!("  Public: {}", keypair.public);
    }
    Err(Error::Generation(e)) => {
        eprintln!("✗ Key generation failed: {}", e);
    }
    Err(Error::Validation(e)) => {
        eprintln!("✗ Key validation failed: {}", e);
    }
    Err(Error::Security(e)) => {
        eprintln!("✗ Security operation failed: {}", e);
    }
}
```

### Accessing Keys

```rust
use age_setup::build_keypair;

let keypair = build_keypair()?;

// Public key (safe to display)
let public_str: &str = keypair.public.expose();
let public_owned: String = keypair.public.to_string();

// Secret key (handle with care!)
let secret_str: &str = keypair.secret.expose();

// Convert to AsRef<str> for compatibility
fn print_key(key: &impl AsRef<str>) {
    println!("{}", key.as_ref());
}
print_key(&keypair.public);

// Secret is automatically zeroized when dropped
{
    let temp_keypair = build_keypair()?;
    // Use temp_keypair...
} // <-- Memory is securely wiped here
```

### Integration with age Encryption

```rust
use age_setup::build_keypair;
use std::io::Write;

let keypair = build_keypair()?;

// Encrypt a file with age
let encrypted = age::encrypt(
    &age::x25519::Recipient::from_str(keypair.public.expose())?,
    plaintext.as_bytes(),
)?;

// Save the secret key securely
let key_path = dirs::home_dir().unwrap().join(".age/key.txt");
std::fs::write(&key_path, format!("# age identity\n{}", keypair.secret.expose()))?;

// Set restrictive permissions (Unix only)
#[cfg(unix)]
{
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&key_path, std::fs::Permissions::from_mode(0o600))?;
}
```

---

## API Reference

### Functions

#### `build_keypair()`

Generates a new age X25519 keypair.

```rust
pub fn build_keypair() -> Result<KeyPair>
```

**Returns:**

- `Ok(KeyPair)` — A new keypair with validated public key
- `Err(Error)` — If generation or validation fails

**Example:**

```rust
let keypair = age_setup::build_keypair()?;
```

---

### Types

#### `KeyPair`

A keypair consisting of an age public key and its corresponding secret key.

```rust
pub struct KeyPair {
    pub public: PublicKey,
    pub secret: SecretKey,
}
```

**Fields:**

- `public: PublicKey` — The public key (starts with "age1")
- `secret: SecretKey` — The secret key (zeroized on drop)

**Example:**

```rust
let keypair = build_keypair()?;
println!("Public: {}", keypair.public);
println!("Secret: {}", keypair.secret); // Prints: [REDACTED]
```

---

#### `PublicKey`

An age public key, guaranteed to start with "age1".

```rust
pub struct PublicKey(String);
```

**Methods:**

##### `expose(&self) -> &str`

Returns the raw string representation of the public key.

```rust
let public_str = keypair.public.expose();
assert!(public_str.starts_with("age1"));
```

**Traits Implemented:**

- `Display` — Prints the public key
- `Debug` — Debug representation
- `Clone` — Can be cloned safely
- `AsRef<str>` — Convert to string reference

---

#### `SecretKey`

An age secret key that securely wipes its memory when dropped.

```rust
pub struct SecretKey { /* private fields */ }
```

**Methods:**

##### `expose(&self) -> &str`

Exposes the raw secret key as a string.

**⚠️ Security Warning:** Only use this when absolutely necessary, as it exposes the secret.

```rust
let secret_str = keypair.secret.expose();
// Use secret_str with caution!
```

**Traits Implemented:**

- `Display` — Prints `[REDACTED]` instead of the actual secret
- `Debug` — Debug representation (does not expose secret)
- `Clone` — Can be cloned (new copy is also zeroized on drop)
- `Drop` — Automatically zeroizes memory when dropped

---

### Error Types

#### `Error`

Main error type for the crate.

```rust
pub enum Error {
    Generation(GenerationError),
    Validation(ValidationError),
    Security(SecurityError),
}
```

**Variants:**

##### `Error::Generation`

Error during key generation (e.g., internal library failure).

```rust
pub enum GenerationError {
    IdentityCreationFailed,
}
```

##### `Error::Validation`

Error validating public key format.

```rust
pub enum ValidationError {
    InvalidPublicKeyFormat { reason: String },
}
```

**Example:**

```rust
use age_setup::types::PublicKey;

let result = PublicKey::new("invalid_key".to_string());
assert!(result.is_err()); // Does not start with "age1"
```

##### `Error::Security`

Error during security operations (e.g., memory wipe).

```rust
pub enum SecurityError {
    MemoryWipeFailed,
}
```

**All errors implement:**

- `std::error::Error` — Standard error trait
- `Display` — Human-readable error messages
- `Debug` — Detailed debug information

---

## Security Features

### Memory Zeroization

The `SecretKey` type automatically zeroes its memory when dropped, preventing secrets from lingering in RAM.

```rust
{
    let keypair = build_keypair()?;
    // Secret is stored in memory
    let secret = keypair.secret.expose();
    // ... use secret ...
} // <-- Memory is securely overwritten with zeros here
```

**Implementation:**

- Uses the [`zeroize`](https://crates.io/crates/zeroize) crate
- Compiler optimizations cannot remove the zeroing operation
- Applies to both the original and cloned instances

---

### Display Redaction

Secret keys are automatically redacted when printed:

```rust
let keypair = build_keypair()?;

println!("{}", keypair.secret);        // Prints: [REDACTED]
println!("{:?}", keypair.secret);      // Prints: SecretKey { ... }

// Only expose() shows the actual secret
println!("{}", keypair.secret.expose()); // Prints actual key
```

This prevents accidental logging or display of sensitive material.

---

### Public Key Validation

All public keys are validated to ensure they start with the "age1" prefix:

```rust
use age_setup::types::PublicKey;

// Valid key
let valid = PublicKey::new("age1abcdef...".to_string())?;

// Invalid key (will return error)
let invalid = PublicKey::new("ssh-rsa AAAA...".to_string());
assert!(invalid.is_err());
```

**Validation Rules:**

- Must not be empty
- Must start with "age1"
- Automatically applied during keypair generation

---

## Development

### Project Structure

```
age-setup/
├── src/
│   ├── apis/
│   │   ├── build.rs          # Keypair building API
│   │   └── mod.rs
│   ├── build/
│   │   ├── identity.rs       # Internal identity generation
│   │   ├── recipient.rs      # Recipient extraction
│   │   └── mod.rs
│   ├── errors/
│   │   ├── buildings.rs      # Generation errors
│   │   ├── security.rs       # Security errors
│   │   ├── validation.rs     # Validation errors
│   │   └── mod.rs
│   ├── security/
│   │   ├── zeroize.rs        # Memory zeroization utilities
│   │   └── mod.rs
│   ├── types/
│   │   ├── keypair.rs        # KeyPair structure
│   │   ├── public_key.rs     # PublicKey type
│   │   ├── secret_key.rs     # SecretKey type
│   │   ├── validation.rs     # Validation logic
│   │   └── mod.rs
│   └── lib.rs                # Library entry point
├── benches/
│   └── keygen.rs             # Benchmarks
├── Cargo.toml
└── README.md
```

---

### Building

```bash
# Clone the repository
git clone https://github.com/neuxdotdev/age-setup.git
cd age-setup

# Build the library
cargo build

# Build with optimizations
cargo build --release

# Build documentation
cargo doc --open
```

---

### Testing

The library has comprehensive test coverage across all modules.

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run tests for a specific module
cargo test types::

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

**Test Coverage:**

- ✅ Keypair generation
- ✅ Public key validation
- ✅ Secret key zeroization
- ✅ Display implementations
- ✅ Error handling
- ✅ Type conversions

---

### Benchmarking

Performance benchmarks are available using Criterion:

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench keygen

# Generate benchmark report
cargo bench -- --save-baseline main
```

**Benchmark Results** (example on Apple M1):

```
keygen/build_keypair   time:   [45.2 µs 45.8 µs 46.5 µs]
```

---

## Contributing

Contributions are welcome! Here's how you can help:

1. **Fork the repository**

    ```bash
    git clone https://github.com/neuxdotdev/age-setup.git
    ```

2. **Create a feature branch**

    ```bash
    git checkout -b feat/your-feature
    ```

3. **Make your changes**
    - Write tests for new functionality
    - Ensure all tests pass: `cargo test`
    - Format code: `cargo fmt`
    - Run clippy: `cargo clippy`

4. **Commit your changes**

    ```bash
    git commit -m "feat: add new feature"
    ```

    Use [Conventional Commits](https://www.conventionalcommits.org/) format:
    - `feat:` — New feature
    - `fix:` — Bug fix
    - `docs:` — Documentation changes
    - `test:` — Test additions/changes
    - `refactor:` — Code refactoring

5. **Push and create a Pull Request**
    ```bash
    git push origin feat/your-feature
    ```

### Code of Conduct

Please be respectful and constructive in all interactions. We're here to build great software together.

---

## License

**MIT License** — see [LICENSE](LICENSE) for details.

This library is free to use in both open-source and commercial projects.

```
Copyright (c) 2026 neuxdotdev

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
```

---

## Credits

- Built on the excellent [`age`](https://crates.io/crates/age) crate by [@str4d](https://github.com/str4d)
- Uses [`zeroize`](https://crates.io/crates/zeroize) for secure memory handling
- Inspired by the simplicity of the [age specification](https://age-encryption.org/)

---

## Related Projects

- **[age](https://age-encryption.org/)** — A simple, modern, and secure file encryption tool
- **[rage](https://github.com/str4d/rage)** — A Rust implementation of age
- **[age-plugin](https://crates.io/crates/age-plugin)** — Framework for age plugins

---

## FAQ

### Why use this instead of calling age directly?

`age-setup` provides:

- **Validation** — Ensures public keys always have the correct format
- **Security** — Automatic memory zeroization of secrets
- **Simplicity** — Single function call instead of multiple steps
- **Safety** — Type-safe wrappers prevent misuse

### Is this production-ready?

Yes! The library:

- ✅ Uses battle-tested dependencies (`age`, `zeroize`)
- ✅ Has comprehensive test coverage
- ✅ Follows Rust security best practices
- ✅ Is actively maintained

### How do I save keys to disk?

```rust
use age_setup::build_keypair;
use std::fs;

let keypair = build_keypair()?;

// Save public key
fs::write("key.pub", keypair.public.expose())?;

// Save secret key (with proper permissions!)
let secret_content = format!("# age identity\n{}", keypair.secret.expose());
fs::write("key.txt", secret_content)?;

#[cfg(unix)]
{
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions("key.txt", fs::Permissions::from_mode(0o600))?;
}
```

### Can I use this with async code?

Yes! Key generation is CPU-bound and very fast (~45µs), so you can call it directly:

```rust
tokio::task::spawn_blocking(|| {
    age_setup::build_keypair()
}).await??;
```

---

> **Repository:** https://github.com/neuxdotdev/age-setup  
> **Issues:** https://github.com/neuxdotdev/age-setup/issues  
> **Crates.io:** https://crates.io/crates/age-setup  
> **Documentation:** https://docs.rs/age-setup

---

**Made with ❤️ by [neuxdotdev](https://github.com/neuxdotdev)**
