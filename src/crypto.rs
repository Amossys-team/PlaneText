extern crate aes;
extern crate cbc;
extern crate sha2;
use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};
use sha2::{Digest, Sha256};
use crate::Error;

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
const IV_LENGTH: usize = 16;

pub fn aes_cbc_cipher(key: &[u8], iv: &[u8], data: &[u8]) -> Result<Vec<u8>, Error> {
    let cipher = Aes128CbcEnc::new(key.into(), iv.into());

    let cipher_length = get_padded_size(data.len());
    let mut outbuf = vec![0; cipher_length];
    let result = cipher.encrypt_padded_b2b_mut::<Pkcs7>(data, &mut outbuf);
    if let Err(e) = result {
        eprintln!("{e}");
        return Err(Error::PaddingError);
    }
    Ok(outbuf)
}

fn get_padded_size(len: usize) -> usize {
    ((len + 1) as f64 / 16_f64).ceil() as usize * 16
}

pub fn update_iv(iv: &[u8]) -> [u8; IV_LENGTH] {
    let mut hasher = Sha256::new();
    hasher.update(iv);
    hasher.finalize()[..IV_LENGTH]
        .try_into()
        .expect("IV Length should be correct")
}

#[cfg(test)]
mod test {
    extern crate bytes;
    use super::*;
    #[test]
    fn test_iv_generation() {
        let iv = update_iv(&(259 as u64).to_be_bytes());
        assert_eq!(iv[0], 5);
        assert_eq!(iv[1], 236);
        assert_eq!(iv[2], 183);
        assert_eq!(iv[3], 226);
    }
    #[test]
    fn test_padding_size() {
        assert_eq!(get_padded_size(6), 16);
        assert_eq!(get_padded_size(15), 16);
        assert_eq!(get_padded_size(16), 32);
        assert_eq!(get_padded_size(32), 48);
    }
    #[test]
    fn test_aes_cbc() {
        let iv = "lalalalalalalala".as_bytes();
        let key = "lol1234567891234".as_bytes();
        let data = "\x00".repeat(16);
        let slice = aes_cbc_cipher(key, iv, &data.as_bytes()).unwrap();
        assert_eq!(227, slice[0]);
        assert_eq!(slice[1], 207);
        assert_eq!(slice[31], 73);
        assert_eq!(slice[30], 235);
    }
}
