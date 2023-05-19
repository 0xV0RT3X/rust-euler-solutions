use std::fs::read_to_string;
use std::time::Instant;
use top_english_words::get_words;

const KEY_LENGTH: usize = 3;
const ENGLISH_WORDS_REQUIRED: u32 = 50;

fn generate_keys() -> Vec<[u8; KEY_LENGTH]> {
    let mut keys = Vec::new();
    for i in b'a'..=b'z' {
        for j in b'a'..=b'z' {
            for k in b'a'..=b'z' {
                keys.push([i, j, k]);
            }
        }
    }
    keys
}

fn contains_common_english_words(text: &str, n: u32) -> bool {
    let mut cnt = 0;
    for word in get_words::<String>() {
        if cnt == n {
            return true;
        }

        if text.contains(&word) {
            cnt += 1;
        }
    }

    false
}

fn decrypt(encrypted: &[u8], key: &[u8; KEY_LENGTH]) -> String {
    let mut decrypted = String::new();
    for (i, &byte) in encrypted.iter().enumerate() {
        let key_byte = key[i % KEY_LENGTH];
        decrypted.push(char::from(byte ^ key_byte));
    }
    decrypted
}

fn bruteforce(encrypted_bytes: Vec<u8>) {
    let possible_keys = generate_keys();

    for key in possible_keys {
        let decrypted_text = decrypt(&encrypted_bytes, &key);

        if contains_common_english_words(&decrypted_text, ENGLISH_WORDS_REQUIRED) {
            println!("{}", decrypted_text);
            println!("{}", decrypted_text.chars().map(|c| c as u32).sum::<u32>());
        }
    }
}

fn read_encrypted_bytes_from_file(encrypted_file: &str) -> Vec<u8> {
    read_to_string(encrypted_file)
        .expect("Failed to read file")
        .trim()
        .split(',')
        .map(|byte_str| byte_str
            .parse()
            .expect("Invalid byte value"))
        .collect()
}

// - Key consists of 3 lowercase characters xxx where x >= a && x <= z
// - If the password is shorter than the message -> repeat the key cyclically throughout the message!
//
// Example:
// - Message to encrypt: Please encrypt me!
// - Key:                Secret
//
// 1. Convert message to byte vec
// - Byte representation of message:
//  P    l    e    a   s    e        e    n    c  r     y   p    t         m    e    !
// [80, 108, 101, 97, 115, 101, 32, 101, 110, 99, 114, 121, 112, 116, 32, 109, 101, 33]
//
// 2. Convert key to to byte vec:
// - Byte representation of key:
//  S    e    c   r    e    t
// [83, 101, 99, 114, 101, 116]
//
// 3. XOR m[mn] k[kn]
// m  ... message
// mn ... message length
//
// k  ... key
// kn ... key length
//
// XOR:
// Message: 80  108  101 97  115 101 32  101 110 99  114 121 112 116 32  109 101 33
// Key:     83  101  99  114 101 116 83  101 99  114 101 116 83  101 99  114 101 116
// Result:  3   9    6   19  22  17  115 0   13  17  23  13  35  17  67  31  0   85
//                                ^                       ^                      ^
//                               EOK                     EOK                    EOK
// EOK ... End of key
fn main() {
    let start = Instant::now();
    let bytes = read_encrypted_bytes_from_file("resources/data/p59_input.txt");
    bruteforce(bytes);
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
