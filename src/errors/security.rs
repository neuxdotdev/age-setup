#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Memory wipe operation failed")]
    MemoryWipeFailed,
}