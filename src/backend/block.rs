use core::fmt::Debug;
use std::fmt;
use std::fmt::Formatter;

use chrono::{Timelike, Utc};
use crypto::{digest::Digest, sha2::*};

use super::functions::u32_bytes;
use super::{
    functions::{check_difficulty, u128_bytes, u64_bytes},
    hashable::Hashable,
};

pub struct Block {
    index: u64,
    timestamp: u32,
    prev: Option<Sha256>,
    nonce: u64,
    difficulty: u128,
    payload: String,
    hash: Sha256,
}

impl Block {
    pub fn new(index: u64, prev_block: Option<Sha256>, difficulty: u128, payload: String) -> Block {
        Block {
            index,
            timestamp: Utc::now().nanosecond(),
            prev: prev_block,
            nonce: 0,
            difficulty,
            payload,
            hash: Sha256::new(),
        }
    }

    pub fn mine(&mut self) {
        let mut arr: [u8; 32] = [0; 32];

        for nonce_target in 0..u64::MAX {
            self.nonce = nonce_target;

            let mut hash_result = self.hash();
            hash_result.result(&mut arr);

            let is_pass = check_difficulty(&arr, &self.difficulty);

            if is_pass {
                self.hash = self.hash();
                println!(
                    "mine attempt success. nonce : {}\n hash : {}",
                    nonce_target,
                    self.hash.result_str()
                );
                return;
            }

            continue;
        }
    }

    pub fn get_current_hash_str(&self) -> String {
        self.hash.clone().result_str()
    }

    pub fn get_prev_hash_str(&self) -> String {
        match &self.prev {
            Some(prev_unwrapped) => prev_unwrapped.clone().result_str(),
            None => "".to_string(),
        }
    }
}

impl Hashable for Block {
    // generate vec of bytes with information given in the Block.
    // hash for self is not included.
    fn bytes(&self) -> Vec<u8> {
        let mut result = vec![];

        result.extend(u64_bytes(&self.index));
        result.extend(u32_bytes(&self.timestamp));

        let prev_has_str = self.get_prev_hash_str();

        //put if prev exists.
        if prev_has_str != "" {
            result.extend(prev_has_str.as_bytes());
        }

        result.extend(u64_bytes(&self.nonce));
        result.extend(self.payload.as_bytes());
        result.extend(u128_bytes(&self.difficulty));

        result
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let hash_str = self.hash.clone().result_str();

        write!(
            f,
            "block hash -{},\n timestamp - {},\n payload - {}",
            hash_str, self.timestamp, self.payload
        )
    }
}

// test codes
#[test]
fn test_check_block() {
    let block = _mint_virtual_block();

    println!("{:?}", block);
}

#[test]
fn test_byte_trasnform() {
    let block = _mint_virtual_block();

    let mut hash = block.hash();

    println!("{}", hash.result_str());
}

#[test]
fn test_difficulty_func() {
    use crate::backend::functions::difficulty_bytes_as_u128;

    let block = _mint_virtual_block();
    let bytes_vec = block.bytes();
    let result = difficulty_bytes_as_u128(&bytes_vec);

    println!("{:?}", bytes_vec);
    println!("{}", result);
}

fn _mint_virtual_block() -> Block {
    let letter = b"@Genesis Block@";

    let mut hasher = Sha256::new();
    hasher.input(letter);

    Block::new(0, None, 1, String::from_utf8(letter.to_vec()).unwrap())
}
