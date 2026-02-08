use indicatif::ProgressIterator;
use itertools::{self, Itertools};
use std::char::MAX;
use std::fs;
use std::path;

// WORKS BUT IS TOO SLOW!
// fn max_jolt(row: String) -> i64 {
//     let n_batteries: usize = 12;
//     let mut max_value: i64 = 0;
//     let combs = row.chars().combinations(n_batteries);
//     for comb in combs.into_iter() {
//         let comb: i64 = comb.into_iter().collect::<String>().parse().unwrap();
//         if comb > max_value {
//             max_value = comb
//         }
//     }
//     max_value
// }

fn max_jolt(row: String) -> i64 {
    const MAX_LENGTH: usize = 12;
    let mut n_to_drop = row.chars().count() - MAX_LENGTH;

    let mut stack: Vec<char> = Vec::new();
    for char in row.chars() {
        while n_to_drop > 0 && !stack.is_empty() && *stack.last().unwrap() < char {
            stack.pop();
            n_to_drop -= 1
        }
        stack.push(char);
    }

    let mut result: String = stack.into_iter().collect();
    result.truncate(MAX_LENGTH);
    result.parse::<i64>().unwrap()
}

fn main() {
    let root_dir = env!("CARGO_MANIFEST_DIR");
    let path = path::PathBuf::from(root_dir)
        .join("data")
        .join("aoc_day_3.txt");
    let data: Vec<String> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();
    let mut total_jolts: i64 = 0;
    for line in data.into_iter().progress() {
        let jolt = max_jolt(line);
        total_jolts += jolt
    }
    println!("total_jolts: {}", total_jolts);
}

#[test]
fn test_max_jolt() {
    let test_cases: Vec<(String, i64)> = vec![
        ("987654321111111".to_string(), 987654321111),
        ("811111111111119".to_string(), 811111111119),
        ("234234234234278".to_string(), 434234234278),
        ("818181911112111".to_string(), 888911112111),
    ];
    for (row, result) in test_cases {
        assert_eq!(max_jolt(row), result)
    }
}
