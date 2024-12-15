use sha256::digest;

pub fn exec() {
    let input = String::from("hello");
    let val = digest(input);
    println!("Hash for hello: {}", val);
    let result: bool = val == "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
    if result {
        println!("Mathes");
    } else {
        println!("Not matches");
    }
    
    // for the mut string
    let mut input = "justice";
    let val = digest(&mut input);
    //println!("{}", val);
    assert_eq!(val, "468c7b6449dd928f2ddad069fbfdb31f59ce214d67a1422c7610efb886ce6aa7");

    // for char
    let mut input = "∞";
    let val = digest(input);
    println!("{}", val);
    assert_eq!(val, "78d9ce976067aaa5aa9024c17a726c9b121d14abb73ac7644c62c70a6e1a9a5f");

}
