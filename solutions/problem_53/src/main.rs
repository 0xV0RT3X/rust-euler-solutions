use std::time::Instant;
use num::BigInt;

/// Calculates the binomial coefficient (n choose k).
///
/// The binomial coefficient represents the number of ways to choose k elements
/// from a set of n elements without regard to the order of selection.
///
/// The function uses the formula: C(n, k) = n! / (k! * (n - k)!)
///
/// # Arguments
///
/// * `n` - The total number of elements in the set.
/// * `k` - The number of elements to choose.
///
/// # Returns
///
/// The binomial coefficient as a BigInt.
fn calculate_binomial_coefficient(n: u32, k: u32) -> BigInt {
    let mut numerator = BigInt::from(1u32);
    let mut denominator = BigInt::from(1u32);

    // Calculate the numerator and denominator separately
    // by iteratively multiplying and dividing.
    for i in 1..=k {
        numerator *= n - (i - 1);
        denominator *= i;
    }

    numerator / denominator
}

fn main() {
    let start = Instant::now();

    let one_million = BigInt::from(1_000_000);

    // Generate pairs of (n, k) values using nested iterators
    // and count the number of binomial coefficients greater than one million.
    let cnt = (1..=100)
        .flat_map(|n| (1..=n).map(move |k| (n, k)))
        .fold(0, |acc, (n, k)| {
            let binomial_coefficient = calculate_binomial_coefficient(n, k);
            if binomial_coefficient > one_million {
                acc + 1
            } else {
                acc
            }
        });

    let end = Instant::now();

    println!("Solution: {}", cnt);
    println!("Elapsed time: {:?}", end - start);
}
