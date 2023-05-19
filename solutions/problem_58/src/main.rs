use std::time::Instant;
use primes::is_prime;

// 37 36 35 34 33 32 31
// 38 17 16 15 14 13 30
// 39 18  5  4  3 12 29
// 40 19  6  1  2 11 28
// 41 20  7  8  9 10 27
// 42 21 22 23 24 25 26
// 43 44 45 46 47 48 49

fn find_side_length(target_ratio: f64) -> u32 {
    // 5  4  3
    // 6  1  2
    // 7  8  9 <- Here we are right now
    let mut diagonal_number = 9;


    // 3 primes in 3x3 square (3, 5 and 7)
    let mut diagonal_primes = 3;
    // (1, 3, 5, 7, 9)
    let mut diagonal_numbers = 5;

    // Next diagonal value is 13:
    // 13 - 9 = 4
    // After the 4th iteration the step increases by 2
    // therefore the next step would be 6 (4+2)
    let mut step = 4;

    let mut side_length = 3;

    while (diagonal_primes as f64 / diagonal_numbers as f64) >= target_ratio {
        // Side length increases by 2 after each new layer
        side_length += 2;
        for _ in 0..4 {
            diagonal_number += step;
            diagonal_numbers += 1;

            if is_prime(diagonal_number) {
                diagonal_primes += 1;
            }

        }

        step += 2;
    }
    side_length
}

fn main() {
    let start = Instant::now();

    const TARGET_RATIO: f64 = 0.1;
    let side_length = find_side_length(TARGET_RATIO);
    let end = Instant::now();

    println!("Solution: {}", side_length);
    println!("Elapsed time: {:?}", end - start);
}