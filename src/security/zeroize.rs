use crate::errors::Result;
use zeroize::Zeroize;
#[must_use = "wipe_memory should be called to ensure memory is cleared"]
pub(crate) fn wipe_memory(data: &mut [u8]) -> Result<()> {
    data.zeroize();
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_wipe_memory_zeroizes() {
        let mut data = vec![1, 2, 3, 4];
        let result = wipe_memory(&mut data);
        assert!(result.is_ok());
        assert_eq!(data, vec![0, 0, 0, 0]);
    }
}
