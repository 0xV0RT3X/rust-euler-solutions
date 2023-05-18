use std::time::Instant;
use num::BigInt;

fn main() {
    // Measure the start time for timing purposes
    let start = Instant::now();

    // Calculate the product of all numbers from 1 to 100 (inclusive)
    let product = (1..=100).product::<BigInt>();

    // Convert the product into a string representation using base 10 (decimal)
    let digit_str = product.to_str_radix(10);

    // Extract each character from the string, convert it to a digit, and sum them up
    let digit_sum = digit_str.chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>();

    // Measure the end time for timing purposes
    let end = Instant::now();

    // Print the sum of the digits
    println!("Solution: {}", digit_sum);

    // Print the total elapsed time for the computation
    println!("Elapsed time: {:?}", end - start);
}
