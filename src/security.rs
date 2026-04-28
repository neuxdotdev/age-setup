use crate::errors::Result;
use zeroize::Zeroize;
#[must_use]
pub fn wipe_memory(data: &mut [u8]) -> Result<()> {
    data.zeroize();
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wipe() {
        let mut v = vec![1, 2, 3];
        wipe_memory(&mut v).unwrap();
        assert_eq!(v, vec![0, 0, 0]);
    }
}
