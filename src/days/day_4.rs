use std::{collections::HashMap, str::Lines, usize, vec};

use itertools::Itertools;

pub fn part_1(lines: Lines) -> usize {
    let games = lines.map(|line| {
        let mut row = line.split(": ").last().unwrap().split(" | ");

        let winning_numbers = row.next().unwrap().split(' ');
        let numbers = row
            .next()
            .unwrap()
            .split(' ')
            .filter(|value| !value.is_empty());

        (winning_numbers, numbers)
    });

    games
        .map(|(winning_numbers, numbers)| {
            numbers.fold(0, |acc, number| {
                if winning_numbers.clone().contains(&number) {
                    if acc == 0 {
                        return 1;
                    }

                    return acc * 2;
                }

                acc
            })
        })
        .sum::<usize>()
}

pub fn part_2(lines: Lines) -> usize {
    let games = lines.map(|line| {
        let mut row = line.split(": ").last().unwrap().split(" | ");

        let winning_numbers = row.next().unwrap().split(' ');
        let numbers = row
            .next()
            .unwrap()
            .split(' ')
            .filter(|value| !value.is_empty());

        (winning_numbers, numbers)
    });

    let mut card_copies: Vec<usize> = (0..games.clone().collect_vec().len())
        .map(|_| 1)
        .collect_vec();

    for (index, (winning_numbers, numbers)) in games.enumerate() {
        let number_of_matches = numbers.fold(0, |acc, number| {
            if winning_numbers.clone().contains(&number) {
                return acc + 1;
            }

            acc
        });

        for _ in 0..card_copies[index] {
            (index + 1..=number_of_matches + index).for_each(|j| {
                card_copies[j] += 1;
            });
        }
    }

    card_copies.iter().sum()
}

#[cfg(test)]
mod test {
    use crate::days::day_4::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT.lines()), 13);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT.lines()), 30);
    }
}
