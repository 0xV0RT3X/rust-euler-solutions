use std::time::Instant;

// Function to calculate the factorial of a number
fn factorial(n: u32) -> u32 {
    (1..=n).product()
}

fn main() {
    let start = Instant::now();

    // Create a vector of digits from 0 to 9
    let mut digits: Vec<u32> = (0..=9).collect();
    let mut result = String::new();

    let mut remaining = 999_999;

    // Iterate from 10 down to 1 (inclusive)
    for i in (1..=10).rev() {
        // Calculate the index of the digit to be chosen based on the remaining permutations
        let index = remaining / factorial(i - 1) as u32;

        // Remove the digit at the calculated index from the vector
        let digit = digits.remove(index as usize);

        // Append the chosen digit to the result string
        result.push((b'0' + digit as u8) as char);

        // Update the remaining permutations by taking the remainder
        remaining %= factorial(i - 1) as u32;
    }

    println!("Solution: {}", result);

    let end = Instant::now();
    println!("Elapsed time: {:?}", end - start);
}
