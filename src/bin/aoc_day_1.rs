use std::fs;
use std::path::PathBuf;

fn turn_left() {}

fn turn_right(dail: i32, amount: i32) -> i32 {
    (dail + amount) % 100
}

fn main() {
    let root = env!("CARGO_MANIFEST_DIR");
    let path = PathBuf::from(root).join("data").join("aoc_1.txt");
    let binding = fs::read_to_string(path).unwrap();
    let lines = binding.lines();

    let mut dail = 50;
    let mut n_zero = 0;
    let mut max_value = -1;
    for line in lines {
        let chars = line.chars();
        let direction: &String = &chars.clone().take(1).collect();
        let amount: &String = &chars.skip(1).collect();
        let amount_num: i32 = amount.parse().unwrap();

        // println!("{} {}", direction, amount_num);

        // if direction == "L" {

        // }
        if amount_num > max_value {
            max_value = amount_num
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn turn_right_to_lt_99() {
        let result = turn_right(30, 20);
        let expected = 50;
        assert_eq!(expected, result);
    }

    #[test]
    fn turn_right_to_gt_99() {
        let result = turn_right(95, 60);
        let expected = 55;
        assert_eq!(expected, result);
    }

    #[test]
    fn turn_right_to_zero() {
        let result = turn_right(52, 48);
        let expected = 0;
        assert_eq!(expected, result);
    }
}
