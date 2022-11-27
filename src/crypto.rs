use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

use crate::prelude::*;
use crate::sbox::*;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

const CHUNK_SIZE: usize = 16;

/// This substitutes a mutable buffer
///
/// # Arguments
/// * `data` - the mutable buffer
fn substitute(data: &mut [u8]) {
    for i in 0..data.len() {
        data[i] = SUBSTITUTION_BOX[data[i] as usize];
    }
}

/// This inverts a substitution of a mutable buffer
///
/// # Arguments
/// * `data` - the mutable buffer
fn invert_substitute(data: &mut [u8]) {
    for i in 0..data.len() {
        data[i] = INVERSE_SUBSTITUTION_BOX[data[i] as usize];
    }
}

/// This xors a mutable buffer with a key
///
/// # Arguments
/// * `data` - the mutable buffer
/// * `key` - the key buffer to xor with
fn xor(data: &mut [u8], key: &Vec<u8>, offset: usize) {
    for i in 0..data.len() {
        data[i] ^= key[(offset + i) % key.len()];
    }
}

/// Returns a shuffled key
///
/// # Arguments
/// * `key` - the key to use
fn shuffle(key: &Vec<u8>) -> Vec<u8> {
    let mut length = key.len();
    while length % 16 != 0 {
        length += 1;
    }

    let mut new_key = vec![0u8; length];

    for i in (0..key.len()).step_by(16) {
        new_key[i + 0] = *key.get(i + 4).unwrap_or(&0);
        new_key[i + 1] = *key.get(i + 8).unwrap_or(&0);
        new_key[i + 2] = *key.get(i + 12).unwrap_or(&0);
        new_key[i + 3] = *key.get(i + 0).unwrap_or(&0);

        new_key[i + 4] = *key.get(i + 9).unwrap_or(&0);
        new_key[i + 5] = *key.get(i + 13).unwrap_or(&0);
        new_key[i + 6] = *key.get(i + 1).unwrap_or(&0);
        new_key[i + 7] = *key.get(i + 5).unwrap_or(&0);

        new_key[i + 8] = *key.get(i + 10).unwrap_or(&0);
        new_key[i + 9] = *key.get(i + 14).unwrap_or(&0);
        new_key[i + 10] = *key.get(i + 2).unwrap_or(&0);
        new_key[i + 11] = *key.get(i + 6).unwrap_or(&0);

        new_key[i + 12] = *key.get(i + 7).unwrap_or(&0);
        new_key[i + 13] = *key.get(i + 11).unwrap_or(&0);
        new_key[i + 14] = *key.get(i + 15).unwrap_or(&0);
        new_key[i + 15] = *key.get(i + 3).unwrap_or(&0);
    }

    new_key
}

/// Returns an expanded round key
///
/// # Arguments
/// * `key` - the key to expand
/// * `round` - the round to expand for
fn key_expansion(key: &Vec<u8>, round: usize) -> Vec<u8> {
    let mut new_key = vec![0u8; key.len()];

    let formula = |i: usize| ((i.pow(3) + 3) * i % 0xFF);

    for i in 0..key.len() {
        new_key[i] = SUBSTITUTION_BOX[formula(round + key[i] as usize)];
    }

    shuffle(&mut new_key);

    new_key
}

/// This function encrypts data
///
/// # Arguments
/// * `input` - a mutable BufReader to read from
/// * `input_size` - the size of the input data
/// * `output` - a mutable BufWriter to write to
/// * `key` - the key to encrypt with
/// * `rounds` - the amount of rounds to use
pub fn encrypt(
    input: &mut BufReader<File>,
    input_size: usize,
    output: &mut BufWriter<File>,
    key: &Vec<u8>,
    rounds: usize,
) -> Result<()> {
    let mut keys = Vec::new();

    // precalculate all round keys
    for round in 0..rounds {
        keys.push(key_expansion(key, round + 1));
    }

    let mut bytes_read = 0;
    let mut buf: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

    while bytes_read < input_size {
        let n = input.read(&mut buf)?;
        let chunk_size = std::cmp::min(input_size - bytes_read, CHUNK_SIZE);

        // apply every round key
        for round_key in keys.iter() {
            substitute(&mut buf);
            xor(&mut buf[..chunk_size], &round_key, bytes_read);
        }

        output.write(&buf[..chunk_size])?;

        buf = [0; CHUNK_SIZE];
        bytes_read += n;
    }

    Ok(())
}

/// This function decrypts data
///
/// # Arguments
/// * `input` - a mutable BufReader to read from
/// * `input_size` - the size of the input data
/// * `output` - a mutable BufWriter to write to
/// * `key` - the key to decrypt with
/// * `rounds` - the amount of rounds to use
pub fn decrypt(
    input: &mut BufReader<File>,
    input_size: usize,
    output: &mut BufWriter<File>,
    key: &Vec<u8>,
    rounds: usize,
) -> Result<()> {
    let mut keys = Vec::new();

    for round in 0..rounds {
        keys.push(key_expansion(key, rounds - round));
    }

    let mut bytes_read = 0;
    let mut buf: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

    while bytes_read < input_size {
        let n = input.read(&mut buf)?;
        let chunk_size = std::cmp::min(input_size - bytes_read, CHUNK_SIZE);

        // apply every round key
        for round_key in keys.iter() {
            xor(&mut buf[..chunk_size], round_key, bytes_read);
            invert_substitute(&mut buf);
        }

        output.write(&buf[..chunk_size])?;

        buf = [0; CHUNK_SIZE];
        bytes_read += n;
    }

    Ok(())
}

/// This function generates a new key. It does this by using the StdRng which
/// uses 12 rounds of ChaCha to generate random numbers.
///
/// # Arguments
/// * `size` - the size of the key
pub fn generate_key(size: usize) -> Vec<u8> {
    let mut key = vec![0; size];

    StdRng::from_entropy().fill_bytes(key.as_mut_slice());

    key
}

#[cfg(test)]
mod test {
    use crate::crypto::{invert_substitute, shuffle, substitute, xor};

    #[test]
    fn test_substitution() {
        let mut a = Vec::with_capacity(256);

        for i in 0..256usize {
            a.push(i as u8);
        }

        substitute(&mut a);
        invert_substitute(&mut a);

        for i in 0..256usize {
            assert_eq!(a[i as usize], i as u8, "failed substitution");
        }
    }

    #[test]
    fn test_shuffle() {
        let a = shuffle(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        let b = vec![5, 9, 13, 1, 10, 14, 2, 6, 11, 15, 3, 7, 8, 12, 16, 4];

        assert_eq!(a, b, "shuffle failed");
    }

    #[test]
    fn test_xor() {
        let original = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let mut data = original.clone();
        let key = vec![
            99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114,
        ];

        xor(&mut data, &key, 0);

        assert_ne!(original, data, "xor did nothing");

        xor(&mut data, &key, 0);

        assert_eq!(data, original, "xor did not work correctly");
    }
}
