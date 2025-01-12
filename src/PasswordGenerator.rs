/*
* This program generates a random password of a specified length.
 */
// Path: src/PasswordGenerator.rs
// Run: cargo run --quiet --release

use rand::Rng; // Import the Rng trait for random number generation
use std::io; // Import the io module for user input

fn main() {
    // Ask the user for the desired password length
    println!("Enter the desired password length:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let password_length: usize = input.trim().parse().expect("Please enter a valid number");

    // Ask the user for character set preferences
    println!("Include lowercase letters? (y/n):");
    let mut include_lowercase = String::new();
    io::stdin().read_line(&mut include_lowercase).expect("Failed to read line");

    println!("Include uppercase letters? (y/n):");
    let mut include_uppercase = String::new();
    io::stdin().read_line(&mut include_uppercase).expect("Failed to read line");

    println!("Include digits? (y/n):");
    let mut include_digits = String::new();
    io::stdin().read_line(&mut include_digits).expect("Failed to read line");

    println!("Include special characters? (y/n):");
    let mut include_special = String::new();
    io::stdin().read_line(&mut include_special).expect("Failed to read line");

    // Define character sets based on user preferences
    let mut charset = String::new();
    if include_lowercase.trim().to_lowercase() == "y" {
        charset.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if include_uppercase.trim().to_lowercase() == "y" {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if include_digits.trim().to_lowercase() == "y" {
        charset.push_str("0123456789");
    }
    if include_special.trim().to_lowercase() == "y" {
        charset.push_str("!@#$%^&*()-_=+[]{}|;:,.<>?");
    }

    // Ensure the charset is not empty
    if charset.is_empty() {
        println!("No character sets selected. Exiting.");
        return;
    }

    let mut result = String::new(); // Initialize an empty string to store the password

    // Loop to generate each character of the password
    for _ in 0..password_length {
        let idx = rand::thread_rng().gen_range(0..charset.len()); // Generate a random index
        result.push(charset.chars().nth(idx).unwrap()); // Append the character at the random index
    }

    println!("Generated Password: {}", result); // Print the resulting password
}