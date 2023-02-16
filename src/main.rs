use crypto::{digest::Digest, sha2::Sha256};
use rust_blockchain_seman::backend::{
    get_genesis_hasher, get_hash, Block, BlockChain, Transaction, Transactions,
};

fn main() {
    test_process();
    // test sucessful!
    loop {}
}

fn test_process() {
    // genesis block
    let genesis_block = Block::gen_genesis();
    // println!("{}", genesis_block.timestamp());
    let mut block_chain = BlockChain::new();

    _ = block_chain.update_block(genesis_block);

    let mut broadcaster_hasher = Sha256::new();
    broadcaster_hasher.input_str("broadcaster");

    let _broadcaster = broadcaster_hasher.result_str();

    let mut transactions = Transactions::empty();

    let test_wallet_1 = get_hash("Bob".to_string());
    let test_wallet_2 = get_hash("Tom".to_string());
    let test_wallet_3 = get_hash("John".to_string());

    let tx1 = Transaction::new(get_genesis_hasher(), test_wallet_1.clone(), 50);
    let tx2 = Transaction::new(test_wallet_1.clone(), test_wallet_2.clone(), 50);
    let tx3 = Transaction::new(get_genesis_hasher(), test_wallet_3, 50);

    transactions.values_mut().push(tx1);
    transactions.values_mut().push(tx2);
    transactions.values_mut().push(tx3);

    let new_block = Block::mine(
        block_chain.get_block_size() as u64 + 1,
        transactions,
        broadcaster_hasher.result_str(),
    );

    let err = block_chain.update_block(new_block);

    println!("{:?}", err);
    println!("{:?}", block_chain.get_block_size());
    println!("{:?}", block_chain.value_store());
}
