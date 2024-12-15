use blake2::{Blake2b512, Blake2s256, Digest};
use hex_literal::hex;

// read https://gist.github.com/sooryan/8d1b2c19bf0b971c11366b0680908d4b
// code from here https://docs.rs/blake2/latest/blake2/
pub fn exec() {
    // b is 32 bits
    let mut hasher = Blake2b512::new();
    hasher.update(b"hello world");
    let res = hasher.finalize();

    println!("Black2b512 hash: {:x}", res);

    assert_eq!(res[..], hex!("
        021ced8799296ceca557832ab941a50b4a11f83478cf141f51f933f653ab9fbcc05a037cddbed06e309bf334942c4e58cdf1a46e237911ccd7fcf9787cbc7fd0
    ")[..]);
    // same example for Blacke2s256

    // s is 64 bits
    let mut hasher = Blake2s256::new();
    hasher.update(b"hello world");
    let res = hasher.finalize();

    assert_eq!(res[..], hex!("9aec6806794561107e594b1f6a8a6b0c92a0cba9acf5e5e93cca06f781813b0b")[..]);
    //assert_eq!(res[..], hex(""))
}
