use std::time::Instant;

/// Finds a Pythagorean triplet whose elements add up to the given sum.
///
/// # Arguments
///
/// * `sum` - The desired sum of the triplet.
///
/// # Returns
///
/// An `Option` containing the Pythagorean triplet as a tuple `(a, b, c)`. If no triplet is found, `None` is returned.
fn pythagorean_triplet(sum: u32) -> Option<(u32, u32, u32)> {
    // Calculate the limits for 'a' and 'b' based on the sum
    let limit_a = sum / 3;
    let limit_b = sum / 2;

    // Iterate over possible values of 'a'
    for a in 1..=limit_a {
        let a_squared = a * a;

        // Iterate over possible values of 'b' starting from 'a'
        for b in a..=limit_b {
            let b_squared = b * b;
            let c = sum - a - b;

            // If 'c' is negative, terminate inner loop and proceed to next 'a'
            if c < 0 {
                break;
            }

            let c_squared = c * c;

            // Check if the sum of squares of 'a' and 'b' equals the square of 'c'
            if a_squared + b_squared == c_squared {
                return Some((a, b, c));
            }
        }
    }

    // No Pythagorean triplet found
    None
}

fn main() {
    let start = Instant::now();

    // Find a Pythagorean triplet with a sum of 1000
    if let Some(triplet) = pythagorean_triplet(1_000) {
        let product = triplet.0 * triplet.1 * triplet.2;
        println!("Solution: {}", product);
    } else {
        eprintln!("Pythagorean triplet not found!");
    }

    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
