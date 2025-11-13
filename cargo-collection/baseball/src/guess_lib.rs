use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

// input:
//   - s:  user input string
// returns:
//   - true/false for validity:
//     - valid:  3 digit, 1-9, and unique each digit
//
pub fn is_valid_guess(s: &str) -> bool {
    if s.len() != 3 {
        return false;
    }

    let mut seen_digits = HashSet::new();
    for c in s.chars() {
        if !('1'..='9').contains(&c) {
            // just curious, why (char*), not simply (char)?
            return false;
        }

        if !seen_digits.insert(c) {
            // conflict digit
            return false;
        }
    }
    true
}

// returns:
//   - 3 digit vector of u32
//     - for a single digit, 1 byte (u8) is enough, but, sometimes conversion is involved.  Use u32 for convenience.
//
pub fn generate_unique_digits() -> Vec<u32> {
    let mut digits: Vec<u32> = (1..=9).collect();
    let mut rng = thread_rng(); // random number generator(RNG)
    digits.shuffle(&mut rng);
    digits[0..3].to_vec()
}

// input:
//   - input_str:  user input string, e.g. "123"
// returns:
//   - converted to Vec<u32>, e.g. [ 1, 2, 3 ]
//
pub fn string_to_digit_vec(input_str: &str) -> Vec<u32> {
    input_str
        .chars()
        .map(|c| {
            c.to_digit(10)
                .expect("Input must contain only valid digits 0-9")
        })
        .collect()
}

// input:
//   - guess:  slice of user guess
//   - secret: slice of computer number
// output:
//   - tuple of (bulls, cows)
//     - bulls:  number matches on the right digit
//     - cows:  number included in a different digit
//
pub fn compare_digits(guess: &[u32], secret: &[u32]) -> (u32, u32) {
    if guess.len() != 3 || secret.len() != 3 {
        eprintln!("Warning: Input slice must be 3 elements long.");
        return (0, 0);
    }

    let mut bulls = 0;
    for (g, s) in guess.iter().zip(secret.iter()) {
        if g == s {
            bulls += 1;
        }
    }

    let secret_set: HashSet<u32> = secret.iter().cloned().collect();
    let mut total_matches = 0;

    // building how many are common:
    for g in guess.iter() {
        if secret_set.contains(g) {
            total_matches += 1;
        }
    }
    let cows = total_matches - bulls;
    (bulls, cows)
}
