use crate::backend::{Hashable, types::Address, types::Hash};

#[derive(Debug, Clone)]
pub struct Output{
    to_address: Address,
    value: u64
}

impl Output{
    pub fn new(to_address: Hash, value: u64) -> Self{
        Output { to_address, value }
    }

    pub fn value(&self) -> u64{
        self.value
    }
}

impl Hashable for Output{
    fn bytes(&self) -> Vec<u8> {
        let mut result = vec![];

        result.extend(self.to_address.as_bytes());
        result.extend(self.value.to_le_bytes());
        
        result
    }
}