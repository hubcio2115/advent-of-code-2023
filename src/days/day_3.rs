// WARNING: This code does not work!

use std::{str::Lines, usize};

const SYMBOLS: [char; 10] = ['#', '@', '*', '=', '+', '%', '$', '-', '/', '&'];
const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[derive(Debug)]
struct PartNumber {
    value: usize,
    x1: usize,
    x2: usize,
    y: usize,
}

impl PartNumber {
    pub fn from(value: usize, x1: usize, x2: usize, y: usize) -> Self {
        Self { value, x1, x2, y }
    }
}

pub fn part_1(lines: Lines) -> usize {
    let map = lines.map(|line| line.chars());

    let mut coords_of_symbols: Vec<(usize, usize)> = vec![];
    let mut part_numbers: Vec<PartNumber> = vec![];
    for (y, row) in map.enumerate() {
        for (x, char) in row.clone().enumerate() {
            if SYMBOLS.contains(&char) {
                coords_of_symbols.push((x, y));
            }
        }

        let chars = row.collect::<Vec<_>>();

        let mut x = 0;
        while x < chars.len() {
            if DIGITS.contains(&chars[x]) {
                let mut number_string_value = String::from("");
                for (i, char) in chars.iter().skip(x).enumerate() {
                    if !DIGITS.contains(char) {
                        x += i;
                        break;
                    }

                    number_string_value.push(*char);
                }

                part_numbers.push(PartNumber::from(
                    number_string_value.parse::<usize>().unwrap(),
                    x - number_string_value.len(),
                    x - 1,
                    y,
                ));
                continue;
            }

            x += 1;
        }
    }

    let mut part_number_adjacent_to_symbols: Vec<&PartNumber> = vec![];
    for (x, y) in coords_of_symbols {
        for part_number in &part_numbers {
            let is_to_left_or_right =
                (part_number.x2 == x - 1 || part_number.x1 == x + 1) && part_number.y == y;

            if is_to_left_or_right {
                part_number_adjacent_to_symbols.push(part_number);
                continue;
            }

            let part_number_coordinates_span = if part_number.x1 == 0 {
                (part_number.x1)..=(part_number.x2 + 1)
            } else {
                (part_number.x1 - 1)..=(part_number.x2 + 1)
            };

            let is_up_or_down = (part_number.y == y - 1 || part_number.y == y + 1)
                && part_number_coordinates_span.contains(&x);

            if is_up_or_down {
                part_number_adjacent_to_symbols.push(part_number);
            }
        }
    }

    part_number_adjacent_to_symbols
        .into_iter()
        .fold(0, |acc, part_number| acc + part_number.value)
}

pub fn part_2(lines: Lines) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part_1(input.lines()), 4361);
    }

    #[test]
    fn part_2_works() {}
}
