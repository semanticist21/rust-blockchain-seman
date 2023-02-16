use crate::backend::{types::Address, types::Hash, Hashable};

#[derive(Debug, Clone)]
pub struct Transaction {
    from_address: Address,
    to_address: Address,
    value: u64,
}

impl Transaction {
    pub fn new(from_address: Hash, to_address: Hash, value: u64) -> Self {
        Transaction {
            from_address,
            to_address,
            value,
        }
    }

    pub fn value(&self) -> &u64 {
        &self.value
    }

    pub fn from_address(&self) -> &Address{
        &self.from_address
    }

    pub fn to_address(&self) -> &Address{
        &self.to_address
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut result = vec![];

        result.extend(self.from_address.as_bytes());
        result.extend(self.to_address.as_bytes());
        result.extend(self.value.to_le_bytes());

        result
    }
}
