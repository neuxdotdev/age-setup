//! Memory security utilities.
//!
//! This module provides functions for securely erasing sensitive data from memory,
//! ensuring that secrets are not left behind after use. It wraps the [`zeroize`]
//! crate to guarantee that overwrites are not optimized away by the compiler.

use zeroize::Zeroize;

/// Overwrites a byte buffer with zeros, preventing sensitive data from lingering
/// in memory.
///
/// This is a thin wrapper around the [`Zeroize`] trait from the `zeroize` crate.
/// It guarantees that the memory will be cleared even if the compiler would
/// otherwise optimize away a simple assignment (e.g., a `memset` call that appears
/// dead).
///
/// # Why is this important?
///
/// When secret keys, passwords, or other sensitive data are no longer needed, they
/// should be explicitly erased from memory. Otherwise, they might remain in
/// freed memory pages, swap space, or core dumps, where they could be recovered
/// by an attacker.
///
/// # Example
///
/// ```rust
/// use age_setup::security::wipe_memory;
///
/// let mut secret = vec![0x41, 0x42, 0x43]; // "ABC"
/// wipe_memory(&mut secret);
/// assert_eq!(secret, vec![0, 0, 0]);
/// ```
///
/// # How it works
///
/// The `zeroize` crate implements the [`Zeroize`] trait for `u8` slices. When
/// called, it writes zeros to every element, using a volatile write to prevent
/// the compiler from optimizing the operation away. After this function returns,
/// the original data is irreversibly gone from that mutable slice.
///
/// # Safety
///
/// This function is memory‑safe. It operates on a mutable reference and does not
/// read or write beyond the bounds of the slice. It never panics.
pub fn wipe_memory(data: &mut [u8]) {
    data.zeroize();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wipe_non_empty_buffer() {
        let mut v = vec![1, 2, 3];
        wipe_memory(&mut v);
        assert_eq!(v, vec![0, 0, 0]);
    }

    #[test]
    fn wipe_empty_buffer() {
        let mut v: Vec<u8> = vec![];
        wipe_memory(&mut v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn wipe_sensitive_data() {
        let mut secret_key = b"AGE-SECRET-KEY-1TEST".to_vec();
        wipe_memory(&mut secret_key);
        assert!(secret_key.iter().all(|&b| b == 0));
    }
}
