use crypto::{digest::Digest, sha2::Sha256};

use crate::backend::functions::hash_array;


pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> Sha256 {
        let mut hasher = Sha256::new();

        let bytes_items = self.bytes();

        hasher.input(&bytes_items);

        hasher
    }

    fn get_vec(hash: &mut Sha256) -> Vec<u8> {
        let mut arr: [u8; 32] = hash_array();

        hash.result(&mut arr);

        arr.to_vec()
    }
}
