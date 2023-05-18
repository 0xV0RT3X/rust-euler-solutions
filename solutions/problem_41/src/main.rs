use std::cmp::max;
use std::collections::HashMap;
use std::time::Instant;
use itertools::Itertools;
use primes::is_prime;

fn largest_pandigital_prime() -> u64 {
    let digits: Vec<u64> =(1..=9).collect();
    let permutations = digits.iter().permutations(7);

    let mut largest_pandigital = 0;

    for permutation in permutations {
        let num: u64 = permutation.iter().fold(0, |acc, &digit| acc * 10 + digit);

        // Check if the number is prime and pandigital
        if is_prime(num) && is_pandigital(num) {
            largest_pandigital = max(largest_pandigital, num);
        }
    }

    largest_pandigital
}

fn is_pandigital(number: u64) -> bool {
    let number_str = number.to_string();
    let length = number_str.len();

    // Create a HashMap to track the count of each digit
    let mut digits_cnt: HashMap<char, u32> = (1..=length).map(|n| (char::from_digit(n as u32, 10).unwrap(), 0)).collect();

    for c in number_str.chars() {
        // Check if the digit is already counted
        if let Some(cnt) = digits_cnt.get_mut(&c) {
            if *cnt != 0 {
                // Found a duplicate digit, not pandigital
                return false;
            }

            // Mark the digit as counted
            *cnt = 1;
        }
    }

    // Check if all digits are counted
    digits_cnt.values().sum::<u32>() == length as u32
}

fn main() {
    let start = Instant::now();
    println!("Solution: {}", largest_pandigital_prime());
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
