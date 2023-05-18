use std::time::Instant;
use primes::{PrimeSet, Sieve};

fn main() {
    let start = Instant::now();
    let mut sieve = Sieve::new();
    println!("Solution: {}", sieve.iter().take_while(|&prime| prime < 2_000_000).sum::<u64>());
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
