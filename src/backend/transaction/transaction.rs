// Check point..
// overspending
// double-spending
// impersonation

use std::collections::HashSet;

use crypto::{digest::Digest, sha2::Sha256};

use crate::backend::{types::Hash, Hashable, Output};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

impl Transaction {
    pub fn empty() -> Transaction {
        Transaction {
            inputs: vec![],
            outputs: vec![],
        }
    }

    pub fn input_value(&self) -> u64 {
        self.inputs.iter().map(|output| output.value()).sum()
    }

    pub fn output_value(&self) -> u64 {
        self.outputs.iter().map(|input| input.value()).sum()
    }

    pub fn input_hashes(&self) -> HashSet<Hash> {
        self.outputs
            .iter()
            .map(|input| Self::hash_str(input.hash()))
            .collect::<HashSet<Hash>>()
    }

    pub fn output_hashes(&self) -> HashSet<Hash> {
        self.outputs
            .iter()
            .map(|output| Self::hash_str(output.hash()))
            .collect::<HashSet<Hash>>()
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }

    fn hash_str(mut sha256: Sha256) -> Hash {
        sha256.result_str()
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        let vec_input = self
            .inputs
            .iter()
            .flat_map(|output| output.bytes())
            .collect::<Vec<u8>>();

        let vec_output = self
            .outputs
            .iter()
            .flat_map(|output| output.bytes())
            .collect::<Vec<u8>>();

        bytes.extend(vec_input);
        bytes.extend(vec_output);

        bytes
    }
}
