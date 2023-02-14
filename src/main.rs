use rust_blockchain_seman::backend::Block;

fn main() {
    let payload = "Genesis Block".to_string();
    let mut block = Block::new(0, None, 0x0000fffffffffffffffffffffffffff, payload);

    let current_hash = block.current_hash_str();
    // println!("{}", current_hash);

    // println!("{:?}", bytes);

    block.mine();

    // print!("{:?}", block);

    for i in 1..=10 {
        println!("{}", i);
    }
}
