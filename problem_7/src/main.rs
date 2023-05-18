use std::time::Instant;
use primes::{PrimeSet, Sieve};

fn main() {
    let start = Instant::now();

    // Create a new PrimeSet using a sieve algorithm
    let mut sieve = Sieve::new();

    // Use the `iter` method of the PrimeSet to iterate over prime numbers
    // The `nth` method is used to retrieve the prime number at index 10,000
    let solution = sieve.iter().nth(10_000).unwrap();

    let end = Instant::now();

    // Print the solution and elapsed time
    println!("Solution: {}", solution);
    println!("Elapsed time: {:?}", end - start);
}
