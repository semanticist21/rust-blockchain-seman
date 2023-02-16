use rust_blockchain_seman::backend::{Transaction, Output, Block, BlockChain};

fn main() {
    test_process();
}

fn test_process(){
    // input should be empty - genesis block
    let transaction_genesis = Transaction{
        inputs: vec![],
        outputs: vec![
            Output::new("Alice".to_string(), 500),
            Output::new("Bob".to_string(), 1000),
        ],
    };

    let tx_vec = vec![transaction_genesis];
    let genesis_block = Block::gen_genesis(tx_vec);


    let mut blockchain = BlockChain::new();
    
    blockchain.update_block(genesis_block)
    .expect("the program panicked.");
    
    // second block
    let transaction_second = Transaction{
        inputs: vec![
            Output::new("Alice".to_string(), 500),
        ],
        outputs: vec![
            Output::new("Alice".to_string(), 450),
            Output::new("Bob".to_string(), 40),
        ],
    };
    let tx_vec = vec![transaction_second];

    let last = blockchain.last().unwrap();

    let block = Block::mine(last.index() + 1, tx_vec);
    blockchain.update_block(block).expect("print error");

}

//dummy code

// let payload = "Genesis Block".to_string();
// let mut block = Block::new(0, None, 0x0000fffffffffffffffffffffffffff, payload);

// let current_hash = block.current_hash_str();
// // println!("{}", current_hash);

// // println!("{:?}", bytes);

// block.mine();

// // print!("{:?}", block);

// for i in 1..=10 {
//     println!("{}", i);
// }
