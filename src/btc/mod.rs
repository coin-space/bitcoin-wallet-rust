use std::str::FromStr;

use bitcoin::network::constants::Network;
use bitcoin::secp256k1::ffi::types::AlignedType;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::address::Address;
use bitcoin::util::bip32::ChildNumber;
use bitcoin::util::bip32::DerivationPath;
use bitcoin::util::bip32::ExtendedPrivKey;
use bitcoin::util::bip32::ExtendedPubKey;
use bitcoin::util::ecdsa::PrivateKey;

use bip32::{Mnemonic};
use rand_core::OsRng;

use crate::logger;

pub fn create_new_account() {

    println!("Generate Bitcoin Wallet....");

    let network = Network::Bitcoin;

    // Generate random Mnemonic using the default language (English)
    let mnemonic = Mnemonic::random(&mut OsRng, Default::default());
    let your_password = "Admin123";

    println!("========================== Your password:");
    println!("{}", your_password);
    println!("========================== Phrase random:");
    println!("{}", mnemonic.phrase());

    logger::write("========================== Pharse random: ").unwrap();
    logger::write(mnemonic.phrase()).unwrap();
    logger::write("========================== Password: ").unwrap();
    logger::write(&your_password).unwrap();

    // Derive a BIP39 seed value using the given password
    let seed = mnemonic.to_seed(your_password);
    let seed_bytes = seed.as_bytes();

    // we need secp256k1 context for key derivation
    let mut buf: Vec<AlignedType> = Vec::new();
    buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
    let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

    // Calculate root key from seed
    let root = ExtendedPrivKey::new_master(network, seed_bytes).unwrap();

    // derive child xpub
    let path = DerivationPath::from_str("m/84h/0h/0h").unwrap();

    let child_xpriv = root.derive_priv(&secp, &path).unwrap();
    println!("========================== Child at {}:", path);
    println!("{}", child_xpriv);

    let child_xpub = ExtendedPubKey::from_private(&secp, &child_xpriv);
    println!("========================== Public key at {}:", path);
    println!("{}", child_xpub);

    // generate first receiving address at m/0/0
    // manually creating indexes this time
    let zero = ChildNumber::from_normal_idx(0).unwrap();
    let public_key = child_xpub
        .derive_pub(&secp, &vec![zero, zero])
        .unwrap()
        .public_key;
    let address = Address::p2wpkh(&public_key, network).unwrap();
    println!("========================== First receiving address:");
    println!("{}",address);

    logger::write("========================== First receiving address:").unwrap();
    logger::write(&format!("{}", address)).unwrap();

}

pub fn get_wallet_info_by_wif(wif_priv_str: &str) {

    // This example derives root xprv
    // from a 32-byte secret of the input WIF string,
    // derives the child xprv with path m/84h/0h/0h,
    // prints out corresponding xpub,
    // calculates and prints out the first receiving segwit address.
    // Run this example with cargo and WIF argument:
    // cargo run --example bip32 L1HKVVLHXiUhecWnwFYF6L3shkf1E12HUmuZTESvBXUdx3yqVP1D

    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     eprintln!("not enough arguments. usage: {} <WIF>", &args[0]);
    //     process::exit(1);
    // }

    // let wif_priv_str = "L2UeUiZsLQHQqugraPfTGZboHVSLjyGaa6yYgY6buzmw5buzGQLs";

    let wif = PrivateKey::from_wif(&wif_priv_str).unwrap();
    println!("Seed WIF: {}", wif);

    // use the network from WIF key
    let network = wif.network;
    println!("Network: {:?}", network);
    // seed is a 32-byte secret in WIF
    let seed = wif.to_bytes();

    // we need secp256k1 context for key derivation
    let mut buf: Vec<AlignedType> = Vec::new();
    buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
    let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

    // calculate root key from seed
    let root = ExtendedPrivKey::new_master(network, &seed).unwrap();
    println!("Root key: {}", root);

    // derive child xpub
    let path = DerivationPath::from_str("m/84h/0h/0h").unwrap();
    let child = root.derive_priv(&secp, &path).unwrap();
    println!("Child at {}: {}", path, child);
    let xpub = ExtendedPubKey::from_private(&secp, &child);
    println!("Public key at {}: {}", path, xpub);

    // generate first receiving address at m/0/0
    // manually creating indexes this time
    let zero = ChildNumber::from_normal_idx(0).unwrap();
    let public_key = xpub
        .derive_pub(&secp, &vec![zero, zero])
        .unwrap()
        .public_key;
    let address = Address::p2wpkh(&public_key, network).unwrap();
    println!("First receiving address: {}", address);

}
