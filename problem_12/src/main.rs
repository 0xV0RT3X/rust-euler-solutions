use std::time::Instant;

const DIVISORS_REQUIRED: u32 = 500;

/// Generates an iterator over triangle numbers.
///
/// A triangle number is the sum of all natural numbers up to a given number.
/// This iterator generates triangle numbers incrementally.
fn triangle_terms() -> impl Iterator<Item = u32> {
    let mut n = 0;
    std::iter::from_fn(move || {
        n += 1;
        // Calculate the nth triangle number using the formula (n * (n + 1)) / 2.
        Some(n * (n + 1) / 2)
    })
}

/// Calculates the number of divisors of a given number.
///
/// # Arguments
///
/// * `n` - The number to calculate the divisors for.
///
/// # Returns
///
/// The number of divisors of `n`.
fn divisors(n: u32) -> u32 {
    let mut cnt = 0;
    let sqrt_n = (n as f64).sqrt() as u32;

    // Iterate from 1 up to the square root of n.
    for i in 1..=sqrt_n {
        if n % i == 0 {
            // If i is a divisor, there are two divisors: i and n/i (except when i equals sqrt(n)).
            cnt += 2;
        }
    }

    // If the square root of n is an integer, subtract 1 from the count (to avoid counting it twice).
    if sqrt_n * sqrt_n == n {
        cnt -= 1;
    }

    cnt
}

fn main() {
    let start = Instant::now();

    // Find the first triangle number with more than DIVISORS_REQUIRED divisors.
    println!("Solution: {}", triangle_terms()
        .find(|&term| divisors(term) > DIVISORS_REQUIRED)
        .unwrap());

    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
