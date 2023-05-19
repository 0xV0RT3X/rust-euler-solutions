use std::collections::HashMap;
use std::time::Instant;

/// Builds a digit map for the given number.
///
/// The digit map contains the count of each digit occurring in the number.
fn build_digit_map(n: usize) -> HashMap<char, usize> {
    let mut digit_map: HashMap<char, usize> = HashMap::new();

    let digits = n.to_string();
    for digit in digits.chars() {
        if !digit_map.contains_key(&digit) {
            digit_map.insert(digit, 1);
        } else {
            if let Some(occurrence) = digit_map.get_mut(&digit) {
                *occurrence += 1;
            }
        }
    }

    digit_map
}

/// Checks if a number meets the condition of having the same digits when multiplied by 2 to 6.
fn meets_condition(n: usize) -> bool {
    let digit_map = build_digit_map(n);

    for factor in 2..=6 {
        let product = n * factor;
        let mut product_digit_map = digit_map.clone();
        if !product_contains_digits(product, &mut product_digit_map) {
            return false;
        }
    }

    true
}

/// Checks if a product contains the same digits as the original number.
///
/// The `digit_map` parameter contains the count of each digit available for the product.
fn product_contains_digits(product: usize, digit_map: &mut HashMap<char, usize>) -> bool {
    let digits = product.to_string();

    if digits.len() > digit_map.values().sum::<usize>() {
        return false;
    }

    for digit in digits.chars() {
        if let Some(occurrence) = digit_map.get_mut(&digit) {
            if *occurrence == 0 {
                return false;
            }

            *occurrence -= 1;
        } else {
            return false;
        }
    }

    digit_map.values().sum::<usize>() == 0
}

/// Finds the smallest integer that meets the condition.
fn smallest_int() -> usize {
    let mut n = 1;
    while !meets_condition(n) {
        n += 1;
    }
    n
}

fn main() {
    let start = Instant::now();
    println!("Solution: {}", smallest_int());
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
