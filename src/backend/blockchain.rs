use crypto::digest::Digest;

use super::{hashable::Hashable, Block, functions::check_difficulty};

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn push(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn is_valid(&mut self) -> bool {
        for (idx, block) in &mut self.blocks.iter().enumerate() {
            // when block is genesis block
            if idx == 0 {
                continue;
            }

            // check index matching
            if *block.idx() != idx as u64{
                println!("index mismatched");
                return false;
            }

            let arr = block.clone();


            // let is_valid_hash = check_difficulty(&arr, block.difficulty());

            // if !is_valid_hash{
            //     return false;
            // }

            let prev_block = self.blocks.get(idx - 1).unwrap();

            // check index, stamp linearity and hash string. 
            if block.idx() < prev_block.idx()
                || block.timestamp() < prev_block.timestamp()
                || block.prev_hash_str() != prev_block.current_hash_str()
            {
                return false;
            }
        }

        true
    }
}
