use std::time::Instant;

// 1. Row: 21 22 23 24 25
// 2. Row: 20  7  8  9 10
// 3. Row: 19  6  1  2 11
// 4. Row: 18  5  4  3 12
// 5. Row  17 16 15 14 13
//
// Layers: l(x) = x/2
//
// Example: l(5) = 5/2 = 2
//
// Where each layer has 4 diagonals (Except the first one)
//
// Total diagonal values: 4*2+1 = 9 Diagonal values
//
// Diagonal values: Dv = l(x) * 4 + 1
//
// Pattern:
// 1    2   3   4   5   6   7   8   9   10  11  12  13  14  15  16  17  18  19  20  21  22  23  24  25
// ^        ^       ^       ^       ^               ^               ^               ^               ^
// |________|_______|_______|_______|_______________|_______________|_______________|_______________|
//     +2       +2      +2      +2          +4              +4              +4                 +4
// |________________________________|_______________________________________________________________|
//               4x                                                   4x
//
// Step increases by 2 after 4th iteration
fn spiral_diagonal_sum(size: u32) -> u32 {
    let mut current_num = 1;
    let mut sum = 1;
    let mut step = 2;

    for _ in 0..(size / 2) {
        // 4 iterations
        for _ in 0..4 {
            // Example:
            // At the beginning current_num is 1 and step 4
            // 1. Iteration: 1 + 2 = 3
            // 2. Iteration: 3 + 2 = 5
            // 3. Iteration: 5 + 2 = 7
            // 4. Iteration: 7 + 2 = 9
            current_num += step;
            sum += current_num;
        }
        // Increase step by 2 after 4th iteration
        step += 2;
    }

    sum
}

fn main() {
    let start = Instant::now();
    println!("{}", spiral_diagonal_sum(1_001));
    let end = Instant::now();

    println!("{:?}", end - start);
}
