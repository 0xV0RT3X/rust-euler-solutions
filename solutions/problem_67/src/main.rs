use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::time::Instant;

fn read_data_from_file(filename: &str) -> Result<Vec<Vec<u64>>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<Vec<u64>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<u64> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        lines.push(numbers);
    }

    Ok(lines)
}

// Algorithm:
//   1.            2.             3.           4.
//
//    3            3              3            23
//   7 4          7  4          20 19
//  2 4 6 ----> 10 13 15 ---->          ---->
// 8 5 9 3
//
// 1.
// 8 is larger than 5 -> add 8 to 2 -> 10
// 9 is larger than 5 -> add 9 to 4 -> 13
// 9 is larger than 3 -> add 8 to 7 -> 15
//
// 2.
// 13 is larger than 10 -> add 13 to 7 -> 20
// 15 is larger than 13 -> add 15 to 4 -> 19
//
// 3. 20 is larger than 19 -> add 20 to 3 -> 23
//
// 4. 23 is the result

fn max_path_sum(data: &mut Vec<Vec<u64>>) -> u64 {
    for row in (0..data.len() - 1).rev() {
        for col in 0..data[row].len() {
            data[row][col] += std::cmp::max(data[row+1][col], data[row+1][col+1]);
        }
    }

    data[0][0]
}

fn main() {
    let start = Instant::now();
    match read_data_from_file("resources/data/p67_input.txt") {
        Ok(mut data) => {
            println!("Solution: {} ", max_path_sum(&mut data));
        }
        Err(err) => eprintln!("Error: {}", err)
    }
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}