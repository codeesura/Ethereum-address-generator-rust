use anyhow::Result;
use secp256k1::{rand::rngs::JitterRng, PublicKey, SecretKey, Secp256k1};
use std::time::{SystemTime, UNIX_EPOCH};
use tiny_keccak::keccak256;
use web3::types::Address;
use std::fs::{OpenOptions, File};
use std::io::{Read, Write};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    loop {
        if let Err(e) = run_bot().await {
            println!("An error occurred: {:?}", e);
            continue;
        }
    }
}

async fn run_bot() -> Result<()> {
    // Key pair generation and address calculation code...
    let secp = Secp256k1::new();
    let get_nstime = || -> u64 {
        let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        dur.as_secs() << 30 | dur.subsec_nanos() as u64
    };
    let mut rng = JitterRng::new_with_timer(get_nstime);
    let (secret_key, pub_key) = secp.generate_keypair(&mut rng);
    let public_key_address = |public_key: &PublicKey| -> Address {
        let public_key = public_key.serialize_uncompressed();
        debug_assert_eq!(public_key[0], 0x04);
        let hash = keccak256(&public_key[1..]);
        Address::from_slice(&hash[12..])
    };
    let pub_address = public_key_address(&pub_key);

    // Filtering part...
    let address_string = format!("{:?}", pub_address);
    if address_string.starts_with("0x123456") {
        let json_data = json!({
            "secret_key": format!("{:?}", secret_key),
            "public_key": format!("{:?}", pub_key),
            "address": address_string
        });
        
        // Reads existing data from file
        let mut file = File::open("addresses.json").unwrap_or_else(|_| File::create("addresses.json").unwrap());
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        // Converts existing data to a JSON array
        let mut data: Vec<Value> = serde_json::from_str(&contents).unwrap_or_else(|_| vec![]);
        
        // Adds the new data to the JSON array
        data.push(json_data);
        
        // Writes the data back to the file in updated form
        let mut file = OpenOptions::new().write(true).create(true).open("addresses.json")?;
        writeln!(file, "{}", serde_json::to_string_pretty(&data)?)?;
    }

    Ok(())
}
