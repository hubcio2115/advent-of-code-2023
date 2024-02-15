use std::{str::Lines, usize};

struct Game(usize, usize, usize);

impl Game {
    pub fn new() -> Self {
        Self(0, 0, 0)
    }

    pub fn from(red: usize, green: usize, blue: usize) -> Self {
        Self(red, green, blue)
    }
}

#[allow(dead_code)]
pub fn part_1(lines: Lines) -> usize {
    let game = Game::from(12, 13, 14);

    lines
        .filter_map(|line| {
            let mut split = line.split(": ");

            let index = split.next().unwrap().split(' ').last().unwrap();
            let is_game_possible = split.next().unwrap().split("; ").all(|round| {
                round.split(", ").all(|cube| {
                    let mut split = cube.split(' ');

                    let number_of_cubes: usize = split.next().unwrap().parse().unwrap();
                    let color_of_cube = split.next().unwrap();

                    match color_of_cube {
                        "red" => game.0 >= number_of_cubes,
                        "green" => game.1 >= number_of_cubes,
                        "blue" => game.2 >= number_of_cubes,
                        _ => unreachable!(),
                    }
                })
            });

            if is_game_possible {
                index.parse::<usize>().ok()
            } else {
                None
            }
        })
        .sum()
}

#[allow(dead_code)]
pub fn part_2(lines: Lines) -> usize {
    lines.fold(0, |acc, line| {
        let split = line.split(": ");
        let mut game = Game::new();

        let rounds = split.last().unwrap().split("; ");
        for round in rounds {
            let cubes = round.split(", ");
            for cube in cubes {
                let mut split = cube.split(' ');

                let number_of_cubes: usize = split.next().unwrap().parse().unwrap();
                let color_of_cube = split.next().unwrap();

                match color_of_cube {
                    "red" => game.0 = game.0.max(number_of_cubes),
                    "green" => game.1 = game.1.max(number_of_cubes),
                    "blue" => game.2 = game.2.max(number_of_cubes),
                    _ => unreachable!(),
                };
            }
        }

        acc + (game.0 * game.1 * game.2)
    })
}

#[cfg(test)]
mod test {
    use super::{part_1, part_2};

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part_1_works() {
        let result = part_1(TEST_INPUT.lines());

        assert_eq!(result, 8);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(TEST_INPUT.lines());

        assert_eq!(result, 2286);
    }
}
