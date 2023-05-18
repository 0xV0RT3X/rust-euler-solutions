use std::time::Instant;

const NUMBERS: u64 = 100_000;

/// Calculates the nth triangle number using the formula n * (n + 1) / 2.
fn t(n: u64) -> u64 {
    n * (n + 1) / 2
}

/// Calculates the nth pentagonal number using the formula n * (3n - 1) / 2.
fn p(n: u64) -> u64 {
    n * (3 * n - 1) / 2
}

/// Calculates the nth hexagonal number using the formula n * (2n - 1).
fn h(n: u64) -> u64 {
    n * (2 * n - 1)
}

/// Finds the next triangle number that is also pentagonal and hexagonal.
/// Returns `Some(triangle_number)` if found, `None` otherwise.
fn next_triangle() -> Option<u64> {
    // Generate vectors of pentagonal and hexagonal numbers
    let pentagonal_numbers: Vec<u64> = (1..=NUMBERS).map(p).collect();
    let hexagonal_numbers: Vec<u64> = (1..=NUMBERS).map(h).collect();

    let mut n = 286;

    while n < NUMBERS {
        let triangle_number = t(n);

        // Check if the triangle number is pentagonal and hexagonal using binary search
        if pentagonal_numbers.binary_search(&triangle_number).is_ok()
            && hexagonal_numbers.binary_search(&triangle_number).is_ok()
        {
            return Some(triangle_number);
        }

        n += 1;
    }

    None
}

fn main() {
    let start = Instant::now();
    println!("Solution: {}", next_triangle().unwrap());
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
