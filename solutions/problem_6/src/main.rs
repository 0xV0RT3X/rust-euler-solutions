use std::time::Instant;

fn main() {
    let start = Instant::now();

    // Initialize variables for sum of squares and square of the sum
    let (sum_of_squares, square_of_the_sum) = (1_u32..=100)
        .fold((0, 0), |(sum_of_squares, sum), x| {
            // Accumulate the sum of squares and the sum of numbers
            (sum_of_squares + x * x, sum + x)
        });

    // Compute the square of the sum
    let square_of_the_sum = square_of_the_sum.pow(2);

    // Compute the solution by subtracting the sum of squares from the square of the sum
    let solution = square_of_the_sum - sum_of_squares;

    let end = Instant::now();

    // Print the solution and elapsed time
    println!("Solution: {}", solution);
    println!("Elapsed time: {:?}", end - start);
}
