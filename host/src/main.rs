use risc0_zkvm::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};
use sha2::{Digest, Sha256};

use methods::{METHOD_NAME_ELF, METHOD_NAME_ID};

fn main() {
    let input = "She, called Nyakio, went to the beach";
    let substring = "Nyakio";
    let hash = Sha256::digest(input.as_bytes());
    let hash_hex = hex::encode(hash);

    let mut prover = Prover::new(METHOD_NAME_ELF)
        .expect("Prover should be constructed from valid method source code and corresponding image ID");
    prover.add_input_u32_slice(to_vec(substring).unwrap().as_slice());
    prover.add_input_u32_slice(to_vec(input).unwrap().as_slice());
    prover.add_input_u32_slice(to_vec(&hash_hex).unwrap().as_slice());
    
    let receipt = prover
        .run()
        .expect("Code should be provable unless it had an error or overflowed the maximum cycle count");

    receipt
        .verify(&METHOD_NAME_ID)
        .expect("Code you have proven should successfully verify; did you specify the correct image ID?");

    let result: bool = from_slice(&receipt.journal).unwrap();
    println!("Substring check result: {}", result);
}
