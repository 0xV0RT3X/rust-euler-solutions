use std::time::Instant;
use num::BigUint;
use num::bigint::ToBigUint;
use std::collections::{HashSet, HashMap};

/// Constant defining the number of non-repeating terms in a chain.
const NON_REPEATING_TERMS: usize = 60;
/// Constant defining the upper limit for the starting value.
const BELOW_ONE_MILLION: u32 = 1_000_000;

/// Computes the factorial of a number passed as a `BigUint`.
/// Uses a caching mechanism to store previously computed values and avoid duplicate calculations.
fn digit_factorial(n: &BigUint, cache: &mut HashMap<BigUint, BigUint>) -> BigUint {
    if let Some(result) = cache.get(n) {
        return result.clone();
    }

    let result = n
        .to_str_radix(10)
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .map(|d| (1..=d).map(BigUint::from).product::<BigUint>())
        .sum::<BigUint>();

    cache.insert(n.clone(), result.clone());
    result
}

/// Counts the number of chains with a starting value below one million
/// that contain exactly sixty non-repeating terms.
fn count_non_repeating_terms() -> usize {
    let mut cache: HashMap<BigUint, BigUint> = HashMap::new();
    let mut count = 0;

    for n in 1..BELOW_ONE_MILLION {
        let mut distinct_terms = HashSet::<BigUint>::new();
        let mut old_fac = n.to_biguint().unwrap();

        while distinct_terms.len() <= NON_REPEATING_TERMS {
            if !distinct_terms.insert(old_fac.clone()) {
                break;
            }

            old_fac = digit_factorial(&old_fac, &mut cache);
        }

        if distinct_terms.len() == NON_REPEATING_TERMS {
            count += 1;
        }
    }

    count
}

fn main() {
    let start = Instant::now();
    println!("Solution: {}", count_non_repeating_terms());
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
