use std::time::Instant;

/// Computes the greatest common divisor (GCD) of two numbers using the Euclidean algorithm.
///
/// The Euclidean algorithm is an efficient method for computing the GCD of two numbers.
/// It repeatedly divides the larger number by the smaller number and replaces the larger number
/// with the remainder until the remainder is zero. The GCD is the non-zero remainder obtained
/// at the end of the process.
///
/// # Arguments
///
/// * `a` - The first number.
/// * `b` - The second number.
///
/// # Returns
///
/// The greatest common divisor of the two numbers.
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Computes the least common multiple (LCM) of two numbers using the GCD.
///
/// The LCM of two numbers is the smallest positive integer that is divisible by both numbers.
/// It can be calculated using the formula:
///     lcm(a, b) = (a * b) / gcd(a, b)
/// where gcd(a, b) represents the greatest common divisor of the two numbers.
///
/// # Arguments
///
/// * `a` - The first number.
/// * `b` - The second number.
///
/// # Returns
///
/// The least common multiple of the two numbers.
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn main() {
    // Start measuring the execution time
    let start = Instant::now();

    // Initialize the variable to hold the smallest multiple
    let mut smallest_multiple = 1;

    // Iterate from 11 to 19 (inclusive)
    // By starting from 11, we skip numbers that are already divisible by smaller numbers
    for i in 11..=19 {
        // Compute the least common multiple of the current number and the previous smallest multiple
        smallest_multiple = lcm(smallest_multiple, i);
    }

    // Print the computed smallest multiple
    println!("Solution: {}", smallest_multiple);

    // Stop measuring the execution time
    let end = Instant::now();

    // Print the elapsed time
    println!("Elapsed time: {:?}", end - start);
}
