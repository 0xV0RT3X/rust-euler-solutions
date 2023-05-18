use std::time::Instant;
use num::BigInt;

fn main() {
    // Start measuring the execution time
    let start = Instant::now();

    // Create a BigInt from the number 2
    let digit_sum: u32 = BigInt::from(2)
        // Raise it to the power of 1,000
        .pow(1_000)
        // Convert it to a string representation in base 10
        .to_str_radix(10)
        // Convert the string into an iterator of characters
        .chars()
        // Convert each character to a digit
        .map(|c| c.to_digit(10).unwrap())
        // Sum all the digits
        .sum();

    // End measuring the execution time
    let end = Instant::now();

    // Print the solution and elapsed time
    println!("Solution: {}", digit_sum);
    println!("Elapsed time: {:?}", end - start);
}
