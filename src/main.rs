use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str;
use colored::*;
use hex::decode;
use md5::{compute, Digest};
use sha256::digest_bytes;

mod init;

fn md5(p: &str) -> Digest {
    let b = p.as_bytes();
    let digest = compute(b);
    digest
}

fn md5_search(data: &String, reader: BufReader<File>) {
    init::print_seq("🦝 Parsing md5 hash");
    let input = Digest(vec_to_array(hex_to_decimal(data)));
    let mut hashes: HashMap<Digest, String> = HashMap::new();
    init::print_seq("🦝 Hashing wordlist with md5 algorithm");
    for line in reader.lines() {
        let hash = md5(&line.as_ref().unwrap());
        hashes.insert(hash, line.unwrap());
    }
    init::print_seq("🦝 Searching HashMap for password...");
    match hashes.get(&input) {
        Some(value) => println!("🦝 Password found: <{}>", value.green().bold()),
        None => println!("Password not found...")
    }
}

fn sha256(p: &str) -> String {
    let b = p.as_bytes();
    let digest = digest_bytes(b);
    digest
}

fn sha256_search(input: &String, reader: BufReader<File>) {
    init::print_seq("🦝 Parsing sha256 hash");
    let mut hashes: HashMap<String, String> = HashMap::new();
    init::print_seq("🦝 Hashing wordlist with sha256 algorithm");
    for line in reader.lines() {
        let hash = sha256(&line.as_ref().unwrap());
        hashes.insert(hash, line.unwrap());
    }
    init::print_seq("🦝 Searching HashMap for password...");
    match hashes.get(input) {
        Some(value) => println!("🦝 Password found: <{}>", value.green().bold()),
        None => println!("Password not found...")
    }
}

// fn decimal_to_string(d: Digest) -> String {
//     let v = d.0;
//     let mut result = vec![];
//     for d in v {
//         let mut r = format!("{:x}", d);
//         result.push(r);
//     }
//     let hex_string = result.join("");
//     hex_string
// }

fn hex_to_decimal(s: &str) -> Vec<u8> {
    let decimal = decode(s).unwrap();
    decimal
}

fn vec_to_array<T>(v: Vec<T>) -> [T; 16] where T: Copy {
    let slice = v.as_slice();
    let array: [T; 16] = match slice.try_into() {
        Ok(ba) => ba,
        Err(_) => panic!("Expected a vec of length {} but it was {}", 16, v.len()),
    };
    array
}

fn main() -> io::Result<()> {
    init::initialize();
    let args: Vec<String> = env::args().collect();
    let rockyou = File::open("rockyou.txt")?;
    let reader = BufReader::new(rockyou);
    init::print_seq("🦝 Loading rockyou wordlist");
    match args[1].as_str() {
        "sha256" => sha256_search(&args[2], reader),
        "md5" => md5_search(&args[2], reader),
        _ => panic!("Error"),
    }
    
    init::print_seq("🦝 Exiting hashraccoon");
    Ok(())
}