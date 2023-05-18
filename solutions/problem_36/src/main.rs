use std::time::Instant;

/// Finds the sum of all numbers below 1,000,000 that are palindromic in both decimal and binary representations.
///
/// # Returns
///
/// Returns the sum of the palindromic numbers.
fn double_base_palindrome_sum() -> u64 {
    (1..1_000_000)
        .filter(|&n| n % 2 == 1 && is_number_palindrome(n) && is_binary_palindrome(n))
        .sum()
}

/// Checks if a number is a palindrome in decimal representation.
///
/// # Arguments
///
/// * `n` - The number to check for palindromicity.
///
/// # Returns
///
/// Returns `true` if the number is a decimal palindrome, `false` otherwise.
fn is_number_palindrome(n: u64) -> bool {
    let n_str = n.to_string();
    let n_str_rev: String = n_str.chars().rev().collect();

    n_str == n_str_rev
}

/// Checks if a number is a palindrome in binary representation.
///
/// # Arguments
///
/// * `n` - The number to check for palindromicity.
///
/// # Returns
///
/// Returns `true` if the number is a binary palindrome, `false` otherwise.
fn is_binary_palindrome(n: u64) -> bool {
    let binary_str = format!("{:b}", n);
    let binary_str_rev: String = binary_str.chars().rev().collect();

    binary_str == binary_str_rev
}

fn main() {
    let start = Instant::now();
    println!("Solution: {}", double_base_palindrome_sum());
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
