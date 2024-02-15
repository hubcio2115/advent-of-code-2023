#![allow(clippy::needless_return)]

use std::fs;
mod days;

fn main() {
    if let Ok(file) = fs::read_to_string("./input.txt") {
        let lines = file.lines();

        let result_part1 = days::day_2::part_1(lines.clone());

        println!("{}", result_part1);

        let result_part2 = days::day_2::part_2(lines.clone());

        println!("{}", result_part2);
    } else {
        eprintln!("Couldn't read the input file.")
    }
}
