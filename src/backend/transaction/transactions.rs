// Check point..
// overspending
// double-spending
// impersonation

use std::collections::HashSet;

use crypto::{digest::Digest, sha2::Sha256};

use crate::backend::{types::Hash, Hashable, Transaction};

#[derive(Debug, Clone)]
pub struct Transactions {
    values: Vec<Transaction>,
}

impl Transactions {
    pub fn empty() -> Transactions {
        Transactions {
            values: vec![],
        }
    }

    pub fn values(&self) -> &Vec<Transaction>{
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut Vec<Transaction>{
        &mut self.values
    }

    pub fn input_sum(&self) -> u64 {
        self.values.iter().map(|output| output.value()).sum()
    }

    // pub fn output_sum(&self) -> u64 {
    //     self.outputs.iter().map(|input| input.value()).sum()
    // }

    pub fn input_hashes(&self) -> HashSet<Hash> {
        self.values
            .iter()
            .map(|input| Self::hash_str(input.hash()))
            .collect::<HashSet<Hash>>()
    }

    // pub fn output_hashes(&self) -> HashSet<Hash> {
    //     self.outputs
    //         .iter()
    //         .map(|output| Self::hash_str(output.hash()))
    //         .collect::<HashSet<Hash>>()
    // }

    pub fn is_input_empty(&self) -> bool {
        self.values.len() == 0
    }

    fn hash_str(mut sha256: Sha256) -> Hash {
        sha256.result_str()
    }
}

impl Hashable for Transactions {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        let vec_input = self
            .values
            .iter()
            .flat_map(|output| output.bytes())
            .collect::<Vec<u8>>();

        // let vec_output = self
        //     .outputs
        //     .iter()
        //     .flat_map(|output| output.bytes())
        //     .collect::<Vec<u8>>();

        bytes.extend(vec_input);
        // bytes.extend(vec_output);

        bytes
    }
}
