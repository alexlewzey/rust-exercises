// Advent of Code 2025 - Day 1 - https://adventofcode.com/2025/day/1
use std::fs;
use std::path::PathBuf;

fn turn_right(dail: i32, amount: i32) -> i32 {
    (dail + amount) % 100
}

fn turn_left(dial: i32, amount: i32) -> i32 {
    ((dial - amount) % 100 + 100) % 100
}



fn main() {
    let root = env!("CARGO_MANIFEST_DIR");
    let path = PathBuf::from(root).join("data").join("aoc_day_1.txt");
    let binding = fs::read_to_string(path).unwrap();
    let lines = binding.lines();

    let mut dail = 50;
    let mut n_zero = 0;
    for line in lines {
        let chars = line.chars();
        let direction: &String = &chars.clone().take(1).collect();
        let amount: &String = &chars.skip(1).collect();
        let amount_num: i32 = amount.parse().unwrap();

        // println!("{} {}", directio n, amount_num);

        if direction == "L" {
            dail = turn_left(dail, amount_num);
        } else {
            dail = turn_right(dail, amount_num);
        }
        if dail == 0 {
            n_zero += 1
        }
        println!("line: {} direction: {} amount_num: {} dail: {}", line, direction, amount_num, dail);
    }
    println!("Secret code: {}", n_zero)


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

    #[test]
    fn turn_left_to_lt_99() {
        let result = turn_left(30, 20);
        let expected = 10;
        assert_eq!(expected, result);
    }

    #[test]
    fn turn_left_to_gt_99() {
        let result = turn_left(50, 68);
        let expected = 82;
        assert_eq!(expected, result);
    }

    #[test]
    fn turn_left_to_zero() {
        let result = turn_left(10, 10);
        let expected = 0;
        assert_eq!(expected, result);
    }


    #[test]
    fn turn_left_full_to_zero() {
        let result = turn_left(10, 110);
        let expected = 0;
        assert_eq!(expected, result);
    }
}

