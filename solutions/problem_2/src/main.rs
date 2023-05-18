use std::time::Instant;

fn main() {
    // Start the timer to measure the execution time
    let start = Instant::now();

    // Generate Fibonacci numbers and calculate the sum of even-valued terms
    let sum: u32 = (1..).scan((1, 2), |state, _| {
        let fib = state.0;
        *state = (state.1, state.0 + state.1);
        Some(fib)
    }).take_while(|&fib| fib <= 4_000_000)
        .filter(|&fib| fib % 2 == 0)
        .sum();

    // Stop the timer and calculate the elapsed time
    let end = Instant::now();

    // Print the solution, which is the sum of even-valued Fibonacci terms
    println!("Solution: {}", sum);

    // Print the elapsed time, which is the duration between start and end
    println!("Elapsed time: {:?}", end - start);
}
