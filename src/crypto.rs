use aes::{self, cipher, cipher::*, Aes256, Aes256Dec, Aes256Enc, Block, Block8};

use cipher::crypto_common::*;

#[derive(Debug, Clone)]
pub enum Error {
    Pkcs7InvalidPaddingLength {
        data_length: usize,
        padding_length: usize,
    },

    Pkcs7PaddingSumMismatch {
        data_length: usize,
        padding_length: usize,
        padding_sum: usize,
    },

    Pkcs7DrainedBytesMismatch(Vec<u8>),

    Pkcs7RedundantStripping {
        padding_length: usize,
        data_length: usize,
        original_data_length: usize,
        original_data_modulo: usize,
    },
}

pub fn pkcs7_apply(data_bytes: &mut Vec<u8>, multiple: usize) -> Result<usize, Error> {
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

pub fn pkcs7_strip(data_bytes: &mut Vec<u8>) -> Result<usize, Error> {
    let mut padding_length: usize = 0;
    let data_length = data_bytes.len();

    if let Some(&last) = data_bytes.last() {
        padding_length = last as usize;

        let original_data_length = data_length - padding_length;
        let original_data_modulo = original_data_length % padding_length;

        if original_data_modulo == 0 {
            return Err(Error::Pkcs7RedundantStripping {
                padding_length,
                data_length,
                original_data_length,
                original_data_modulo,
            });
        }

        if padding_length == 0 || padding_length >= data_length {
            return Err(Error::Pkcs7InvalidPaddingLength {
                data_length,
                padding_length,
            });
        }

        let padding_start = data_length - padding_length;

        let padding_sum = (padding_start..data_length).fold(0, |acc, i| {
            (usize::from(data_bytes[i]) == padding_length)
                .then_some(acc + 1)
                .unwrap_or(acc)
        });

        if padding_sum != padding_length {
            return Err(Error::Pkcs7PaddingSumMismatch {
                data_length,
                padding_length,
                padding_sum,
            });
        }

        let mut drained = data_bytes.drain(padding_start..data_length);

        if drained.len() != padding_length || drained.any(|b| b != last) {
            return Err(Error::Pkcs7DrainedBytesMismatch(drained.collect()));
        }
    }

    Ok(padding_length)
}

pub fn encrypt(plain_bytes: Vec<u8>) -> Vec<u8> {
    // Aes256Enc::encrypt_blocks_mut(&mut self, block)
    todo!();
}

pub fn decrypt(cipher_bytes: Vec<u8>) -> Vec<u8> {
    todo!();
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
}
