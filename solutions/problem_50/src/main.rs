use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::time::Instant;

/// Reads a list of prime numbers from a file.
///
/// # Arguments
///
/// * `filename` - The path to the file containing the prime numbers.
///
/// # Returns
///
/// * A `Result` containing a `Vec` of prime numbers on success, or an `Error` on failure.
fn read_primes_from_file(filename: &str) -> Result<Vec<u64>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.and_then(|number| {
                number
                    .parse()
                    .map_err(|e| Error::new(ErrorKind::InvalidData, e))
            })
        })
        .collect()
}

/// Finds the prime number below one million that can be written as the sum of the most consecutive primes.
///
/// # Arguments
///
/// * `primes` - A slice of prime numbers.
///
/// # Returns
///
/// * An `Option` containing a tuple with the prime number and the number of consecutive primes that form the sum, or `None` if no prime is found.
fn longest_sum_of_consecutive_primes(primes: &[u64]) -> Option<(u64, usize)> {
    let mut longest_addends = 0;
    let mut corresponding_prime = 0;

    for (n, &prime) in primes.iter().enumerate() {
        let mut sum = prime;
        let mut addends = 1;

        for &inner_prime in primes.iter().skip(n + 1) {
            sum += inner_prime;
            addends += 1;

            if sum > 999_983 {
                break;
            } else if primes.binary_search(&sum).is_ok() && addends > longest_addends {
                longest_addends = addends;
                corresponding_prime = sum;
            }
        }
    }

    if corresponding_prime != 0 {
        Some((corresponding_prime, longest_addends))
    } else {
        None
    }
}

fn main() {
    let start = Instant::now();

    if let Ok(primes) = read_primes_from_file("resources/data/primes_bellow_one_million.txt") {
        if let Some((prime, addends)) = longest_sum_of_consecutive_primes(&primes) {
            println!("Solution: Prime: {}, Addends: {}", prime, addends);
        } else {
            println!("No prime found.");
        }
    } else {
        eprintln!("Error reading primes from file.");
    }

    let end = Instant::now();
    println!("Elapsed time: {:?}", end - start);
}
