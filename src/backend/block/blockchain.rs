use core::fmt::Debug;

use crypto::digest::Digest;

use crate::backend::{functions::*, Block, Hashable};

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain { blocks: vec![] }
    }

    pub fn push(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn is_valid(&mut self) -> bool {
        for (idx, block) in self.blocks.iter_mut().enumerate() {
            // when block is genesis block
            if *block.idx() == 0 {
                if block.prev_hash_str() != "" {
                    println!("It is not genesis block, but its index is 0");
                    return false;
                }

                return true;
            }

            // check index matching
            if *block.idx() != idx as u64 {
                println!("index mismatched block index {} != {}", *block.idx(), idx);
                return false;
            }

            // check hash validity.
            // check current hash validity.
            let mut current_hash = block.hash();
            let is_equal_with_current_hash = current_hash.result_str() == block.current_hash_str();

            let mut out = hash_array();
            current_hash.result(&mut out);

            let is_valid_hash = check_difficulty(&out, block.difficulty());

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
            if block.idx() < prev_block.idx()
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

impl Debug for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Blockchain")
            .field("blocks", &self.blocks)
            .finish()
    }
}

#[test]
fn test_verify() {
    let mut blk_chain = Blockchain::new();

    let mut block = Block::gen_genesis();
    block.mine();

    blk_chain.push(block);

    println!("{}", blk_chain.is_valid());
}

#[test]
fn test_verify_blocks() {
    use crypto::sha2::Sha256;

    let gen_block = Block::gen_genesis();
    let mut last_hash: Sha256 = *gen_block.current_hash();
    let mut blk_chain = Blockchain::new();

    // put genesis block
    blk_chain.push(gen_block);

    let str: &str = "anotehr blcok";

    for i in 1..=10 {
        println!("{}", i);
        let mut block = Block::new(i, Some(last_hash.clone()), str.to_string());
        block.mine();

        last_hash = block.current_hash().clone();
        blk_chain.push(block);
    }

    println!("{:?}", blk_chain);
    println!("{}", blk_chain.is_valid());
}
