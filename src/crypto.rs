use aes::{
    self,
    cipher::{
        block_padding::Pkcs7, generic_array::GenericArray, typenum::U32, BlockDecryptMut,
        BlockEncryptMut, KeyIvInit,
    },
    Aes256,
};

use cbc::{Decryptor, Encryptor};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use sha2::{Digest, Sha256};

use anyhow as ah;
use anyhow::{anyhow, bail};

enum Pkcs7Error {
    Pkcs7DrainedBytesMismatch {
        drained_bytes: Vec<u8>,
    },
    Pkcs7PaddingSumMismatch {
        data_length: usize,
        padding_length: usize,
        padding_sum: usize,
    },
    Pkcs7InvalidPaddingLength {
        data_length: usize,
        padding_length: usize,
    },
}

impl std::error::Error for Pkcs7Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl std::fmt::Debug for Pkcs7Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // match self {
        //     // Pkcs7Error::Pkcs7DrainedBytesMismatch => todo!(),
        //     // Pkcs7Error::Pkcs7PaddingSumMismatch => todo!(),
        //     // Pkcs7Error::Pkcs7InvalidPaddingLength => todo!(),
        //     todo!();
        // }
        //
        todo!()
    }
}
impl std::fmt::Display for Pkcs7Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub fn pkcs7_apply(data_bytes: &mut Vec<u8>, multiple: usize) -> ah::Result<usize> {
    let data_length = data_bytes.len();

    if multiple == 0 || data_length == 0 {
        return Ok(0);
    }

    let length_remainder = data_length % multiple;

    if length_remainder == 0 {
        return Ok(0);
    }

    let distance_to_multiple = multiple - length_remainder;
    let padding_length = data_length + distance_to_multiple;

    data_bytes.resize(padding_length, distance_to_multiple as u8);

    Ok(distance_to_multiple)
}

pub fn pkcs7_strip(data_bytes: &mut Vec<u8>) -> ah::Result<usize> {
    let mut padding_length: usize = 0;
    let data_length = data_bytes.len();

    if let Some(&last) = data_bytes.last() {
        padding_length = last as usize;

        let original_data_length = data_length - padding_length;
        let _original_data_modulo = original_data_length % padding_length;

        if padding_length == 0 || padding_length >= data_length {
            bail!(Pkcs7Error::Pkcs7InvalidPaddingLength {
                data_length,
                padding_length,
            });
        }

        let padding_start = data_length - padding_length;

        let padding_sum = (padding_start..data_length).fold(0, |acc, i| {
            if usize::from(data_bytes[i]) == padding_length {
                acc + 1
            } else {
                acc
            }
        });

        if padding_sum != padding_length {
            bail!(Pkcs7Error::Pkcs7PaddingSumMismatch {
                data_length,
                padding_length,
                padding_sum
            });
        }

        let mut drained = data_bytes.drain(padding_start..data_length);

        if drained.len() != padding_length || drained.any(|b| b != last) {
            bail!(Pkcs7Error::Pkcs7DrainedBytesMismatch {
                drained_bytes: drained.collect()
            });
        }
    }

    Ok(padding_length)
}

fn random_block() -> [u8; 16] {
    let mut rng = StdRng::from_entropy();
    let mut buffer: [u8; 16] = [0; 16];
    rng.fill_bytes(&mut buffer);
    buffer
}

pub fn derive_key(data: &Vec<u8>) -> ah::Result<[u8; 32]> {
    let config = argon2::Config::default();
    let mut buffer: [u8; 32] = [0u8; 32];

    let hash = argon2::hash_raw(data.as_slice(), data.as_slice(), &config)?;

    if hash.len() <= buffer.len() {
        for (i, e) in hash.iter().enumerate() {
            buffer[i] = *e;
        }
    }

    Ok(buffer)
}

pub fn get_sha256_digest(data: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data.as_slice());
    let hash: GenericArray<u8, U32> = hasher.finalize();
    hash.to_vec()
}

pub fn encrypt(plaintext: &Vec<u8>, key: &[u8; 32]) -> ah::Result<Vec<u8>> {
    let mut in_data: Vec<u8> = random_block().to_vec();
    in_data.append(&mut get_sha256_digest(plaintext));
    in_data.append(&mut plaintext.clone());

    let iv: [u8; 16] = random_block();
    let encryptor = Encryptor::<Aes256>::new(key.into(), &iv.into());

    let in_length = in_data.len();

    let mut cipher_buf = in_data.clone();
    cipher_buf.resize(cipher_buf.len() + 16, 0u8);

    let ciphertext = encryptor.encrypt_padded_mut::<Pkcs7>(cipher_buf.as_mut_slice(), in_length)?;

    Ok(ciphertext.to_vec())
}

pub fn decrypt(ciphertext: &Vec<u8>, key: &[u8; 32]) -> ah::Result<Vec<u8>> {
    let iv: [u8; 16] = random_block();
    let decryptor = Decryptor::<Aes256>::new(key.into(), &iv.into());

    let mut cipher_buf = ciphertext.clone();
    let plaintext = decryptor.decrypt_padded_mut::<Pkcs7>(cipher_buf.as_mut_slice())?;

    if plaintext.len() < 16 {
        return Err(anyhow!(
            "The decrypted output is smaller than the minimum of 16 bytes. Is {} bytes instead.",
            plaintext.len()
        ));
    }

    let mut plaintext = plaintext.to_vec();
    let _ = plaintext.drain(0..16);
    let checksum_stored: Vec<u8> = plaintext.drain(0..32).collect();
    let checksum_actual = get_sha256_digest(&plaintext);

    if checksum_stored != checksum_actual {
        return Err(anyhow!(
            "There is a checksum mismatch: {:?} (stored) != {:?} (calculated)",
            checksum_stored,
            checksum_actual
        ));
    }

    Ok(plaintext.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkcs7_apply_fixed() {
        let mut sample = vec![1, 2, 3, 4, 5];
        match pkcs7_apply(&mut sample, 2) {
            Ok(padding_length) => {
                assert_eq!(padding_length, 1);
                assert_eq!(sample, vec![1, 2, 3, 4, 5, 1]);
            }
            Err(error) => {
                panic!("Unexpected error: {:?}", error);
            }
        };
    }

    #[test]
    fn test_pkcs7_strip_fixed() {
        let mut sample = vec![1, 2, 3, 4, 5, 1];

        match pkcs7_strip(&mut sample) {
            Ok(padding_length) => {
                assert_eq!(padding_length, 1);
                assert_eq!(sample, vec![1, 2, 3, 4, 5]);
            }
            Err(error) => {
                panic!("Unexpected error: {:?}", error);
            }
        };
    }
}
