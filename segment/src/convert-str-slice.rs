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
fn string_to_digit_vec(input_str: &str) -> Vec<u32> {
    input_str
        // Iterate over the characters of the string
        .chars()
        // Convert each character digit (e.g., '1') to a numeric digit (e.g., 1)
        .map(|c| {
            // c.to_digit(10) returns an Option<u32>. Since we are guaranteed 
            // valid digits, we can safely unwrap and cast to u32.
            c.to_digit(10).expect("Input must contain only valid digits 0-9") as u32
        })
        // Collect the results into a new Vector of u32
        .collect()
}

fn main() {
    let input = "592";
    let digit_vector = string_to_digit_vec(input);

    println!("Input string: \"{}\"", input);
    println!("Output digit vector: {:?}", digit_vector); // Output: [5, 9, 2]

    // Example with the previous function's output structure
    let random_set = vec![4, 9, 2];
    let random_str: String = random_set.iter().map(|d| d.to_string()).collect();

    let random_digits = string_to_digit_vec(&random_str);
    println!("\nTest with converted random set:");
    println!("Original set: {:?}", random_set);
    println!("Converted string: \"{}\"", random_str);
    println!("Vector from string: {:?}", random_digits);
}
