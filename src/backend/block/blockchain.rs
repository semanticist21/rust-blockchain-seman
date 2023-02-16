use core::fmt::Debug;
use std::collections::VecDeque;

use crypto::digest::Digest;

use crate::backend::types::Hash;
use crate::backend::{enums::BlockValidationError, functions::*, Block, Hashable};
use crate::backend::{Transaction, ValueStore};

pub struct BlockChain {
    blocks: Vec<Block>,
    value_store: ValueStore,
}

// Blockchain - contains block infos, and values with address inside.
// condition - gives 1 new coin to the miner.

impl BlockChain {
    pub fn new() -> BlockChain {
        BlockChain {
            blocks: vec![],
            value_store: ValueStore::new(),
        }
    }

    pub fn last(&self) -> Option<&Block> {
        if self.blocks.len() == 0 {
            return None;
        }

        Some(self.blocks.last().unwrap())
    }

    pub fn get_block_size(&self) -> usize{
        self.blocks.len()
    }

    pub fn value_store(&self) -> &ValueStore {
        &self.value_store
    }

    fn _push(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn update_block(&mut self, mut new_block: Block) -> Result<(), BlockValidationError> {
        // if blockchain is empty
        if self.blocks.len() == 0 {
            // check genesis block
            if new_block.index() != 0 {
                println!("Please check index of genesis block.");
                return Err(BlockValidationError::MismatchedIndex);
            }

            let result =
                check_difficulty(&mut new_block.current_hash_bytes(), &new_block.difficulty());

            if !result {
                return Err(BlockValidationError::InvalidHash);
            }

            // transact one transaction on genesis block
            // apply exception to this.
            if let Ok(()) = self.value_store.insert_genesis_balance(&mut new_block) {
                self.blocks.push(new_block);
                println!("Successful Genesis Block inserting!");
                
                return Ok(());
            } else {
                return Err(BlockValidationError::InvalidInput);
            }
        }

        // normal blokchain update.

        if self.blocks.len() >= new_block.index() as usize {
            return Err(BlockValidationError::MismatchedIndex);
        }

        let last_block = self.blocks.last().unwrap();

        let is_valid_hash =
            check_difficulty(&new_block.current_hash_bytes(), &last_block.difficulty());
        if !is_valid_hash {
            return Err(BlockValidationError::InvalidHash);
        }

        if last_block.timestamp() >= new_block.timestamp() {
            println!("genesis_block - {}, other - {}", last_block.timestamp(), new_block.timestamp());
            return Err(BlockValidationError::AchronologicalTimestamp);
        }

        let mut processed_txs = VecDeque::new();

        // insert Block data
        for tx in new_block.transactions().values().iter() {
            if let Ok(()) = self.insert_value_to_balance(tx) {
                processed_txs.push_back(tx);
            } else {
                //rollback txs
                while !processed_txs.is_empty() {
                    let tx_done = processed_txs.pop_front().unwrap();
                    _ = self.set_transaction(tx_done.to_address(), tx_done.from_address(), *tx.value());
                }
                //exit the function.
                return Err(BlockValidationError::InvalidConinbaseTransaction);
            }
        }

        let broadcaster = new_block.broadcaster().clone();
        new_block.set_prev_block(*self.blocks.last().unwrap().current_hash());
        self.blocks.push(new_block);

        // add miners reward.
        self.value_store.reward_miner(&self.blocks, broadcaster);

        return Ok(());
    }

    fn insert_value_to_balance(&mut self, tx: &Transaction) -> Result<(), BlockValidationError> {
        // should not be applied without genesis block.
        if self.blocks.len() == 0 {
            return Err(BlockValidationError::InvalidInput);
        }

        let from_adr = tx.from_address();
        let to_adr = tx.to_address();

        let value = tx.value();

        if let Err(e) = self.set_transaction(from_adr, to_adr, *value) {
            return Err(e);
        } else {
            return Ok(());
        }
    }

    pub fn set_transaction(
        &mut self,
        from_address: &Hash,
        to_address: &Hash,
        value: u64,
    ) -> Result<(), BlockValidationError> {
        self.value_store
            .set_transaction(from_address, to_address, value)
    }

    // todo
    // not used currently.
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

        /*
        pub fn update_block(&mut self, mut block: Block) -> Result<(), BlockValidationError> {
            // only will verify input, output not the ownership.
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

            // check general requirements
            let last_item = self.blocks.last_mut().unwrap();

            if last_item.index() >= block.index() {
                return Err(BlockValidationError::MismatchedIndex);
            }

            if last_item.timestamp() >= block.timestamp() {
                return Err(BlockValidationError::AchronologicalTimestamp);
            }

            if !check_difficulty(&block.current_hash_bytes(), &block.difficulty()) {
                return Err(BlockValidationError::InvalidHash);
            }

            // check transaction
            if let Some((first_tx, transactions)) = block.transactions().split_first() {
                // verify whether input balance 0

                // in case of input being empty
                if first_tx.is_coinbase() {
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

                    let input_val = transaction.input_sum();
                    let output_val = transaction.output_sum();

                    if output_val > input_val {
                        return Err(BlockValidationError::InsufficientInputVal);
                    }

                    total_fee += input_val - output_val;
                    block_spent.extend(input_hashes);
                    block_created.extend(output_hashes);
                }

                if first_tx.output_sum() < total_fee {
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
        */

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
    use crate::backend::Transactions;
    use crypto::sha2::Sha256;

    let mut blk_chain = BlockChain::new();

    let block = Block::mine(1, Transactions::empty(), Sha256::new().result_str());
    blk_chain._push(block);

    // println!("{}", blk_chain._is_valid());
}

#[test]
fn test_verify_blocks() {
    use crate::backend::Transactions;
    use crypto::sha2::Sha256;

    let gen_block = Block::gen_genesis();
    let mut last_hash: Sha256 = *gen_block.current_hash();
    let mut blk_chain = BlockChain::new();

    // put genesis block
    blk_chain._push(gen_block);

    // let str: &str = "anotehr blcok";

    // let mut last_block = None;
    let mut hasher = Sha256::new();
    hasher.input_str("Broadcaster");

    for i in 1..=10 {
        println!("{}", i);
        // let mut block = Block::new(i, Some(last_hash.clone()), vec![]);
        let block = Block::mine(1, Transactions::empty(), hasher.result_str());
        // last_block = Some(block.current_hash().clone());

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
