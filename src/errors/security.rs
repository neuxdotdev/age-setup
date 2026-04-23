//! Security operation error types.

/// Errors that occur during security operations (e.g., memory wiping).
#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    /// Memory wipe operation failed (unlikely, but included for completeness).
    #[error("Memory wipe operation failed")]
    MemoryWipeFailed,
}
