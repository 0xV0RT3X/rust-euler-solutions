use num::bigint::BigUint;
use num::traits::One;
use std::time::Instant;

/// Calculates the index of the Fibonacci number that has the specified number of digits.
/// Returns the index of the Fibonacci number.
fn fibonacci_digits_count(digits: usize) -> u32 {
    // Initialize the starting values of Fibonacci sequence
    // First Fibonacci number
    let mut a: BigUint = One::one();
    // Second Fibonacci number
    let mut b: BigUint = One::one();

    // Current index of Fibonacci number
    let mut idx: u32 = 2;

    // Generate Fibonacci sequence until a number with the specified number of digits is reached
    while b.to_string().chars().count() < digits {
        // Calculate the next Fibonacci number
        let temp = a + &b;
        // Update the values of a and b
        a = std::mem::replace(&mut b, temp);
        // Increment the index
        idx += 1;
    }

    // Return the index of the Fibonacci number
    idx
}

fn main() {
    let start = Instant::now();

    // Number of digits desired
    let digits = 1_000;
    // Calculate the index of Fibonacci number
    let idx = fibonacci_digits_count(digits);

    // Print the index of Fibonacci number
    println!("Solution: {}", idx);
    let end = Instant::now();

    // Print the execution time
    println!("Elapsed time: {:?}", end - start);
}
