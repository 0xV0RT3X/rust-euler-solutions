use std::collections::HashSet;
use std::time::Instant;
use num::{BigInt, One, Signed};
use num::bigint::ToBigInt;

/// Calculates the nth pentagonal number.
///
/// # Arguments
///
/// * `n` - The index of the pentagonal number to calculate.
///
/// # Returns
///
/// The nth pentagonal number as a `BigInt`.
fn p(n: u32) -> BigInt {
    let three = BigInt::from(3);
    let one = BigInt::one();
    let n_bigint = BigInt::from(n);
    let numerator = &n_bigint * (three * &n_bigint - one);
    let denominator = BigInt::from(2);
    numerator / denominator
}

/// Finds a pair of pentagonal numbers conditions.
///
/// The function that satisfy the given searches for a pair of pentagonal numbers (pj, pk) such that both the sum (pj + pk)
/// and the difference (|pj - pk|) are pentagonal numbers as well. It iterates through pentagonal
/// numbers in the range from 1 to 10,000 to find such a pair.
///
/// # Returns
///
/// - `Some((pj, pk))` - A tuple containing the pair of pentagonal numbers (pj, pk) that satisfy the conditions.
/// - `None` - If no such pair is found within the search range.
fn find_pair() -> Option<(BigInt, BigInt)> {
    // Create a HashSet to store pentagonal numbers.
    let pentagonals: HashSet<BigInt> = (1..=1_000_000).map(p).collect();

    for j in 1..=10_000 {
        let pj = p(j).to_bigint().unwrap();

        for k in (j + 1)..=10_000 {
            let pk = p(k).to_bigint().unwrap();
            let sum = &pj + &pk;
            let difference = (&pk - &pj).abs();

            if pentagonals.contains(&sum) && pentagonals.contains(&difference) {
                return Some((pj, pk));
            }
        }
    }

    None
}

fn main() {
    let start = Instant::now();
    if let Some(pair) = find_pair() {
        let difference = (&pair.1 - &pair.0).abs();
        println!("Solution: {}", difference);
    } else {
        eprintln!("No solution found!");
    }

    let end = Instant::now();
    println!("Elapsed time: {:?}", end - start);
}
