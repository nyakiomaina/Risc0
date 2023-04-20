use ring::digest::{Context, Digest, SHA256};
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    print!("Enter the string to search: ");
    io::stdout().flush().unwrap();
    let search_string = iterator.next().unwrap().unwrap();

    print!("Enter the string to commit: ");
    io::stdout().flush().unwrap();
    let commit_string = iterator.next().unwrap().unwrap();

    let found = commit_string.contains(&search_string);

    let digest = sha256(&commit_string);

    println!(
        "SHA256 hash of commit string: {}\nFound search string: {}",
        digest, found
    );
}

fn sha256(data: &str) -> String {
    let mut context = Context::new(&SHA256);
    context.update(data.as_bytes());
    let digest = context.finish();

    to_hex_string(digest)
}

fn to_hex_string(digest: Digest) -> String {
    digest
        .as_ref()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<String>>()
        .join("")
}
