use std::collections::HashSet;

use rand::seq::SliceRandom;
use rand::thread_rng;

// NOTE:  I use <u32> instead of <u8>
//   - 4 byte integer is common type returning from stdlib.  
//   - wasting a little bit of memory, to reduce conversion down (u32->u8)
//


/// Checks if a 3-digit string number is valid for the guessing game.
///
/// A valid number must meet three criteria:
/// 1. It must be exactly 3 characters long.
/// 2. Each character must be a digit from '1' to '9' (no '0').
/// 3. All three digits must be unique.
///
/// # Examples
///
/// * "123" -> true
/// * "012" -> false (contains '0')
/// * "393" -> false (duplicate '3')
/// * "1234" -> false (wrong length)
pub fn is_valid_guess(s: &str) -> bool {
    // Rule 1: Must be exactly 3 characters long.
    if s.len() != 3 {
        return false;
    }

    let mut seen_digits = HashSet::new();

    for c in s.chars() {
        // Rule 2: Check if the character is a digit from '1' to '9'.
        // This implicitly handles non-digit characters too.
        if !('1'..='9').contains(&c) {
            return false;
        }

        // Rule 3: Check for uniqueness.
        // HashSet::insert returns true if the value was newly inserted, false if it was already present.
        if !seen_digits.insert(c) {
            // If insert returns false, the digit is a duplicate.
            return false;
        }
    }

    // If we passed all checks for all characters, the guess is valid.
    true
}

/// Generates a vector containing 3 unique random digits ranging from 1 to 9.
///
/// Your proposed method:
/// 1. Create a source vector [1, 2, ..., 9].
/// 2. Shuffle the vector randomly.
/// 3. Take the first 3 elements (a slice of [0..3]).
pub fn generate_unique_digits() -> Vec<u32> {
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

/// Converts a 3-digit string into a Vector of its individual digits (u32).
///
/// Example: "132" -> [1, 3, 2]
///
/// NOTE: The prompt guarantees the input is "valid checked," so we use `expect`
/// for straightforward conversion, assuming the input only contains valid digits
/// (1-9, as per the previous requirement).
///
/// # Arguments
/// * `input_str` - A reference to the 3-digit string.
///
/// # Returns
/// A `Vec<u32>` containing the three digits.
pub fn string_to_digit_vec(input_str: &str) -> Vec<u32> {
    input_str
        // Iterate over the characters of the string
        .chars()
        // Convert each character digit (e.g., '1') to a numeric digit (e.g., 1)
        .map(|c| {
            // c.to_digit(10) returns an Option<u32>. 
            c.to_digit(10).expect("Input must contain only valid digits 0-9")
        })
        // Collect the results into a new Vector of u32
        .collect()
}


/// Compares two 3-digit slices and calculates the Bulls (i) and Cows (j) scores.
/// This uses the standard "Bulls and Cows" game logic, which correctly handles 
/// the case where a digit should only be counted once (either as a Bull or a Cow).
/// 
/// # Arguments
/// * `guess` - The player's 3-digit guess (slice of u32).
/// * `secret` - The secret 3-digit code (slice of u32).
///
/// # Returns
/// A tuple `(i, j)`:
/// * `i` (Bulls): Count of same digit in the exact same spot.
/// * `j` (Cows): Count of same digit in a different spot.
pub fn compare_digits(guess: &[u32], secret: &[u32]) -> (u32, u32) {
    // For safety, though the prompt implies 3-digit slices.
    if guess.len() != 3 || secret.len() != 3 {
        eprintln!("Warning: Input slices must be 3 elements long.");
        return (0, 0); 
    }

    let mut bulls = 0;
    
    // --- 1. Calculate 'i' (Bulls - Same spot) ---
    // zip() pairs up the elements from both slices for simultaneous iteration.
    for (g, s) in guess.iter().zip(secret.iter()) {
        if g == s {
            bulls += 1;
        }
    }

    // --- 2. Calculate Total Matches ---
    // Use a HashSet for the secret code for fast, non-positional lookup.
    // .cloned() is used because HashSet requires ownership of its elements.
    let secret_set: HashSet<u32> = secret.iter().cloned().collect();
    let mut total_matches = 0;

    // Check how many digits in the guess are present anywhere in the secret code.
    for g in guess.iter() {
        if secret_set.contains(g) {
            total_matches += 1;
        }
    }

    // --- 3. Calculate 'j' (Cows - Different spot) ---
    // Cows = (Total Digits Present) - (Digits in Correct Spot)
    let cows = total_matches - bulls;

    (bulls, cows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_guesses() {
        assert!(is_valid_guess("123"));
        assert!(is_valid_guess("987"));
        assert!(is_valid_guess("456"));
    }

    #[test]
    fn test_invalid_length() {
        assert!(!is_valid_guess("12"));
        assert!(!is_valid_guess("1234"));
        assert!(!is_valid_guess(""));
    }

    #[test]
    fn test_contains_zero() {
        assert!(!is_valid_guess("012"));
        assert!(!is_valid_guess("102"));
        assert!(!is_valid_guess("120"));
    }

    #[test]
    fn test_duplicate_digits() {
        assert!(!is_valid_guess("112"));
        assert!(!is_valid_guess("121"));
        assert!(!is_valid_guess("211"));
        assert!(!is_valid_guess("333"));
    }

    #[test]
    fn test_non_digit_characters() {
        assert!(!is_valid_guess("abc"));
        assert!(!is_valid_guess("12a"));
        assert!(!is_valid_guess("a12"));
        assert!(!is_valid_guess("1 2"));
        assert!(!is_valid_guess("1.2"));
    }
}
