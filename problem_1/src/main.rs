use std::time::Instant;

fn main() {
    // Start the timer to measure the execution time
    let start = Instant::now();

    // Print the solution, which is the sum of multiples of 3 and 5 within the range of 1 to 999
    // The range is generated using the inclusive range operator '..=' which includes both ends of the range
    // The filter closure checks if a number is divisible by 3 or 5 using the remainder operator '%'
    // The sum::<u32>() function calculates the sum of all the filtered numbers
    println!("Solution: {}", (1_u32..=999).filter(|n| n % 3 == 0 || n % 5 == 0).sum::<u32>());

    // Stop the timer and calculate the elapsed time
    let end = Instant::now();

    // Print the elapsed time, which is the duration between start and end
    println!("Elapsed time: {:?}", end - start);
}
