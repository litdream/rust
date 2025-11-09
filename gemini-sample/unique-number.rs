use std::collections::HashSet;

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

fn main() {
    println!("--- Testing Input Validation Function ---");

    let tests = vec![
        ("123", true, "Valid: unique, 1-9, length 3"),
        ("987", true, "Valid: reverse order"),
        ("112", false, "Invalid: duplicate digits ('1')"),
        ("012", false, "Invalid: contains '0'"),
        ("12", false, "Invalid: too short"),
        ("1234", false, "Invalid: too long"),
        ("abc", false, "Invalid: contains non-digits"),
        ("333", false, "Invalid: all duplicates"),
        ("793", true, "Valid: mix of digits"),
    ];

    for (input, expected, description) in tests {
        let result = is_valid_guess(input);
        println!(
            "Test: \"{}\" -> Expected: {} | Got: {} | Status: {} ({})",
            input,
            expected,
            result,
            if result == expected { "PASS" } else { "FAIL" },
            description
        );
    }
}
