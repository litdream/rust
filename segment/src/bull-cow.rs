use std::collections::HashSet;

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
fn compare_digits(guess: &[u32], secret: &[u32]) -> (u32, u32) {
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

fn main() {
    // The inputs are slices, so we pass references to arrays or vectors.

    // Example 1: [1, 2, 3] vs [4, 1, 2] -> (0 Bulls, 2 Cows)
    // '1' is in wrong spot, '2' is in wrong spot. '3'/'4' are misses.
    let guess1 = [1, 2, 3];
    let secret1 = [4, 1, 2];
    let result1 = compare_digits(&guess1, &secret1);
    println!("Guess: {:?}, Secret: {:?} -> (i, j): {:?}", guess1, secret1, result1); 

    // Example 2: [1, 2, 3] vs [1, 8, 2] -> (1 Bull, 1 Cow)
    // '1' is a Bull. '2' is a Cow. '3'/'8' are misses.
    let guess2 = [1, 2, 3];
    let secret2 = [1, 8, 2];
    let result2 = compare_digits(&guess2, &secret2);
    println!("Guess: {:?}, Secret: {:?} -> (i, j): {:?}", guess2, secret2, result2); 

    // Example 3: No matches
    let guess3 = [7, 7, 7];
    let secret3 = [1, 2, 3];
    let result3 = compare_digits(&guess3, &secret3);
    println!("Guess: {:?}, Secret: {:?} -> (i, j): {:?}", guess3, secret3, result3); // (0, 0)


    // Example 4: No matches
    let guess3 = [1, 2, 3];
    let secret3 = [1, 2, 3];
    let result3 = compare_digits(&guess3, &secret3);
    println!("Guess: {:?}, Secret: {:?} -> (i, j): {:?}", guess3, secret3, result3); // (0, 0)
}

