use crate::errors::Result;
use zeroize::Zeroize;
#[must_use]
pub(crate) fn wipe_memory(data: &mut [u8]) -> Result<()> {
    data.zeroize();
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_wipe() {
        let mut data = vec![1, 2, 3, 4];
        wipe_memory(&mut data).unwrap();
        assert_eq!(data, vec![0, 0, 0, 0]);
    }
}