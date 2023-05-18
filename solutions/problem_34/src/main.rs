use std::collections::HashMap;
use std::time::Instant;

fn factorial_digits() -> u32 {
    // Create a HashMap to store factorial values for digits 0 to 9
    let factorials: HashMap<u32, u32> = (0..=9).map(|k| (k, (1..=k).product())).collect();

    // Find numbers whose sum of factorial of digits equals the number itself
    let sum: u32 = (3..=200_000)
        .filter_map(|number| {
            let fac_sum: u32 = number
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .map(|d| factorials.get(&d).unwrap_or(&0))
                .sum();

            // Check if the sum of factorial of digits equals the number
            if fac_sum == number {
                Some(number)
            } else {
                None
            }
        })
        .sum();

    sum
}

fn main() {
    // Measure the execution time
    let start = Instant::now();

    // Calculate the solution and print it
    println!("Solution: {}", factorial_digits());

    let end = Instant::now();
    println!("Elapsed time: {:?}", end - start);
}
