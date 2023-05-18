use std::time::Instant;

/// Calculates the sum of proper divisors for a given number `n`.
/// Proper divisors are the positive divisors excluding `n` itself.
fn divisor_sum(n: u32) -> u32 {
    // Iterate over the range from 1 to `n/2` (inclusive) and filter for divisors.
    // Then, calculate the sum of the filtered divisors.
    (1..=n/2)
        .filter(|d| n % d == 0)
        .sum()
}

/// Calculates the sum of all amicable numbers up to a given limit `n`.
/// Amicable numbers are pairs of numbers where the sum of proper divisors of one number
/// equals the other number, and vice versa.
fn sum_amicable_numbers(n: u32) -> u32 {
    // Iterate over the range from 1 to `n` (exclusive) and filter for amicable numbers.
    // An amicable number is a pair of numbers where the sum of proper divisors of one number
    // equals the other number, and vice versa.
    (1..n)
        .filter(|&i| {
            let sum = divisor_sum(i);
            sum != i && divisor_sum(sum) == i
        })
        .sum()
}

fn main() {
    let start = Instant::now();

    // Calculate the sum of amicable numbers up to 10,000 and print the solution.
    let solution = sum_amicable_numbers(10_000);
    println!("Solution: {}", solution);

    let end = Instant::now();

    // Measure and print the elapsed time.
    println!("Elapsed time: {:?}", end - start);
}
