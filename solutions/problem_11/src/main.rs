use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::time::Instant;

/// Reads a grid of numbers from a file and returns it as a 2D vector.
///
/// # Arguments
///
/// * `filename` - The name of the file to read from.
///
/// # Returns
///
/// A 2D vector containing the numbers from the file.
fn read_grid_from_file(filename: &str) -> Result<Vec<Vec<u32>>, Error> {
    let file = File::open(filename)?;
    let file_reader = BufReader::new(file);

    let mut grid = Vec::new();

    for line in file_reader.lines() {
        let line = line?;
        let row: Vec<u32> = line
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect();
        grid.push(row);
    }

    Ok(grid)
}

/// Calculates the maximum product of adjacent numbers in a horizontal direction within the grid.
fn max_product_horizontal(grid: &Vec<Vec<u32>>, num_adjacent: usize) -> u32 {
    let mut max_product: u32 = 0;
    for row in grid {
        for row_idx in 0..=row.len() - num_adjacent {
            let mut product: u32 = 1;
            for i in 0..num_adjacent {
                product *= row[row_idx + i];
            }
            max_product = max(max_product, product);
        }
    }

    max_product
}

/// Calculates the maximum product of adjacent numbers in a vertical direction within the grid.
fn max_product_vertical(grid: &Vec<Vec<u32>>, num_adjacent: usize) -> u32 {
    let mut max_product = 0;

    for col_idx in 0..grid[0].len() {
        for row_idx in 0..grid.len() - num_adjacent {
            let mut product = 1;
            for i in 0..num_adjacent {
                product *= grid[row_idx + i][col_idx];
            }
            max_product = max(max_product, product);
        }
    }

    max_product
}

/// Calculates the maximum product of adjacent numbers in a diagonal direction from left to right within the grid.
fn max_product_diagonal_lr(grid: &Vec<Vec<u32>>, num_adjacent: usize) -> u32 {
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut max_product: u32 = 0;

    for i in 0..=num_rows - num_adjacent {
        for j in 0..=num_cols - num_adjacent {
            let mut product: u32 = 1;
            for k in 0..num_adjacent {
                product *= grid[i + k][j + k];
            }
            max_product = max(max_product, product);
        }
    }

    max_product
}

/// Calculates the maximum product of adjacent numbers in a diagonal direction from right to left within the grid.
fn max_product_diagonal_rl(grid: &Vec<Vec<u32>>, num_adjacent: usize) -> u32 {
    let mut max_product: u32 = 0;
    for i in (num_adjacent - 1)..grid.len() {
        for j in 0..=(grid[i].len() - num_adjacent) {
            let mut product: u32 = 1;
            for k in 0..num_adjacent {
                product *= grid[i - k][j + k];
            }
            if product > max_product {
                max_product = product;
            }
        }
    }
    max_product
}

fn max_product_in_grid(grid: &Vec<Vec<u32>>, adjacent_numbers: usize) -> u32 {
    let horizontal = max_product_horizontal(grid, adjacent_numbers);
    let vertical = max_product_vertical(grid, adjacent_numbers);
    let diagonal_lr = max_product_diagonal_lr(grid, adjacent_numbers);
    let diagonal_rl = max_product_diagonal_rl(grid, adjacent_numbers);

    max(horizontal, max(vertical, max(diagonal_lr, diagonal_rl)))
}

fn main() {
    let start = Instant::now();
    if let Ok(grid) = read_grid_from_file("resources/data/p11_input.txt") {
        println!("Solution: {}", max_product_in_grid(&grid, 4));
    }
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
