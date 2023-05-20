use std::time::Instant;
use itertools::Itertools;

/// Checks if the given slice of digits satisfies the divisibility property.
///
/// The divisibility property requires that each 3-digit substring starting from the second digit
/// is divisible by a corresponding divisor. The divisors are [2, 3, 5, 7, 11, 13, 17].
///
/// # Arguments
///
/// * `digits` - A slice of characters representing the digits to be checked.
///
/// # Returns
///
/// A boolean value indicating whether the divisibility property holds for the given digits.
fn check_divisibility_property(digits: &[char]) -> bool {
    let divisors = [2, 3, 5, 7, 11, 13, 17];

    for (idx, _) in (1..=7).zip(digits.iter().skip(1)) {
        let number_str: String = digits[idx as usize..idx as usize + 3].iter().collect();
        let number: u32 = number_str.parse().unwrap();

        if number % divisors[idx as usize - 1] != 0 {
            return false;
        }
    }

    true
}

fn main() {
    let start = Instant::now();
    let sum: u64 = "0123456789"
        .chars()
        .permutations(10)
        .filter(|perm| check_divisibility_property(perm))
        .map(|perm| perm.iter().collect::<String>().parse::<u64>().unwrap())
        .sum();
    let end = Instant::now();

    println!("Solution: {}", sum);
    println!("Elapsed time: {:?}", end - start);
}
