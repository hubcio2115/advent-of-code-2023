use std::{collections::HashMap, str::Lines, usize};

use regex::Regex;

pub fn part_1(lines: Lines) -> usize {
    let pattern = Regex::new(r"\d").unwrap();

    lines
        .map(|line| -> usize {
            let mut number_matches = pattern.find_iter(line);

            let left = number_matches.next().unwrap().as_str();
            let right = match number_matches.last() {
                Some(value) => value.as_str(),
                None => left,
            };

            format!("{}{}", left, right)
                .parse()
                .unwrap()
        })
        .sum()
}

pub fn part_2(lines: Lines) -> usize {
    let digits: HashMap<String, usize> = "one,two,three,four,five,six,seven,eight,nine"
        .split(',')
        .map(|s| s.to_string())
        .zip(1..=9)
        .chain((0..=9).map(|i| (i.to_string(), i)))
        .collect();

    let res: usize = lines
        .map(|line| {
            let first_digit = digits
                .iter()
                .map(|(digit_string, value)| (line.find(digit_string), *value))
                .filter(|(index, _)| index.is_some())
                .min()
                .unwrap()
                .1;
            let last_digit = digits
                .iter()
                .map(|(digit_string, &value)| (line.rfind(digit_string), value))
                .max()
                .unwrap()
                .1;
            first_digit * 10 + last_digit
        })
        .sum();

    return res;
}

#[cfg(test)]
mod test {
    use crate::days::day_1::{part_1, part_2};

    #[test]
    fn part_1_works() {
        let lines = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            .lines();

        let result = part_1(lines);

        assert_eq!(result, 142);
    }

    #[test]
    fn part_2_works() {
        let lines = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .lines();

        let result = part_2(lines);

        assert_eq!(result, 281);
    }
}
