use std::time::Instant;

const LAST_TEN_DIGITS: u64 = 10_000_000_000;
const TARGET_EXPONENT: u64 = 7_830_457;
const MULTIPLIER: u64 = 28_433;

fn calculate_last_10_digits() -> u64 {
    let mut power_of_two = 1;
    let mut exponent = 0;

    while exponent < TARGET_EXPONENT {
        power_of_two = (power_of_two * 2) % LAST_TEN_DIGITS;
        exponent += 1;
    }

    let last_10_digits = (MULTIPLIER * power_of_two + 1) % LAST_TEN_DIGITS;

    last_10_digits
}

fn main() {
    let start = Instant::now();
    println!("Solution: {}", calculate_last_10_digits());
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
