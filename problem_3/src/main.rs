use std::time::Instant;
use primes::factors;

fn main() {
    // Start the timer to measure the execution time
    let start = Instant::now();

    // Define the number for which we want to find the largest prime factor
    let number = 600_851_475_143;

    // Calculate the prime factors of the number using the `factors` function from the `primes` crate
    let prime_factors = factors(number);

    // Get the largest prime factor by retrieving the last element from the list of prime factors
    let largest_prime_factor = *prime_factors.last().unwrap();

    // Stop the timer and calculate the elapsed time
    let end = Instant::now();

    // Print the solution, which is the largest prime factor
    println!("Solution: {}", largest_prime_factor);

    // Print the elapsed time, which is the duration between start and end
    println!("Elapsed time: {:?}", end - start);
}
