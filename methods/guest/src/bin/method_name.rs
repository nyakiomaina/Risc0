#![no_main]
use hex;
use risc0_zkvm::guest::env;
use sha2::{Digest, Sha256};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let substring: String = env::read();
    let input: String = env::read();
    let committed_hash_hex: String = env::read();

    let input_hash = Sha256::digest(input.as_bytes());
    let input_hash_hex = hex::encode(input_hash);

    let result = input.contains(&substring) && input_hash_hex == committed_hash_hex;

    env::commit(&result);
}
