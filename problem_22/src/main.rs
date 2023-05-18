use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::time::Instant;

/// Read names from a file and return a sorted BTreeSet.
fn read_names_from_file(filename: &str) -> Result<BTreeSet<String>, Error> {
    // Open the file
    let file = File::open(filename)?;
    // Wrap the file reader in a buffered reader
    let reader = BufReader::new(file);

    // Create an empty BTreeSet to store the names
    let mut sorted_names: BTreeSet<String> = BTreeSet::new();

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?; // Unwrap the line or propagate any errors

        // Split the line by commas to extract individual names
        let names = line.split(',');

        // Process each name and insert it into the BTreeSet
        for name in names {
            sorted_names.insert(name.trim_matches('"').to_string());
        }
    }

    Ok(sorted_names)
}

/// Calculate the total name score for a given set of names.
fn total_name_score(names: BTreeSet<String>) -> u32 {
    let alphabet: HashMap<char, usize> = ('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect();

    let mut total_name_score = 0;

    // Iterate over each name and its position
    for (pos, name) in names.into_iter().enumerate() {
        let name_score = name
            .chars()
            .map(|c| alphabet[&c] as u32)
            .sum::<u32>();

        total_name_score += name_score * (pos as u32 + 1);
    }

    total_name_score
}

fn main() {
    // Start measuring the elapsed time
    let start = Instant::now();

    match read_names_from_file("resources/data/p22_input.txt") {
        Ok(names) => println!("Solution: {}", total_name_score(names)),
        Err(err) => eprintln!("Error: {}", err),
    }

    // Stop measuring the elapsed time
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
