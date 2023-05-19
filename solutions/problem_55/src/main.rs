use std::time::Instant;
use num::BigUint;
use std::ops::Range;

const MAX_ITERATIONS: u32 = 50;
const LYCHREL_RANGE: Range<u32> = 1..10_000;

/// Calculates the count of Lychrel numbers within the specified range.
fn lychrel_numbers() -> usize {
    LYCHREL_RANGE
        .filter(|&number| is_lychrel_number(number))
        .count()
}

/// Checks if a number is a Lychrel number.
/// A Lychrel number is a number that does not form a palindrome
/// even after repeatedly reversing and adding its reverse.
/// Returns true if the number is a Lychrel number, false otherwise.
fn is_lychrel_number(n: u32) -> bool {
    let mut n = BigUint::from(n);

    for _ in 1..=MAX_ITERATIONS {
        let rev_n = reverse_number(&n);
        let sum = &n + &rev_n;

        if is_palindrome(&sum) {
            return false;
        }

        n = sum;
    }

    true
}

/// Reverses the digits of a number represented as a `BigUint`.
/// Returns the reversed number as a `BigUint`.
fn reverse_number(n: &BigUint) -> BigUint {
    let s = n.to_string();
    let rev_s = s.chars().rev().collect::<String>();
    BigUint::parse_bytes(rev_s.as_bytes(), 10).unwrap()
}

/// Checks if a number represented as a `BigUint` is a palindrome.
/// Returns true if the number is a palindrome, false otherwise.
fn is_palindrome(n: &BigUint) -> bool {
    let s = n.to_string();
    let rev_s = s.chars().rev().collect::<String>();
    s == rev_s
}

fn main() {
    let start = Instant::now();
    println!("Solution: {}", lychrel_numbers());
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
