use crypto::{sha2::Sha256, digest::Digest};

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> Sha256{
        let mut hasher = Sha256::new();

        let bytes_items = self.bytes();

        hasher.input(&bytes_items);

        hasher
    }
}
