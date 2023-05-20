use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::time::Instant;

/// Read words from a file and return a sorted set of unique words.
///
/// # Arguments
///
/// * `filename` - A string slice representing the path to the file.
///
/// # Returns
///
/// * `Result<BTreeSet<String>, Error>` - A Result containing the set of words if successful,
///   or an Error if an I/O error occurs.
///
fn read_words_from_file(filename: &str) -> Result<BTreeSet<String>, Error> {
    let file = File::open(filename)?; // Handle file open error explicitly
    let reader = BufReader::new(file);

    let mut words = BTreeSet::new();
    for line in reader.lines() {
        let line = line?; // Handle read error explicitly

        for word in line.split(",") {
            words.insert(word.replace("\"", ""));
        }
    }

    Ok(words)
}

/// Count the number of triangle words in a given set of words.
///
/// # Arguments
///
/// * `words` - A reference to a BTreeSet containing the words to be checked.
///
/// # Returns
///
/// * `usize` - The count of triangle words in the given set.
///
fn count_triangle_words(words: &BTreeSet<String>) -> usize {
    let alphabet: HashMap<char, u32> = ('A'..='Z')
        .enumerate()
        .map(|(index, letter)| (letter, (index + 1) as u32))
        .collect();

    let triangle_numbers: HashSet<u32> = (1..=20).map(|n| triangle_number(n)).collect();

    words
        .iter()
        .filter(|word| is_triangle_word(word, &alphabet, &triangle_numbers))
        .count()
}

/// Calculate the n-th triangle number.
///
/// # Arguments
///
/// * `n` - The index of the triangle number to calculate.
///
/// # Returns
///
/// * `u32` - The calculated triangle number.
///
fn triangle_number(n: u32) -> u32 {
    (n * (n + 1)) / 2
}

/// Check if a word is a triangle word.
///
/// # Arguments
///
/// * `word` - A reference to a String representing the word to be checked.
/// * `alphabet` - A reference to a HashMap containing character-to-value mappings.
/// * `triangle_numbers` - A reference to a HashSet containing triangle numbers.
///
/// # Returns
///
/// * `bool` - true if the word is a triangle word, false otherwise.
///
fn is_triangle_word(word: &String, alphabet: &HashMap<char, u32>, triangle_numbers: &HashSet<u32>) -> bool {
    let word_value = word.chars().map(|c| alphabet[&c]).sum(); // Calculate the word value

    triangle_numbers.contains(&word_value)
}

fn main() {
    let start = Instant::now();
    match read_words_from_file("resources/data/p42_input.txt") {
        Ok(words) => {
            let triangle_word_count = count_triangle_words(&words);
            println!("Solution: {}", triangle_word_count);
        }
        Err(err) => eprintln!("Error: {}", err),
    }
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
