use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, Message};
use secp256k1::hashes::{sha256, Hash};
// refer to that library for usage 
// https://docs.rs/secp256k1/latest/secp256k1/
pub fn exec() {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

    //println!("Secret key: {:?}", secret_key.display_secret());
    //println!("Public key: {:?}", public_key);

    let digest = sha256::Hash::hash("hello world".as_bytes());
    println!("Digest: {:?}", digest);
    let message = Message::from_digest(digest.to_byte_array());

    let sig = secp.sign_ecdsa(&message, &secret_key);
    assert!(secp.verify_ecdsa(&message, &sig, &public_key).is_ok());
    println!("Signature verified successfully!");

}
