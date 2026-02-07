use std::cmp::max;
use std::fs;
use std::path;

fn max_jolt(row: String) -> i32 {
    let mut max_value: i32 = 0;
    let chars: Vec<char> = row.chars().collect();
    for i in 0..(chars.len() - 1) {
        for j in (i + 1)..chars.len() {
            let jolt: i32 = format!("{}{}", chars[i], chars[j]).parse().unwrap();
            if jolt > max_value {
                max_value = jolt
            }
            println!("{} {}", jolt, max_value);
        }
    }
    max_value
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
    let mut total_jolts: i32 = 0;
    for line in data {
        let jolt = max_jolt(line);
        total_jolts += jolt
    }
    println!("total_jolts: {}", total_jolts);
}

#[test]
fn test_max_jolt() {
    let test_cases: Vec<(String, i32)> = vec![
        ("987654321111111".to_string(), 98),
        ("811111111111119".to_string(), 89),
        ("234234234234278".to_string(), 78),
        ("818181911112111".to_string(), 92),
        ("819181911112111".to_string(), 99),
    ];
    for (row, result) in test_cases {
        assert_eq!(max_jolt(row), result)
    }
}

// 17374
// 17554
