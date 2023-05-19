use std::time::Instant;

/// Calculates the next value in the chain sequence.
///
/// This function takes a positive integer `n` and returns the next value
/// in the chain by summing the squares of its digits.
fn next_link(n: u64) -> u64 {
    n.to_string()
        .chars()
        .map(|d| {
            let d = d.to_digit(10).unwrap() as u64;
            d * d
        })
        .sum()
}

/// Counts the number of square digit chains that arrive at 89.
///
/// This function generates chain sequences starting from 2 up to a specified limit.
/// It counts the number of sequences that eventually reach the value 89.
///
/// Returns:
/// - The count of square digit chains that arrive at 89.
fn count_square_digit_chains() -> usize {
    const LIMIT: usize = 10_000_000;
    let mut cache: Vec<Option<bool>> = vec![None; LIMIT];
    let mut count = 0;

    for n in 2..LIMIT {
        if chain_arrives_at_89(n as u64, &mut cache) {
            count += 1;
        }
    }

    count
}

/// Checks if a chain sequence arrives at 89.
///
/// This function takes a positive integer `n` and a mutable reference to a cache.
/// It generates the chain sequence for `n` and checks if it eventually reaches 89.
///
/// Returns:
/// - `true` if the chain sequence arrives at 89.
/// - `false` if the chain sequence arrives at 1.
fn chain_arrives_at_89(n: u64, cache: &mut Vec<Option<bool>>) -> bool {
    let mut link = n;

    while link != 1 && link != 89 {
        if let Some(result) = cache[link as usize] {
            return result;
        }

        link = next_link(link);
    }

    let result = link == 89;

    let mut link = n;
    while link != 1 && link != 89 {
        cache[link as usize] = Some(result);
        link = next_link(link);
    }

    result
}

fn main() {
    let start = Instant::now();
    println!("Solution: {}", count_square_digit_chains());
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
