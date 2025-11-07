use serde_json::Value;
use sha2::{Digest, Sha256};

fn main() {
    let json_str1 = r#"{"a":1,"nested":{"x":10,"y":20},"array":[1,2,3],"b":2}"#;
    let json_str2 = r#"{"b":2,"array":[1,2,3],"nested":{"y":20,"x":10},"a":1}"#;

    let parsed1: Value = serde_json::from_str(json_str1).unwrap();
    let parsed2: Value = serde_json::from_str(json_str2).unwrap();

    println!("Original");
    println!("String 1: {}", json_str1);
    println!("String 2: {}", json_str2);
    println!();

    let bytes1 = serde_json::to_vec(&parsed1).unwrap();
    let bytes2 = serde_json::to_vec(&parsed2).unwrap();

    let str1 = String::from_utf8(bytes1.clone()).unwrap();
    let str2 = String::from_utf8(bytes2.clone()).unwrap();

    println!("to_vec");
    println!("Result 1: {}", str1);
    println!("Result 2: {}", str2);
    println!();

    let hash1 = Sha256::digest(&bytes1);
    let hash2 = Sha256::digest(&bytes2);

    println!("SHA256 digests");
    println!("Hash1: {:x}", hash1);
    println!("Hash2: {:x}", hash2);
}
