use std::collections::HashMap;
use std::time::Instant;

/// Checks if a triangle with sides `a`, `b`, and `c` is a right angle triangle.
fn is_right_angle_triangle(a: u64, b: u64, c: u64) -> bool {
    a * a + b * b == c * c
}

/// Finds the perimeter with the maximum number of right angle triangle solutions.
fn max_solutions(perimeter: u64) -> u64 {
    let mut solutions: HashMap<u64, u32> = HashMap::new();

    // Calculate the perimeter of right angle triangles by iterating over side lengths.
    for a in 1_u64..=perimeter {
        for b in 1_u64..=perimeter {
            // Calculate the third side length using the Pythagorean theorem.
            let c = ((a * a + b * b) as f64).sqrt() as u64;

            // Check if it forms a right angle triangle.
            if is_right_angle_triangle(a, b, c) {
                // Calculate the perimeter.
                let p = a + b + c;

                if p > perimeter {
                    // Perimeter exceeded, move to the next iteration.
                    break;
                }

                // Count the number of solutions for each perimeter.
                if let Some(cnt) = solutions.get_mut(&p) {
                    // Increment the count if the perimeter is already in the map.
                    *cnt += 1;
                } else {
                    // Add the perimeter to the map with a count of 1.
                    solutions.insert(p, 1);
                }
            }
        }
    }

    // Find the perimeter with the maximum count of solutions.
    let (max_p, _) = solutions.iter().max_by_key(|&(_, cnt)| cnt).unwrap();
    *max_p // Return the maximum perimeter.
}

fn main() {
    let start = Instant::now();
    println!("{}", max_solutions(1000));
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
