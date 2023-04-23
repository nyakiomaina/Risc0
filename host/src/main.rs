use risc0_zkvm::guest::GuestInput;
use risc0_zkvm::prover::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};
use sha2::{Digest, Sha256};
use methods::{CHECK_SUBSTRING_ELF, CHECK_SUBSTRING_ID};

#![no_std]
#![feature(lang_items)]

#[lang = "panic_impl"]
#[no_mangle]
pub fn panic_impl(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}


fn main() {
    let input = "Hello rust world";
    let substring = "hello";
    let hash = Sha256::digest(input.as_bytes());
    let hash_hex = hex::encode(hash);

    let mut prover = Prover::new(CHECK_SUBSTRING_ELF)
        .expect("Prover should be constructed from valid method source code and corresponding image ID");

    prover.add_input_string(&substring);
    prover.add_input_string(&input);
    prover.add_input_string(&hash_hex);

    let receipt = prover
        .run()
        .expect("Code should be provable unless it had an error or overflowed the maximum cycle count");

    receipt
        .verify(CHECK_SUBSTRING_ID)
        .expect("Code you have proven should successfully verify; did you specify the correct image ID?");

    let result: bool = from_slice(&receipt.journal).unwrap();
    println!("Substring check result: {}", result);
}
