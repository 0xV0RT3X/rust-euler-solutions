use std::time::Instant;
use primes::{PrimeSet, Sieve};

/// Checks if a number is a circular prime.
///
/// A circular prime is a number that remains prime after every possible rotation
/// of its digits.
///
/// # Arguments
///
/// * `n` - The number to check for circular primality.
///
/// # Returns
///
/// Returns `true` if the number is a circular prime, `false` otherwise.
fn is_circular_prime(n: usize, primes: &mut Sieve) -> bool {
    let mut number = n;
    let mut digits = Vec::new();

    // Extract digits from the number
    while number > 0 {
        digits.push(number % 10);
        number /= 10;
    }

    let num_digits = digits.len();

    // Check each rotation of the digits
    for _ in 0..num_digits {
        // Combine the rotated digits to form a new number
        let rotated_number = digits.iter().fold(0, |acc, &digit| acc * 10 + digit);

        if !primes.is_prime(rotated_number as u64) {
            return false;
        }

        // Rotate the digits by moving the last digit to the front
        let last_digit = digits.pop().unwrap();
        digits.insert(0, last_digit);
    }

    true
}

/// Finds the count of circular primes up to a given limit.
///
/// # Arguments
///
/// * `n` - The limit up to which to find circular primes.
///
/// # Returns
///
/// Returns the count of circular primes up to the given limit.
fn circular_primes(n: usize) -> usize {
    let mut primes = Sieve::new();
    (2..n)
        .filter(|&num| is_circular_prime(num, &mut primes))
        .count()
}

fn main() {
    let start = Instant::now();
    println!("Solution: {}", circular_primes(1_000_000));
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
