use std::time::Instant;
use itertools;
use itertools::iproduct;

/// Checks if a number is a palindrome.
///
/// # Arguments
///
/// * `n` - The number to check for palindrome.
///
/// # Returns
///
/// Returns `true` if the number is a palindrome, otherwise `false`.
fn is_palindrome<T: std::fmt::Display>(n: T) -> bool {
    let n_str = n.to_string();
    n_str.chars().eq(n_str.chars().rev())
}

fn main() {
    let start = Instant::now();

    // Find the largest palindrome product of two three-digit numbers
    let palindrome_product = iproduct!(100..=999, 100..=999)
        .map(|(f1, f2)| f1 * f2)
        .filter(|product| is_palindrome(product))
        .max();

    if let Some(product) = palindrome_product {
        println!("Solution: {}", product);
    }

    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
