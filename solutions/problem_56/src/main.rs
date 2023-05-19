use std::cmp::max;
use std::time::Instant;
use num::BigInt;

/// Computes the maximum digit sum of the numbers `a^b` for `1 <= a < 100` and `1 <= b < 100`.
fn max_digit_sum() -> u32 {
    let mut max_sum = 0;

    // Iterate over the base `a` from 1 to 99.
    for a in 1..100 {
        let base = BigInt::from(a);

        // Iterate over the exponent `b` from 1 to 99.
        for b in 1..100 {
            // Compute the power of `base` raised to `b`.
            let result = base.pow(b);

            // Calculate the digit sum of the result.
            let digit_sum = digit_sum(&result);

            // Update the maximum digit sum if necessary.
            max_sum = max(max_sum, digit_sum);
        }
    }

    max_sum
}

/// Computes the sum of digits in a `BigInt` number.
fn digit_sum(num: &BigInt) -> u32 {
    let digits = num
        .to_str_radix(10)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum();

    digits
}

fn main() {
    let start = Instant::now();
    println!("{}", max_digit_sum());
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
