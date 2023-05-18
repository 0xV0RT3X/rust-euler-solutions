use std::collections::HashSet;
use std::time::Instant;
use primes::{PrimeSet, Sieve};

fn truncatable_primes(n: usize) -> Vec<u64> {
    // Create a new prime sieve
    let mut prime_sieve = Sieve::new();

    // Create a HashSet to store the first n primes for constant time (O(1)) lookup
    let primes: HashSet<u64> = prime_sieve.iter().take(n).collect();

    // Iterate over the first n primes (excluding 2, 3, 5, and 7)
    // Use the Sieve iterator instead of the HashSet iterator because HashSet values are stored randomly
    let truncatable_primes: Vec<u64> = prime_sieve
        .iter()
        .take(n)
        // Skip the first four primes (2, 3, 5, and 7) as they are not considered truncatable
        .skip(4)
        .filter(|&prime| is_truncatable_prime(prime, &primes))
        .collect();

    truncatable_primes
}

fn is_truncatable_prime(prime: u64, primes: &HashSet<u64>) -> bool {
    remain_prime_left_to_right(prime, primes) && remain_prime_right_to_left(prime, primes)
}

fn remain_prime_left_to_right(prime: u64, primes: &HashSet<u64>) -> bool {
    let prime_as_str = prime.to_string();

    for i in 1..=prime_as_str.len() - 1 {
        if !primes.contains(&prime_as_str[i..].parse::<u64>().unwrap()) {
            return false;
        }
    }

    true
}

fn remain_prime_right_to_left(prime: u64, primes: &HashSet<u64>) -> bool {
    let prime_as_str = prime.to_string();

    for i in (1..=prime_as_str.len() - 1).rev() {
        if !primes.contains(&prime_as_str[..i].parse::<u64>().unwrap()) {
            return false;
        }
    }

    true
}

fn main() {
    let start = Instant::now();
    let result = truncatable_primes(100_000);
    println!("Solution: {}", result.iter().sum::<u64>());
    println!("Elapsed time: {:?}", Instant::now() - start);
}
