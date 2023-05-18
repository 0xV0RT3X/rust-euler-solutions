use std::time::Instant;

// Mapping of numbers (1-9), TEENS (10-19), and TENS (20-90) to their respective letter counts
// The index corresponds to the number, e.g., ONES[5] represents the letter count for "five"
const ONES: [usize; 10] = [0, 3, 3, 5, 4, 4, 3, 5, 5, 4];
const TEENS: [usize; 10] = [3, 6, 6, 8, 8, 7, 7, 9, 8, 8];
const TENS: [usize; 10] = [0, 0, 6, 6, 5, 5, 5, 7, 6, 6];

// Number of characters for "hundred", "and", and "one thousand"
const HUNDRED_CHARS: usize = 7;
const AND_CHARS: usize = 3;
const ONE_THOUSAND_CHARS: usize = 11;

// Function to count the number of letters in a given number (up to 1000)
fn count_letters(n: u32) -> usize {
    return if n < 10 {
        // Return the letter count for numbers 1-9
        ONES[n as usize]
    } else if n < 20 {
        // Return the letter count for TEENS (10-19)
        TEENS[(n - 10) as usize]
    } else if n < 100 {
        // For numbers 20-99, calculate the letter count by summing the letter counts for TENS and ONES
        TENS[(n / 10) as usize] + ONES[(n % 10) as usize]
    } else if n < 1000 {
        // For numbers 100-999
        // Calculate the letter count for the hundreds place
        let hundred = ONES[(n / 100) as usize] + HUNDRED_CHARS;
        // Get the remaining two-digit number
        let remainder = n % 100;

        if remainder != 0 {
            // If there is a remainder (e.g., not an exact hundred), calculate the letter count for the remainder
            hundred + AND_CHARS + count_letters(remainder)
        } else {
            // If there is no remainder, return the letter count for the exact hundred
            hundred
        }
    } else {
        // For number 1000, return the letter count for "one thousand"
        ONE_THOUSAND_CHARS
    }
}

fn main() {
    let start = Instant::now();

    // Calculate the sum of letter counts for numbers 1 to 1000 using the count_letters function
    let sum_of_letter_counts: usize = (1..=1_000).map(|n| count_letters(n)).sum::<usize>();

    let end = Instant::now();

    // Print the solution (sum of letter counts) and the elapsed time
    println!("Solution: {}", sum_of_letter_counts);
    println!("Elapsed time: {:?}", end - start);
}
