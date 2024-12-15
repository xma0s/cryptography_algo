use hex_literal::hex;
use sha3::{Digest, Sha3_256};

pub fn exec() {
    let mut hasher = Sha3_256::new();

    hasher.update(b"abc");

    //println!("{:?}", &hasher.clone().decompose());

    let result = hasher.finalize();

    println!("{:?}",result);


    let arr = hex!("3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532");

    println!("{:?}", arr);

    assert_eq!(result[..], hex!("3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532")[..])
}
