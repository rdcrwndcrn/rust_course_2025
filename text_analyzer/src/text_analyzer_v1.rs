use std::io;

pub fn slice_to_words() {
    // Get user input
    let input = get_input();

    // Initialize variables for text processing
    let mut last = 0;
    let mut now;

    // Main loop to process each word
    loop {
        // Find the next word boundary starting from the 'last' position
        now = slicer(&input[last..]);

        // Extract the current word slice and trim whitespace
        let slice = &input[last..last + now].trim();

        // Move the position forward for the next iteration
        last += now;

        // Break if no more characters to process
        if now == 0 {
            break;
        }

        // Print word information: text, character count, vowel count
        println!("slice: {}, length: {}, nouns: {}", slice, count_chars(slice), count_vowels(slice));
    }
}

fn get_input() -> String  {
    let mut input = String::new();
    let read_result = io::stdin().read_line(&mut input);
    
    match read_result {
        Ok(_) => input.trim().to_string(),
        Err(_) => get_input()
    }
}

// find the next word boundary (space or end of string)
fn slicer(slice: &str) -> usize {
    // Iterate through characters with their indices using enumerate()
    for (i, c) in slice.chars().enumerate() {
        // Return position after the first space found
        if  c == ' ' {
            return i + 1;
        }
    }
    // If no space found, return the entire length
    slice.len()
}

// count characters in a string (excluding whitespace)
fn count_chars (slice: &str) -> usize {
    slice.trim().len()
}

// Function to count vowels in a string (including German umlauts)
// Note: Function name is misleading - it counts vowels, not nouns
fn count_vowels (slice: &str) -> usize {
    let mut count = 0;
    // converting all characters to lowercase to handle mixed case input
    let lower_slice = slice.to_lowercase();

    // Iterate through each character
    for c in lower_slice.chars() {
        // Check if a character is a vowel (including German umlauts)
        if ['a', 'e', 'i', 'o', 'u', 'ö', 'ä', 'ü'].contains(&c){
            count += 1;
        }
    }
    count
}