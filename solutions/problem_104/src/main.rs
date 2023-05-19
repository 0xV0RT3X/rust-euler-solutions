use std::collections::HashMap;
use std::time::Instant;
use num::{BigUint, Num};

fn is_pandigital(str: &str) -> bool {
    let mut found_digits: HashMap<char, bool> = ('1'..='9').map(|n| (n, false)).collect();
    for c in str.chars() {
        if let Some(_) = found_digits.insert(c, true) {
            false;
        }
    }


    found_digits.values().all(|&value| value)
}

fn main() {
    let start = Instant::now();
    let mut a = BigUint::from_str_radix(&"160500643816367088", 10).unwrap();
    let mut b = BigUint::from_str_radix(&"259695496911122585", 10).unwrap();

    let mut k = 85;
    loop {

        let next = &a + &b;
        a = std::mem::replace(&mut b, next.clone());

        let str = a.to_string();
        let digits = str.len();

        let first_nine = &str[..9];
        let last_nine = &str[digits - 9..];
        if is_pandigital(first_nine) && is_pandigital(last_nine) {
            break;
        }

        k += 1;
    }
    let end = Instant::now();

    println!("Solution: {}", k);
    println!("Elapsed time: {:?}", end - start);
}