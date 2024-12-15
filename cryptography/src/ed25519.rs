use ed25519_dalek::ed25519::signature::{self, Keypair};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng; // Random number generator
use rand::RngCore; // To generate random bytes



pub fn exec() {
    let mut rng = OsRng;
    let mut secret_keys_bytes = [0u8;32];
    rng.fill_bytes(&mut secret_keys_bytes);
    //println!("{:?}", secret_keys_bytes);
    // we're obtaining it from the secret keys bytes
    let signing_key = SigningKey::from_bytes(&secret_keys_bytes);
    // obtaining verifying key from the signing key 
    let verifying_key: VerifyingKey = signing_key.verifying_key();
    println!("Signing key {:?}", signing_key);
    println!("Verifying key {:?}", verifying_key);
    let message: &[u8] = b"This is a test message";
    
    let signature: Signature = signing_key.sign(message);
    println!("Signature r: {:?}", signature.r_bytes());
    println!("Signature s: {:?}", signature.s_bytes());
    println!("Signature : {:?}", signature);
    //print!("Message: {:?}", std::str::from_utf8(message).unwrap());
    //println!("Signature: {:?}", signature);

    // verify signature 

    match verifying_key.verify(message, &signature) {
        Ok(_) => println!("Signature is valid"),
        Err(_) => println!("Signature is invalid"),
    }

}
