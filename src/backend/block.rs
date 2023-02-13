use core::fmt::Debug;
use std::fmt;
use std::fmt::Formatter;

use chrono::{DateTime, Utc};
use crypto::{digest::Digest, sha2::*};

use crate::*;

use super::hashable::Hashable;

pub struct Block {
    index: u64,
    timestamp: DateTime<Utc>,
    prev: Option<Sha256>,
    nonce: u64,
    difficulty: u128,
    payload: String,
    hash: Sha256,
}

impl Block {
    pub fn new(
        index: u64,
        prev_block: Option<Sha256>,
        nonce: u64,
        difficulty: u128,
        payload: String,
    ) -> Block {
        Block {
            index,
            timestamp: Utc::now(),
            prev: prev_block,
            nonce,
            difficulty,
            payload,
            hash: Sha256::new(),
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
    // generate vec of bytes with information given in the Block
    fn bytes(&self) -> Vec<u8> {
        let mut result = vec![];

        result.extend(u64_bytes(&self.index));
        result.extend(i64_bytes(&self.timestamp.timestamp()));

        let prev_has_str = self.get_prev_hash_str();

        //put if prev exists.
        if prev_has_str != "" {
            result.extend(prev_has_str.as_bytes());
        }

        result.extend(u64_bytes(&self.nonce));
        result.extend(self.payload.as_bytes());

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
fn test_print_timestamps() {
    let block = _mint_virtual_block();

    let a = block.timestamp.to_rfc2822();
    println!("{}", a);
    let b = block.timestamp.to_string();
    println!("{}", b);
    let c = block.timestamp.timestamp_subsec_micros();
    println!("{}", c);
    let d = block.timestamp.timestamp();
    println!("{}", d);
}

#[test]
fn test_byte_trasnform() {
    let block = _mint_virtual_block();

    let mut hash = block.hash();

    println!("{}", hash.result_str());
}

#[test]
fn test_difficulty_func() {
    let block = _mint_virtual_block();
    let bytes_vec = block.bytes();
    let result = difficulty_bytes_as_u128(&bytes_vec);

    println!("{}", result);
}

fn _mint_virtual_block() -> Block {
    let letter = b"@Genesis Block@";

    let mut hasher = Sha256::new();
    hasher.input(letter);

    Block::new(0, None, 1, 1, String::from_utf8(letter.to_vec()).unwrap())
}
