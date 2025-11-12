// Dependenc in TOML
// ----
// [dependencies]
// rand = "0.8"

use rand::seq::SliceRandom;
use rand::thread_rng;

/// Generates a vector containing 3 unique random digits ranging from 1 to 9.
///
/// Your proposed method:
/// 1. Create a source vector [1, 2, ..., 9].
/// 2. Shuffle the vector randomly.
/// 3. Take the first 3 elements (a slice of [0..3]).
fn generate_unique_digits() -> Vec<u32> {
    // 1. Create the source vector of digits from 1 to 9.
    let mut digits: Vec<u32> = (1..=9).collect();
    // 

    // Get a thread-local random number generator (RNG).
    let mut rng = thread_rng();

    // 2. Shuffle the vector randomly.
    digits.shuffle(&mut rng);

    // 3. Take a slice of the first 3 elements and convert it back to a Vec<u32>.
    // Using digits[0..3] creates a slice (&[u32]), which is then cloned into a new Vec.
    digits[0..3].to_vec()
}

fn main() {
    let random_set = generate_unique_digits();
    println!("Generated unique digits: {:?}", random_set);
    
    // Example output might be: [4, 9, 2]
}
