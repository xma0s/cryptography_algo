mod secp256;
mod ed25519;
mod sha256;
mod sha3;
mod blake2;
mod poseidon;
fn main() {
    //secp256::exec();
    //ed25519::exec();
    //sha256::exec();
    //sha3::exec();
    //blake2::exec();
    poseidon::exec();
}
