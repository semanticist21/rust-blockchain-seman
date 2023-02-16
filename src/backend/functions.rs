use crypto::{sha2::Sha256, digest::Digest};

pub const fn u32_bytes(&item: &u32) -> [u8; 4] {
    item.to_le_bytes()
}

pub const fn u64_bytes(&item: &u64) -> [u8; 8] {
    item.to_le_bytes()
}

pub const fn u128_bytes(&item: &u128) -> [u8; 16] {
    item.to_le_bytes()
}

pub const fn _i64_bytes(&item: &i64) -> [u8; 8] {
    item.to_le_bytes()
}

pub const fn hash_array() -> [u8; 32] {
    [0 as u8; 32]
}

// little endian
pub const fn difficulty_bytes_as_u128(v: &[u8]) -> u128 {
    // let new_vec = arr.iter().take(16).copied().collect::<Vec<u8>>();

    // let mut new_arr:[u8;16] = [0;16];
    // new_arr.copy_from_slice(&new_vec);

    // println!("{}",u128::from_le_bytes(new_arr));
    // u128::from_le_bytes(new_arr)

    // println!("{}", v.into_boxed_slice().into());

    // move nums to the left side.
    ((v[31] as u128) << 0xf * 8)
        | ((v[30] as u128) << 0xe * 8)
        | ((v[29] as u128) << 0xd * 8)
        | ((v[28] as u128) << 0xc * 8)
        | ((v[27] as u128) << 0xb * 8)
        | ((v[26] as u128) << 0xa * 8)
        | ((v[25] as u128) << 0x9 * 8)
        | ((v[24] as u128) << 0x8 * 8)
        | ((v[23] as u128) << 0x7 * 8)
        | ((v[22] as u128) << 0x6 * 8)
        | ((v[21] as u128) << 0x5 * 8)
        | ((v[20] as u128) << 0x4 * 8)
        | ((v[19] as u128) << 0x3 * 8)
        | ((v[18] as u128) << 0x2 * 8)
        | ((v[17] as u128) << 0x1 * 8)
        | ((v[16] as u128) << 0x0 * 8)
}

pub const fn check_difficulty(hash_bytes: &[u8], difficulty: &u128) -> bool {
    if hash_bytes.len() != 32 {
        panic!();
    }

    // println!("{}",difficulty_bytes_as_u128(&hash_bytes));
    *difficulty > difficulty_bytes_as_u128(&hash_bytes)
}

pub fn get_genesis_hasher() -> String {
    let mut init_wallet = Sha256::new();
    init_wallet.input_str("Genesis Block");
    let result_hash = init_wallet.result_str();
    result_hash
}

pub fn get_hash(word: String) -> String{
    let mut hasher = Sha256::new();
    hasher.input_str(&word);

    hasher.result_str()
}
