use core::fmt::Debug;
use std::fmt;
use std::fmt::Formatter;

use chrono::Utc;
use crypto::{digest::Digest, sha2::*};

use crate::backend::{functions::*, types::Hash, Hashable, Transaction, Transactions};

pub struct Block {
    index: u64,
    timestamp: u64,
    prev: Option<Sha256>,
    nonce: u64,
    difficulty: u128,
    transactions: Transactions,
    hash: Sha256,
    broadcaster_addr: Hash,
}

impl Block {
    pub fn gen_genesis() -> Block {
        let mut initial_tx = Transactions::empty();
        const INITIAL_BALANCE: u64 = 5000;

        let mut init_wallet = Sha256::new();
        init_wallet.input_str("Genesis Block");
        let result_hash = init_wallet.result_str();

        let initial_output = Transaction::new("".to_string(), result_hash.clone(), INITIAL_BALANCE);
        initial_tx.values_mut().push(initial_output);

        Block::mine(0, initial_tx, result_hash)
    }

    pub fn mine(block_idx: u64, transactions: Transactions, broadcaster_addr: Hash) -> Block {
        let mut block = Block::new(block_idx, None, transactions, broadcaster_addr);

        let mut arr: [u8; 32] = hash_array();

        for nonce_target in 0..u64::MAX {
            block.nonce = nonce_target;

            let mut hash_result = block.hash();
            hash_result.result(&mut arr);

            let is_pass = check_difficulty(&arr, &block.difficulty);

            if is_pass {
                block.hash = block.hash();
                println!(
                    "mine attempt success. nonce : {}\n hash : {}",
                    nonce_target,
                    block.hash.result_str()
                );
                break;
            }

            continue;
        }

        block
    }

    fn new(
        index: u64,
        prev_block: Option<Sha256>,
        transaction: Transactions,
        broadcaster_addr: Hash,
    ) -> Block {
        Block {
            index,
            timestamp: Utc::now().timestamp_nanos() as u64,
            prev: prev_block,
            nonce: 0,
            difficulty: 0x000ffffffffffffffffffffffffffff,
            transactions: transaction,
            hash: Sha256::new(),
            broadcaster_addr,
        }
    }

    fn _mine(&mut self) {
        let mut arr: [u8; 32] = hash_array();

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

    pub fn index(&self) -> u64 {
        self.index
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn difficulty(&self) -> u128 {
        self.difficulty
    }

    pub fn current_hash(&self) -> &Sha256 {
        &self.hash
    }

    pub fn transactions(&self) -> &Transactions {
        &self.transactions
    }

    pub fn broadcaster(&self) -> &Hash {
        &self.broadcaster_addr
    }

    pub fn current_hash_str(&self) -> Hash {
        self.hash.clone().result_str()
    }

    pub fn set_prev_block(&mut self, hash: Sha256) {
        self.prev = Some(hash);
    }

    pub fn current_hash_bytes(&mut self) -> [u8; 32] {
        let mut arr: [u8; 32] = hash_array();

        self.hash.result(&mut arr);

        arr
    }

    pub fn prev_hash_str(&self) -> Hash {
        match &self.prev {
            Some(prev_unwrapped) => prev_unwrapped.clone().result_str(),
            None => "".to_string(),
        }
    }

    pub fn prev_hash_bytes(&self) -> [u8; 32] {
        let mut arr: [u8; 32] = hash_array();

        arr = match &self.prev {
            Some(prev_unwrapped) => {
                prev_unwrapped.clone().result(&mut arr);
                arr
            }
            None => arr,
        };

        arr
    }
}

impl Hashable for Block {
    // generate vec of bytes with information given in the Block.
    // hash for self is not included.
    fn bytes(&self) -> Vec<u8> {
        let mut result = vec![];

        result.extend(u64_bytes(&self.index));
        result.extend(u64_bytes(&self.timestamp));

        let prev_has_str = self.prev_hash_str();

        //put if prev exists.
        if prev_has_str != "" {
            result.extend(prev_has_str.as_bytes());
        }

        let tx_vec = self
            .transactions
            .values()
            .iter()
            .flat_map(|x| {
                let mut hash_arr = hash_array();
                x.hash().result(&mut hash_arr);
                hash_arr
            })
            .collect::<Vec<u8>>();

        result.extend(u64_bytes(&self.nonce));
        result.extend(tx_vec);
        result.extend(u128_bytes(&self.difficulty));

        result
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let hash_str = self.hash.clone().result_str();

        write!(
            f,
            "index - {}\n block hash - {},\n timestamp - {},\n transaction - {:?}",
            self.index, hash_str, self.timestamp, self.transactions
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

    // Block::new(0, None, String::from_utf8(letter.to_vec()).unwrap())
    Block::new(0, None, Transactions::empty(), hasher.result_str())
}
