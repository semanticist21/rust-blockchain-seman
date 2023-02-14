use rust_blockchain_seman::backend::Block;

fn main() {
    let payload = "Genesis Block".to_string();
    let mut block = Block::new(0, None, 0x0000fffffffffffffffffffffffffff, payload);

    let current_hash = block.get_current_hash_str();
    // println!("{}", current_hash);
    let bytes = current_hash.as_bytes();

    // println!("{:?}", bytes);

    block.mine();

    // print!("{:?}", block);
}
