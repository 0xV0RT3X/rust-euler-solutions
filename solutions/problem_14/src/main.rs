use std::time::Instant;

/// Calculates the length of the Collatz sequence for a given starting number.
fn collatz_sequence_length(n: u64) -> u64 {
    let mut count = 1;
    let mut num = n;

    while num != 1 {
        // Apply the Collatz sequence rule: n → n/2 (if n is even) or n → 3n + 1 (if n is odd)
        num = if num % 2 == 0 { num / 2 } else { 3 * num + 1 };
        count += 1;
    }

    count
}

fn main() {
    let start = Instant::now();

    // Find the starting number that produces the longest Collatz chain
    let max_chain = (1..1_000_000)
        .max_by_key(|&n| collatz_sequence_length(n))
        .unwrap();

    let end = Instant::now();

    // Print the solution and elapsed time
    println!("Solution: {}", max_chain);
    println!("Elapsed time: {:?}", end - start);
}
