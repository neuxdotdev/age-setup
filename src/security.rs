use zeroize::Zeroize;

/// Overwrites a byte buffer with zeros in a way that cannot be optimized away.
///
/// Uses the [`Zeroize`] trait from the `zeroize` crate to guarantee that the
/// memory is cleared even in release builds. This is essential for securely
/// erasing sensitive material such as secret keys from memory.
///
/// # Parameters
///
/// * `data` – A mutable byte slice whose contents will be zeroized.
///
/// # Examples
///
/// ```rust
/// use age_setup::security::wipe_memory;
///
/// let mut secret = vec![1, 2, 3, 4];
/// wipe_memory(&mut secret);
/// assert_eq!(secret, vec![0, 0, 0, 0]);
/// ```
///
/// Empty buffers are handled gracefully:
///
/// ```rust
/// use age_setup::security::wipe_memory;
///
/// let mut empty: Vec<u8> = vec![];
/// wipe_memory(&mut empty);
/// assert_eq!(empty, vec![]);
/// ```
///
/// # See Also
///
/// * [`SecretKey`](crate::SecretKey) – Uses `Zeroizing` for automatic cleanup.
/// * [`zeroize`](https://docs.rs/zeroize) – The underlying crate.
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
