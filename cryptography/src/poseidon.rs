/*use curve25519_dalek::scalar::Scalar;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub fn exec() {
    // Initialize Poseidon parameters
    let width = 6; // Number of state elements
    let full_rounds_beginning = 4;
    let full_rounds_end = 4;
    let partial_rounds = 140;

    // Create Poseidon parameters
    let params = PoseidonParams::new(width, full_rounds_beginning, full_rounds_end, partial_rounds);

    // Choose the S-box type (Cube in this case)
    let sbox_type = SboxType::Cube;

    // Generate random inputs for the hash
    let mut rng = StdRng::from_seed([24u8; 32]);
    let xl = Scalar::random(&mut rng); // First input
    let xr = Scalar::random(&mut rng); // Second input

    // Compute the Poseidon 2:1 hash
    let hash = Poseidon_hash_2(xl, xr, &params, &sbox_type);

    println!("Input xl: {:?}", xl);
    println!("Input xr: {:?}", xr);
    println!("Poseidon 2:1 Hash: {:?}", hash);
} */
