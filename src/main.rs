use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str;
use colored::*;
use hex::decode;
use md5::{compute, Digest};

// use sha256::digest_bytes;

mod init;



fn md5(p: &str) -> Digest {
    let b = p.as_bytes();
    let digest = compute(b);
    digest
}

// fn sha256(p: &str) {
//     let b = p.as_bytes();
//     let digest = digest_bytes(b);
//     println!("{}", digest)
// }

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

fn string_to_decimal(s: &str) -> Vec<u8> {
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
    let input = Digest(vec_to_array(string_to_decimal(&args[1])));
    init::print_seq("🦝 Parsing md5 hash");
    let mut hashes: HashMap<Digest, String> = HashMap::new();
    let rockyou = File::open("rockyou.txt")?;
    let reader = BufReader::new(rockyou);
    init::print_seq("🦝 Loading rockyou wordlist");
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
    init::print_seq("🦝 Exiting hashraccoon");
    Ok(())
}
