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
