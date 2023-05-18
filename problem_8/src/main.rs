use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::time::Instant;

/// Reads digits from a file and returns them as a vector.
///
/// # Arguments
///
/// * `filename` - The name of the file to read from.
///
/// # Returns
///
/// A vector containing the digits read from the file.
fn read_digits_from_file(filename: &str) -> Result<Vec<u8>, Error> {
    // Open the file
    let file = File::open(filename)?;
    // Wrap the file in a buffered reader
    let file_reader = BufReader::new(file);
    // Read lines from the file
    let lines: Result<Vec<String>, _> = file_reader.lines().collect();
    let digits = lines?
        .iter()
        // Convert each character to a digit
        .flat_map(|line| line.chars().filter_map(|c| c.to_digit(10)))
        // Convert the digits to u8
        .map(|digit| digit as u8)
        // Collect the digits into a vector
        .collect();

    Ok(digits)
}

/// Finds the greatest product of `adjacent_digits` consecutive digits in the given vector.
///
/// # Arguments
///
/// * `digits` - The vector of digits.
/// * `adjacent_digits` - The number of adjacent digits to consider for the product.
///
/// # Returns
///
/// The greatest product of `adjacent_digits` consecutive digits.
fn greatest_product(digits: &[u8], adjacent_digits: usize) -> u64 {
    let digits_len = digits.len();
    let mut max_product: u64 = 0;

    for i in 0..=digits_len - adjacent_digits {
        let product: u64 = digits[i..i + adjacent_digits]
            .iter()
            .map(|&digit| digit as u64)
            // Calculate the product of adjacent digits
            .product();

        // Update the maximum product if necessary
        max_product = max_product.max(product);
    }

    max_product
}

fn main() {
    let start = Instant::now();

    if let Ok(digits) = read_digits_from_file("resources/data/p8_input.txt") {
        println!("Solution: {}", greatest_product(&digits, 13));
    } else {
        eprintln!("Error reading digits from the file");
    }

    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
