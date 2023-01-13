extern crate sha2;
use sha2::{Digest, Sha256};

// I do not trust random generators made by others.
// I know for sure that the output of a SHA-256 
// hash is indistinguishible from random bytes.
// Thus, inside a given socket, just renew the IV 
// at each request with this function when logging.
pub fn update_iv(iv: &[u8]) -> [u8; IV_LENGTH] {
    let mut hasher = Sha256::new();
    hasher.update(iv);
    hasher.finalize()[..IV_LENGTH]
        .try_into()
        .expect("IV Length should be correct")
}
