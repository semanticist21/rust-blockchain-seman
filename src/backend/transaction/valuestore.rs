use crate::backend::{enums::BlockValidationError, types::Hash, Block};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ValueStore {
    values_store: HashMap<Hash, u64>,
}

impl ValueStore {
    pub fn new() -> Self {
        ValueStore {
            values_store: HashMap::<Hash, u64>::new(),
        }
    }

    pub fn reward_miner(&mut self, block_chain: &Vec<Block>, rewarder: Hash) {
        if let None = block_chain.last() {
            return;
        }

        let last_block = block_chain.last().unwrap();

        if *last_block.broadcaster() == rewarder {
            if let Some(value) = self.values_store.get_mut(&rewarder) {
                *value = *value + 1;
            } else {
                self.values_store.insert(rewarder, 1);
            }
        }
    }

    pub fn insert_genesis_balance(
        &mut self,
        block: &mut Block,
    ) -> Result<(), BlockValidationError> {
        let transactions = block.transactions();

        if transactions.values().len() == 0 || block.index() != 0 {
            return Err(BlockValidationError::InvalidInput);
        }

        let genesis_tx = transactions.values().first().unwrap();
        self.values_store
            .insert(genesis_tx.to_address().to_string(), *genesis_tx.value());

        return Ok(());
    }

    pub fn set_transaction(
        &mut self,
        from_address: &Hash,
        to_address: &Hash,
        value: u64,
    ) -> Result<(), BlockValidationError> {
        if let Ok(()) = self.set_value(from_address, value, true) {
            if let Ok(()) = self.set_value(to_address, value, false) {
                return Ok(());
            }
        }
        // in case of not retruning anything yet.
        return Err(BlockValidationError::InvalidInput);
    }

    fn set_value(
        &mut self,
        key: &Hash,
        value: u64,
        is_sender: bool,
    ) -> Result<(), BlockValidationError> {
        if value == 0 {
            return Ok(());
        }

        let value_wrapped = self.values_store.get_mut(key);

        if let Some(target_value) = value_wrapped {
            // in case sender, we deduct value from the asset.
            if is_sender {
                if *target_value < value {
                    return Err(BlockValidationError::InsufficientInputVal);
                }

                *target_value = *target_value - value;
                return Ok(());
            } else {
                *target_value = *target_value + value;
                return Ok(());
            }
        } else {
            // in case of no wallet 
            if is_sender {
                return Err(BlockValidationError::InvalidInput);
            }

            self.values_store.insert(key.clone(), value);
            return Ok(());
        }
    }
}
