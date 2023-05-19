use std::time::Instant;

/// Calculates the self power of numbers from 1 to n and returns the last 10 digits as a string.
///
/// # Arguments
///
/// * `n` - The upper limit of the range (inclusive).
fn self_power(n: u32) -> String {
    let mut sum: u64 = 0;
    let modulo: u64 = 10_000_000_000;

    for i in 1..=n {
        let mut pow: u64 = i as u64;
        for _ in 2..=i {
            pow = (pow * (i as u64)) % modulo;
        }
        sum = (sum + pow) % modulo;
    }

    sum.to_string()
}

fn main() {
    let start = Instant::now();
    let result = self_power(1000);

    println!("Solution: {}", result);
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
