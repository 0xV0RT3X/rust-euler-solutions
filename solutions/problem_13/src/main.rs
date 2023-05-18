use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::str::FromStr;
use std::time::Instant;

use num::BigInt;

/// Reads a file containing a list of numbers and calculates their large sum.
///
/// # Arguments
///
/// * `filename` - The name of the file to read from.
///
/// # Returns
///
/// * `Result<BigInt, Error>` - The sum of the numbers as a `BigInt`, or an `Error` if there was an issue reading the file or parsing the numbers.
fn large_sum_from_file(filename: &str) -> Result<BigInt, Error> {
    // Open the file
    let file = File::open(filename)?;
    // Wrap the file in a `BufReader` for efficient reading
    let file_reader = BufReader::new(file);

    // Iterate over the lines of the file
    let large_sum: BigInt = file_reader.lines()
        // Parse each line as a `BigInt`
        .map(|line| BigInt::from_str(&*line.unwrap()).unwrap())
        // Calculate the sum of the `BigInt` values
        .sum();

    Ok(large_sum)
}

fn main() {
    // Start measuring the elapsed time
    let start = Instant::now();

    if let Ok(sum) = large_sum_from_file("resources/data/p13_input.txt") {
        // Convert the sum to a string representation in base 10
        let sum_str = sum.to_str_radix(10);
        // Get the first 10 characters of the string
        if let Some(first_10_chars) = sum_str.get(..10) {
            // Print the solution
            println!("Solution: {}", first_10_chars);
        }
    }

    // Stop measuring the elapsed time
    let end = Instant::now();

    // Print the elapsed time
    println!("Elapsed time: {:?}", end - start);
}
