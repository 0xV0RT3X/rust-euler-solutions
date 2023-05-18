use std::time::Instant;

fn digit_fifth_powers() -> usize {
    // Array of fifth powers of digits from 0 to 9
    let fifth_powers: [u64; 10] = [0, 1, 32, 243, 1024, 3125, 7776, 16807, 32768, 59049];

    // Create a range of numbers from 2 to 999,999 and perform operations on them
    (2..1_000_000)
        .filter(|&number| {
            // Convert the number to a string, iterate over its characters,
            // calculate the fifth power for each digit, and sum them up
            let sum_of_pow: u64 = number
                .to_string()
                .chars()
                .map(|c| fifth_powers[c.to_digit(10).unwrap() as usize])
                .sum();

            // Check if the sum of fifth powers is equal to the original number
            sum_of_pow == number as u64
        })
        // Sum up all the filtered numbers
        .sum()
}

fn main() {
    let start = Instant::now();
    println!("Solution: {}", digit_fifth_powers());
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
