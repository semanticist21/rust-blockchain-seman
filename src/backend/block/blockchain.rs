use core::fmt::Debug;
use std::collections::HashSet;

use crypto::digest::Digest;

use crate::backend::types::Hash;
use crate::backend::{enums::BlockValidationError, functions::*, Block, Hashable};

pub struct BlockChain {
    blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
}

impl BlockChain {
    pub fn new() -> BlockChain {
        BlockChain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }

    pub fn last(&self) -> Option<&Block>{
        if self.blocks.len() == 0{
            return None
        }

        Some(self.blocks.last().unwrap())
    }

    pub fn update_block(&mut self, mut block: Block) -> Result<(), BlockValidationError> {
        // verifying blocks
        // in case of genesis
        if self.blocks.len() == 0 {
            if block.index() != 0 {
                return Err(BlockValidationError::InvalidGenesisBlock);
            }

            if block.prev_hash_str() != "" {
                return Err(BlockValidationError::InvalidGenesisBlock);
            }

            self.blocks.push(block);
            return Ok(());
        }

        let last_item = self.blocks.last_mut().unwrap();

        if last_item.index() >= block.index() {
            return Err(BlockValidationError::MismatchedIndex);
        }

        if !check_difficulty(&block.current_hash_bytes(), &block.difficulty()) {
            return Err(BlockValidationError::InvalidHash);
        }

        if let Some((first_tx, transactions)) = block.transactions().split_first() {
            // verify whether input balance 0
            if !first_tx.is_coinbase() {
                return Err(BlockValidationError::InvalidConinbaseTransaction);
            }

            let mut block_spent = HashSet::new();
            let mut block_created = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();
                let output_hashes = transaction.output_hashes();

                let is_empty = (&input_hashes - &self.unspent_outputs).is_empty();
                let is_identical_emtpy = (&input_hashes - &self.unspent_outputs).is_empty();

                if !is_empty || !is_identical_emtpy {
                    return Err(BlockValidationError::InvalidInput);
                }

                let input_val = transaction.input_value();
                let output_val = transaction.output_value();

                if output_val > input_val {
                    return Err(BlockValidationError::InsufficientInputVal);
                }

                total_fee += input_val - output_val;
                block_spent.extend(input_hashes);
                block_created.extend(output_hashes);
            }

            if first_tx.output_value() < total_fee {
                return Err(BlockValidationError::InvalidConinbaseTransaction);
            }

            // include first.
            block_created.extend(first_tx.output_hashes());

            self.unspent_outputs
                .retain(|output| !block_spent.contains(output));

            self.unspent_outputs.extend(block_created);

            // put block to blockchain
            let last_block = self.blocks.last_mut().unwrap().current_hash();
            block.set_prev_block(*last_block);
            self.blocks.push(block);
        }
        //

        Ok(())
    }

    fn _push(&mut self, block: Block) {
        self.blocks.push(block);
    }

    //todo
    fn _is_valid(&mut self) -> bool {
        for (idx, block) in self.blocks.iter_mut().enumerate() {
            // when block is genesis block
            if block.index() == 0 {
                if block.prev_hash_str() != "" {
                    println!("It is not genesis block, but its index is 0");
                    return false;
                }

                return true;
            }

            // check index matching
            if block.index() != idx as u64 {
                println!("index mismatched block index {} != {}", block.index(), idx);
                return false;
            }

            // check hash validity.
            // check current hash validity.
            let mut current_hash = block.hash();
            let is_equal_with_current_hash = current_hash.result_str() == block.current_hash_str();

            let mut out = hash_array();
            current_hash.result(&mut out);

            let is_valid_hash = check_difficulty(&out, &block.difficulty());

            if !is_equal_with_current_hash || !is_valid_hash {
                println!("It is not a valid hash. Calculating Hash result is different or difficulty tests failed.");
                return false;
            }
        }

        //check linearity
        for (idx, block) in self.blocks.iter().enumerate() {
            let prev_block = self.blocks.get(idx).unwrap();

            // check hash validity.
            // check validity of previous hash & previous hash info in current block .

            let is_valid_prev_info = block.prev_hash_str() == prev_block.hash().result_str();
            if !is_valid_prev_info {
                println!("It is not a preious valid hash. Calculating Hash result is different or difficulty tests failed.");
                return false;
            }

            // check index, stamp linearity and hash string.
            if block.index() < prev_block.index()
                || block.timestamp() < prev_block.timestamp()
                || block.prev_hash_str() != prev_block.current_hash_str()
            {
                println!("It contains invalid data.");
                return false;
            }
        }

        true
    }
}

impl Debug for BlockChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Blockchain")
            .field("blocks", &self.blocks)
            .finish()
    }
}

#[test]
fn test_verify() {
    let mut blk_chain = BlockChain::new();

    let block = Block::mine(1, vec![]);
    blk_chain._push(block);

    // println!("{}", blk_chain._is_valid());
}

#[test]
fn test_verify_blocks() {
    use crypto::sha2::Sha256;

    let gen_block = Block::new(0, None, vec![]);
    let mut last_hash: Sha256 = *gen_block.current_hash();
    let mut blk_chain = BlockChain::new();

    // put genesis block
    blk_chain._push(gen_block);

    // let str: &str = "anotehr blcok";

    for i in 1..=10 {
        println!("{}", i);
        // let mut block = Block::new(i, Some(last_hash.clone()), vec![]);
        let block = Block::mine(1, vec![]);

        last_hash = block.current_hash().clone();
        blk_chain._push(block);
    }

    println!("{:?}", last_hash.result_str());
    println!("{:?}", blk_chain);
    println!("{}", blk_chain._is_valid());
}

#[test]
fn test_derefence() {
    let str_a = String::new();
    let _str_b = &str_a;
}
