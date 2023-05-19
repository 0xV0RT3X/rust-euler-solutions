use std::time::Instant;

const MIN: u64 = 1020304050607080900;
const MAX: u64 = 1929394959697989990;

// 1_2_3_4_5_6_7_8_9_0
fn solution() {
    for i in (1_000_000_000_u64..10_000_000_000).step_by(10) {
        let i_pow = i.pow(2);
        if compare_n_with_pattern(i_pow) {
            println!("Solution: {}", i);
            break;
        }
    }
}

fn compare_n_with_pattern(n: u64) -> bool {
    if n < MIN || n > MAX {
        return false;
    }

    let n_str: String = n.to_string().chars().step_by(2).collect();
    n_str == "1234567890"
}

fn main() {
    let start = Instant::now();
    solution();
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
