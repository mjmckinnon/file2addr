extern crate bitcoin;
extern crate secp256k1;
extern crate rand;
extern crate crypto;
extern crate hex;

use bitcoin::util::address::{Address,Privkey};
use secp256k1::Secp256k1;
use secp256k1::key::SecretKey;
use bitcoin::network::constants::Network;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::error::Error;

fn str2address(s: &String) -> Address {
	// Init our Sha256 hasher
	let mut hasher = Sha256::new();
	hasher.input_str(&s);
	let mut newhash: [u8; 32] = [0; 32];
	hasher.result(&mut newhash);
	let secp = Secp256k1::new();
	let mysecret = match SecretKey::from_slice(&secp, &newhash[0..32]) {
		Ok(key) => { key },
		Err(e) => { panic!("{}", e) }
	};
	let myprivkey = Privkey::from_key(Network::Bitcoin, mysecret, false);
	// Returning our address
	myprivkey.to_address(&secp).unwrap()
}

fn main() {
	// Parse command line arguments, we want
	// filename
	let args: Vec<String> = env::args().collect();
	let path = match args.len() {
		2 => { Path::new(&args[1]) },
		_ => panic!("Error. Usage: file2addr <importfile>")
	};
	// Open the file
	let display = path.display();
	let file = match File::open(&path) {
		Err(why) => panic!("coudn't open {}: {}", display, why.description()),
		Ok(file) => file
	};
	// Read the file in
	let buf = BufReader::new(&file);
	for (num,line) in buf.lines().enumerate() {
		let l = line.unwrap();
		let myaddress = str2address(&l);
		println!("{:?},{},s2({})", myaddress, num, l);
	}
}