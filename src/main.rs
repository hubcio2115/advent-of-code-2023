use std::{env::args, fs, process::exit};
mod days;

fn main() {
    let Some(file_path) = args().nth(1) else {
        eprintln!("You didn't provide a file.");
        exit(1);
    };

    let Ok(file) = fs::read_to_string(file_path) else {
        eprintln!("Couldn't read the input file.");
        exit(1);
    };

    let result_part1 = days::day_3::part_1(file.lines());
    println!("{}", result_part1);
}
