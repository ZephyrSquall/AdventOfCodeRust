use crate::solver::{AdventOfCode, Solution};
use std::cmp::max;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2023,
    day: 2,
    title: "Cube Conundrum",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let mut possible_game_id_sum = 0;

    for line in input.lines() {
        let mut was_game_possible = true;

        let strip_game_prefix_str = line
            .strip_prefix("Game ")
            .expect("All lines should start with \"Game \"");
        let (game_id_str, games_str) = strip_game_prefix_str
            .split_once(':')
            .expect("All lines should have a colon");

        'game_loop: for game_str in games_str.split(';') {
            for draw_str in game_str.split(',') {
                let (number_str, color_str) = draw_str
                    // There is always an additional space at the start of a draw_str (i.e.
                    // " 9 red") to separate it from whatever came before it in the line. Trim it
                    // first so the only space left in draw_str is the one between the number and
                    // color.
                    .trim_ascii_start()
                    .split_once(' ')
                    .expect("All draws should be a number and color separated by a space");

                let number = number_str
                    .parse::<u8>()
                    .expect("Draw number should be a valid number");
                let limit = match color_str {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => panic!("Invalid color encountered"),
                };

                if number > limit {
                    was_game_possible = false;
                    break 'game_loop;
                }
            }
        }

        if was_game_possible {
            let game_id = game_id_str
                .parse::<u32>()
                .expect("Game id should be a valid number");
            possible_game_id_sum += game_id;
        }
    }

    Solution::U32(possible_game_id_sum)
}

fn solve_2(input: &str) -> Solution {
    let mut power_sum = 0;

    for line in input.lines() {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        let strip_game_prefix_str = line
            .strip_prefix("Game ")
            .expect("All lines should start with \"Game \"");
        let (_, games_str) = strip_game_prefix_str
            .split_once(':')
            .expect("All lines should have a colon");

        for game_str in games_str.split(';') {
            for draw_str in game_str.split(',') {
                let (number_str, color_str) = draw_str
                    .trim_ascii_start()
                    .split_once(' ')
                    .expect("All draws should be a number and color separated by a space");

                let number = number_str
                    .parse::<u32>()
                    .expect("Draw number should be a valid number");
                let min_color_ref = match color_str {
                    "red" => &mut min_red,
                    "green" => &mut min_green,
                    "blue" => &mut min_blue,
                    _ => panic!("Invalid color encountered"),
                };

                *min_color_ref = max(*min_color_ref, number);
            }
        }

        let power = min_red * min_green * min_blue;
        power_sum += power;
    }

    Solution::U32(power_sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            Solution::U8(8)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            Solution::U16(2286)
        );
    }
}
