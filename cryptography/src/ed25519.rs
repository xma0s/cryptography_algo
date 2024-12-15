use rand::rngs::OsRng;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Signature;
// https://docs.rs/ed25519-dalek/latest/ed25519_dalek/ 
pub fn exec() {
    let mut csprng = OsRng;
    //let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    
}
