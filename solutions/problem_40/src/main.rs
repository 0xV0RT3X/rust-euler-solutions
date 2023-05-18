use std::time::Instant;

/// Concatenates integers from 0 to `n` into a single string.
fn concat_ints(n: u32) -> String {
    (0..=n)
        .map(|n| n.to_string())
        .collect()
}

fn main() {
    let start = Instant::now();

    // Create an iterator that generates powers of 10: 1, 10, 100, ...
    let iter_10x = (1..=7).scan(1, |state, _| {
        let current_step = *state;
        *state *= 10;
        Some(current_step)
    }).step_by(1);

    let mut product = 1;
    let concat_str = concat_ints(1000000);

    // Iterate over the powers of 10 and calculate the product of digits in the concatenated string.
    for i in iter_10x {
        // Extract the character at index `i` from the string and convert it to a digit.
        let digit = concat_str.chars().nth(i).unwrap().to_digit(10).unwrap();
        product *= digit;
    }

    let end = Instant::now();

    println!("Solution: {}", product);
    println!("Elapsed time: {:?}", end - start);
}
