extern "C" {
    pub static BATCH_SIZE: u32;
    pub fn hash(challenge: *const u8, nonce: *const u8, out: *mut u64);
    pub fn solve_all_stages(hashes: *const u64, out: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INDEX_SPACE: usize = 65536;

    fn hashspace_size() -> usize {
        unsafe { BATCH_SIZE as usize * INDEX_SPACE }
    }

    #[test]
    fn test_gpu() {
        unsafe {
            let challenge = [255; 32];
            let nonce = [2; 8];
            let mut hashes = vec![vec![0u64; INDEX_SPACE]; BATCH_SIZE as usize];
            hash(
                challenge.as_ptr(),
                nonce.as_ptr(),
                hashes.as_mut_ptr() as *mut u64,
            );
            for i in 0..BATCH_SIZE as usize {
                let mut digest = [0u8; 16];
                solve_all_stages(hashes[i].as_ptr(), digest.as_mut_ptr());
                let solution = crate::Solution::new(digest, nonce);
                assert!(solution.is_valid(&challenge));
            }
        }
    }
}
