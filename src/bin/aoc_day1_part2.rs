// Advent of Code 2025 - Day 1 - Part 2 - https://adventofcode.com/2025/day/1
use std::fs;
use std::path::PathBuf;

fn turn_left(dial: &mut i32, turn: i32, n_zero: &mut i32) {
    for _ in 0..turn {
        if *dial == 0 {
            *dial = 99
        } else {
            *dial -= 1;
        }
        if *dial == 0 {
            *n_zero += 1;
        }
    }
}

fn turn_right(dial: &mut i32, turn: i32, n_zero: &mut i32) {
    for _ in 0..turn {
        if *dial == 99 {
            *dial = 0
        } else {
            *dial += 1;
        }
        if *dial == 0 {
            *n_zero += 1;
        }
    }
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

        if direction == "L" {
            turn_left(&mut dail, amount_num, &mut n_zero);
        } else {
            turn_right(&mut dail, amount_num, &mut n_zero);
        }
        println!(
            "line: {} direction: {} amount_num: {} dail: {}",
            line, direction, amount_num, dail
        );
    }
    println!("Secret code: {}", n_zero)
}
