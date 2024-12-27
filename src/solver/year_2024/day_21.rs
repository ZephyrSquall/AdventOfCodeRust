use crate::solver::{AdventOfCode, Solution};
use rustc_hash::FxHashMap;
use std::cmp::min;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2024,
    day: 21,
    title: "Keypad Conundrum",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    solve(input, 2)
}

fn solve_2(input: &str) -> Solution {
    solve(input, 25)
}

fn solve(input: &str, depth: u8) -> Solution {
    struct Position {
        x: i64,
        y: i64,
    }

    // A representation of the parameters that go into the get_mc_to method on DirButs (Directional
    // Buttons) so that the results can be stored in a map for memoization.
    #[derive(PartialEq, Eq, Hash)]
    struct DirButMovementCostParameters {
        start_button: DirBut,
        end_button: DirBut,
        depth: u8,
    }

    enum NumericButton {
        Zero,
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        A,
    }
    impl NumericButton {
        fn new(letter: char) -> NumericButton {
            match letter {
                '0' => NumericButton::Zero,
                '1' => NumericButton::One,
                '2' => NumericButton::Two,
                '3' => NumericButton::Three,
                '4' => NumericButton::Four,
                '5' => NumericButton::Five,
                '6' => NumericButton::Six,
                '7' => NumericButton::Seven,
                '8' => NumericButton::Eight,
                '9' => NumericButton::Nine,
                'A' => NumericButton::A,
                _ => panic!("Unsupported letter"),
            }
        }
        fn get_position(&self) -> Position {
            match self {
                NumericButton::Zero => Position { x: 1, y: 3 },
                NumericButton::One => Position { x: 0, y: 2 },
                NumericButton::Two => Position { x: 1, y: 2 },
                NumericButton::Three => Position { x: 2, y: 2 },
                NumericButton::Four => Position { x: 0, y: 1 },
                NumericButton::Five => Position { x: 1, y: 1 },
                NumericButton::Six => Position { x: 2, y: 1 },
                NumericButton::Seven => Position { x: 0, y: 0 },
                NumericButton::Eight => Position { x: 1, y: 0 },
                NumericButton::Nine => Position { x: 2, y: 0 },
                NumericButton::A => Position { x: 2, y: 3 },
            }
        }
        // Get the minimum movement cost to move from one position to another on the numeric keypad
        // which is behind a number of directional keypads equal to the depth (or depth + 1 if
        // including the keypad at the end that I directly operate). This is done by converting the
        // difference in position to a sequence of directional button presses that must be made on
        // the immediately-preceeding directional keypad. All movement passes over the minimum
        // number of spaces, and always moves entirely vertically or entirely horizontally before
        // moving in the other direction due to how pressing the same directional button multiple
        // times in a row is much cheaper than any other options.
        fn get_mc_to(
            &self,
            next: &NumericButton,
            depth: u8,
            mc_cache: &mut FxHashMap<DirButMovementCostParameters, i64>,
        ) -> i64 {
            let self_position = self.get_position();
            let next_position = next.get_position();
            let position_diff = Position {
                x: next_position.x - self_position.x,
                y: next_position.y - self_position.y,
            };

            // If there is no difference in position, just press A again to tell the robot to
            // immediately press the same button as before.
            if position_diff.x == 0 && position_diff.y == 0 {
                return DirBut::A.get_mc_to(&DirBut::A, depth, mc_cache);
            }

            let horizontal_direction = if position_diff.x < 0 {
                DirBut::Left
            } else {
                DirBut::Right
            };
            let vertical_direction = if position_diff.y < 0 {
                DirBut::Up
            } else {
                DirBut::Down
            };

            // If the new position is vertically aligned, only make vertical moves to the new
            // position.
            if position_diff.x == 0 {
                return DirBut::A.get_mc_to(&vertical_direction, depth, mc_cache)
                    + (position_diff.y.abs() - 1)
                        * vertical_direction.get_mc_to(&vertical_direction, depth, mc_cache)
                    + vertical_direction.get_mc_to(&DirBut::A, depth, mc_cache);
            }
            // If the new position is horizontally aligned, only make horizontal moves to the new
            // position.
            if position_diff.y == 0 {
                return DirBut::A.get_mc_to(&horizontal_direction, depth, mc_cache)
                    + (position_diff.x.abs() - 1)
                        * horizontal_direction.get_mc_to(&horizontal_direction, depth, mc_cache)
                    + horizontal_direction.get_mc_to(&DirBut::A, depth, mc_cache);
            }
            // If the starting position is in the final row and the ending position is in the first
            // column, make all vertical moves before horizontal moves to avoid passing over the
            // gap.
            if self_position.y == 3 && next_position.x == 0 {
                return DirBut::A.get_mc_to(&vertical_direction, depth, mc_cache)
                    + (position_diff.y.abs() - 1)
                        * vertical_direction.get_mc_to(&vertical_direction, depth, mc_cache)
                    + vertical_direction.get_mc_to(&horizontal_direction, depth, mc_cache)
                    + (position_diff.x.abs() - 1)
                        * horizontal_direction.get_mc_to(&horizontal_direction, depth, mc_cache)
                    + horizontal_direction.get_mc_to(&DirBut::A, depth, mc_cache);
            }
            // If the starting position is in the first column and the ending position is in the
            // final row, make all horizontal moves before vertical moves to avoid passing over the
            // gap.
            if self_position.x == 0 && next_position.y == 3 {
                return DirBut::A.get_mc_to(&horizontal_direction, depth, mc_cache)
                    + (position_diff.y.abs() - 1)
                        * horizontal_direction.get_mc_to(&horizontal_direction, depth, mc_cache)
                    + horizontal_direction.get_mc_to(&vertical_direction, depth, mc_cache)
                    + (position_diff.x.abs() - 1)
                        * vertical_direction.get_mc_to(&vertical_direction, depth, mc_cache)
                    + vertical_direction.get_mc_to(&DirBut::A, depth, mc_cache);
            }
            // None of the previous restrictions have been met, which makes it unclear whether
            // moving horizontally first or moving vertically first is ideal, so check both cases
            // and return the one which gives the minimum movement cost.
            min(
                DirBut::A.get_mc_to(&vertical_direction, depth, mc_cache)
                    + (position_diff.y.abs() - 1)
                        * vertical_direction.get_mc_to(&vertical_direction, depth, mc_cache)
                    + vertical_direction.get_mc_to(&horizontal_direction, depth, mc_cache)
                    + (position_diff.x.abs() - 1)
                        * horizontal_direction.get_mc_to(&horizontal_direction, depth, mc_cache)
                    + horizontal_direction.get_mc_to(&DirBut::A, depth, mc_cache),
                DirBut::A.get_mc_to(&horizontal_direction, depth, mc_cache)
                    + (position_diff.y.abs() - 1)
                        * horizontal_direction.get_mc_to(&horizontal_direction, depth, mc_cache)
                    + horizontal_direction.get_mc_to(&vertical_direction, depth, mc_cache)
                    + (position_diff.x.abs() - 1)
                        * vertical_direction.get_mc_to(&vertical_direction, depth, mc_cache)
                    + vertical_direction.get_mc_to(&DirBut::A, depth, mc_cache),
            )
        }
    }

    #[derive(Clone, PartialEq, Eq, Hash)]
    enum DirBut {
        Up,
        Right,
        Down,
        Left,
        A,
    }
    impl DirBut {
        // Get the movement cost (how many times I need to physically push buttons on my directional
        // keypad at the end of the chain) so that the keypad at a number of levels down equal to
        // the depth can move from any key to any key and press it.
        //
        // Variable names here are aggressively shortened even to the point of becoming acronyms,
        // as this is still more readable than if any of the function calls have to be split over
        // multiple lines due to the 25 unique cases that have to be covered in the match statement.
        //
        // "DirBur" is short for "DirectionalButton"
        // "mc" is short for "movement_cost"
        fn get_mc_to(
            &self,
            next: &DirBut,
            depth: u8,
            mc_cache: &mut FxHashMap<DirButMovementCostParameters, i64>,
        ) -> i64 {
            // Depth 0 is considered to be the keypad that I am directly operating. The movement
            // cost is a count of how many times I need to press a button to move the robot at a
            // certain depth from one button to another. This means that by definition, the movement
            // cost of every button at depth 0 is 1, since I simply have to press the button once.
            if depth == 0 {
                return 1;
            }
            // If it is not at depth zero, next check if this combination of parameters has been
            // calculated before. If so, return the cached value from last time.
            if let Some(mc) = mc_cache.get(&DirButMovementCostParameters {
                start_button: self.clone(),
                end_button: next.clone(),
                depth,
            }) {
                return *mc;
            }

            // The depth is not 0 and the result was not cached, so calculate the movement cost
            // manually.
            //
            // This match statement is an exhaustive list of the cheapest way to instruct a robot
            // one level down to go from any button to any button. This function is called
            // recursively as many times as needed to reduce the problem to depth 0 where the result
            // is trivially 1. In some cases, there are two different paths of equal length from one
            // button to another, so both are checked and the one with the minimum movement cost is
            // returned.
            //
            // Arms are intentionally duplicated for clarity so all paths that begin from the same
            // button are kept together.
            #[allow(clippy::match_same_arms)]
            let mc = match (self, next) {
                (DirBut::A, DirBut::A) => DirBut::A.get_mc_to(&DirBut::A, depth - 1, mc_cache),
                (DirBut::A, DirBut::Up) => {
                    DirBut::A.get_mc_to(&DirBut::Left, depth - 1, mc_cache)
                        + DirBut::Left.get_mc_to(&DirBut::A, depth - 1, mc_cache)
                }
                (DirBut::A, DirBut::Right) => {
                    DirBut::A.get_mc_to(&DirBut::Down, depth - 1, mc_cache)
                        + DirBut::Down.get_mc_to(&DirBut::A, depth - 1, mc_cache)
                }
                (DirBut::A, DirBut::Down) => min(
                    DirBut::A.get_mc_to(&DirBut::Left, depth - 1, mc_cache)
                        + DirBut::Left.get_mc_to(&DirBut::Down, depth - 1, mc_cache)
                        + DirBut::Down.get_mc_to(&DirBut::A, depth - 1, mc_cache),
                    DirBut::A.get_mc_to(&DirBut::Down, depth - 1, mc_cache)
                        + DirBut::Down.get_mc_to(&DirBut::Left, depth - 1, mc_cache)
                        + DirBut::Left.get_mc_to(&DirBut::A, depth - 1, mc_cache),
                ),
                (DirBut::A, DirBut::Left) => min(
                    DirBut::A.get_mc_to(&DirBut::Left, depth - 1, mc_cache)
                        + DirBut::Left.get_mc_to(&DirBut::Down, depth - 1, mc_cache)
                        + DirBut::Down.get_mc_to(&DirBut::Left, depth - 1, mc_cache)
                        + DirBut::Left.get_mc_to(&DirBut::A, depth - 1, mc_cache),
                    DirBut::A.get_mc_to(&DirBut::Down, depth - 1, mc_cache)
                        + DirBut::Down.get_mc_to(&DirBut::Left, depth - 1, mc_cache)
                        + DirBut::Left.get_mc_to(&DirBut::Left, depth - 1, mc_cache)
                        + DirBut::Left.get_mc_to(&DirBut::A, depth - 1, mc_cache),
                ),

                (DirBut::Up, DirBut::A) => {
                    DirBut::A.get_mc_to(&DirBut::Right, depth - 1, mc_cache)
                        + DirBut::Right.get_mc_to(&DirBut::A, depth - 1, mc_cache)
                }
                (DirBut::Up, DirBut::Up) => DirBut::A.get_mc_to(&DirBut::A, depth - 1, mc_cache),
                (DirBut::Up, DirBut::Right) => min(
                    DirBut::A.get_mc_to(&DirBut::Right, depth - 1, mc_cache)
                        + DirBut::Right.get_mc_to(&DirBut::Down, depth - 1, mc_cache)
                        + DirBut::Down.get_mc_to(&DirBut::A, depth - 1, mc_cache),
                    DirBut::A.get_mc_to(&DirBut::Down, depth - 1, mc_cache)
                        + DirBut::Down.get_mc_to(&DirBut::Right, depth - 1, mc_cache)
                        + DirBut::Right.get_mc_to(&DirBut::A, depth - 1, mc_cache),
                ),
                (DirBut::Up, DirBut::Down) => {
                    DirBut::A.get_mc_to(&DirBut::Down, depth - 1, mc_cache)
                        + DirBut::Down.get_mc_to(&DirBut::A, depth - 1, mc_cache)
                }
                (DirBut::Up, DirBut::Left) => {
                    DirBut::A.get_mc_to(&DirBut::Down, depth - 1, mc_cache)
                        + DirBut::Down.get_mc_to(&DirBut::Left, depth - 1, mc_cache)
                        + DirBut::Left.get_mc_to(&DirBut::A, depth - 1, mc_cache)
                }

                (DirBut::Right, DirBut::A) => {
                    DirBut::A.get_mc_to(&DirBut::Up, depth - 1, mc_cache)
                        + DirBut::Up.get_mc_to(&DirBut::A, depth - 1, mc_cache)
                }
                (DirBut::Right, DirBut::Up) => min(
                    DirBut::A.get_mc_to(&DirBut::Left, depth - 1, mc_cache)
                        + DirBut::Left.get_mc_to(&DirBut::Up, depth - 1, mc_cache)
                        + DirBut::Up.get_mc_to(&DirBut::A, depth - 1, mc_cache),
                    DirBut::A.get_mc_to(&DirBut::Up, depth - 1, mc_cache)
                        + DirBut::Up.get_mc_to(&DirBut::Left, depth - 1, mc_cache)
                        + DirBut::Left.get_mc_to(&DirBut::A, depth - 1, mc_cache),
                ),
                (DirBut::Right, DirBut::Right) => {
                    DirBut::A.get_mc_to(&DirBut::A, depth - 1, mc_cache)
                }
                (DirBut::Right, DirBut::Down) => {
                    DirBut::A.get_mc_to(&DirBut::Left, depth - 1, mc_cache)
                        + DirBut::Left.get_mc_to(&DirBut::A, depth - 1, mc_cache)
                }
                (DirBut::Right, DirBut::Left) => {
                    DirBut::A.get_mc_to(&DirBut::Left, depth - 1, mc_cache)
                        + DirBut::Left.get_mc_to(&DirBut::Left, depth - 1, mc_cache)
                        + DirBut::Left.get_mc_to(&DirBut::A, depth - 1, mc_cache)
                }

                (DirBut::Down, DirBut::A) => min(
                    DirBut::A.get_mc_to(&DirBut::Right, depth - 1, mc_cache)
                        + DirBut::Right.get_mc_to(&DirBut::Up, depth - 1, mc_cache)
                        + DirBut::Up.get_mc_to(&DirBut::A, depth - 1, mc_cache),
                    DirBut::A.get_mc_to(&DirBut::Up, depth - 1, mc_cache)
                        + DirBut::Up.get_mc_to(&DirBut::Right, depth - 1, mc_cache)
                        + DirBut::Right.get_mc_to(&DirBut::A, depth - 1, mc_cache),
                ),
                (DirBut::Down, DirBut::Up) => {
                    DirBut::A.get_mc_to(&DirBut::Up, depth - 1, mc_cache)
                        + DirBut::Up.get_mc_to(&DirBut::A, depth - 1, mc_cache)
                }
                (DirBut::Down, DirBut::Right) => {
                    DirBut::A.get_mc_to(&DirBut::Right, depth - 1, mc_cache)
                        + DirBut::Right.get_mc_to(&DirBut::A, depth - 1, mc_cache)
                }
                (DirBut::Down, DirBut::Down) => {
                    DirBut::A.get_mc_to(&DirBut::A, depth - 1, mc_cache)
                }
                (DirBut::Down, DirBut::Left) => {
                    DirBut::A.get_mc_to(&DirBut::Left, depth - 1, mc_cache)
                        + DirBut::Left.get_mc_to(&DirBut::A, depth - 1, mc_cache)
                }

                (DirBut::Left, DirBut::A) => min(
                    DirBut::A.get_mc_to(&DirBut::Right, depth - 1, mc_cache)
                        + DirBut::Right.get_mc_to(&DirBut::Up, depth - 1, mc_cache)
                        + DirBut::Up.get_mc_to(&DirBut::Right, depth - 1, mc_cache)
                        + DirBut::Right.get_mc_to(&DirBut::A, depth - 1, mc_cache),
                    DirBut::A.get_mc_to(&DirBut::Right, depth - 1, mc_cache)
                        + DirBut::Right.get_mc_to(&DirBut::Right, depth - 1, mc_cache)
                        + DirBut::Right.get_mc_to(&DirBut::Up, depth - 1, mc_cache)
                        + DirBut::Up.get_mc_to(&DirBut::A, depth - 1, mc_cache),
                ),
                (DirBut::Left, DirBut::Up) => {
                    DirBut::A.get_mc_to(&DirBut::Right, depth - 1, mc_cache)
                        + DirBut::Right.get_mc_to(&DirBut::Up, depth - 1, mc_cache)
                        + DirBut::Up.get_mc_to(&DirBut::A, depth - 1, mc_cache)
                }
                (DirBut::Left, DirBut::Right) => {
                    DirBut::A.get_mc_to(&DirBut::Right, depth - 1, mc_cache)
                        + DirBut::Right.get_mc_to(&DirBut::Right, depth - 1, mc_cache)
                        + DirBut::Right.get_mc_to(&DirBut::A, depth - 1, mc_cache)
                }
                (DirBut::Left, DirBut::Down) => {
                    DirBut::A.get_mc_to(&DirBut::Right, depth - 1, mc_cache)
                        + DirBut::Right.get_mc_to(&DirBut::A, depth - 1, mc_cache)
                }
                (DirBut::Left, DirBut::Left) => {
                    DirBut::A.get_mc_to(&DirBut::A, depth - 1, mc_cache)
                }
            };

            // Insert the new value into the cache, then return it.
            mc_cache.insert(
                DirButMovementCostParameters {
                    start_button: self.clone(),
                    end_button: next.clone(),
                    depth,
                },
                mc,
            );
            mc
        }
    }

    let mut directional_button_movement_cost_cache = FxHashMap::default();
    let mut complexity_sum = 0;

    // For each line, determine its complexity.
    for line in input.lines() {
        // Get the desired sequence of numeric button presses.
        let output_numeric_buttons = line.chars().map(NumericButton::new).collect::<Vec<_>>();
        // Get the numeric part of the line.
        let numeric_part = line
            .replace('A', "")
            .parse::<i64>()
            .expect("Line should only contain digits after removing \"A\"s");

        // Calculate the movement cost to go to the next button. The starting button before the
        // first output button is always A. Sum the movement costs together to get the minimum total
        // movement cost, which is equivalent to the length of the shortest sequence of buttons that
        // need to be pressed on the last keypad.
        let mut minimum_total_movement_cost = 0;
        let mut last_numeric_button = NumericButton::A;
        for next_numeric_button in output_numeric_buttons {
            minimum_total_movement_cost += last_numeric_button.get_mc_to(
                &next_numeric_button,
                depth,
                &mut directional_button_movement_cost_cache,
            );

            last_numeric_button = next_numeric_button;
        }

        // Get the complexity by multiplying the numeric part with the shortest sequence length.
        complexity_sum += minimum_total_movement_cost * numeric_part;
    }

    Solution::I64(complexity_sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("029A"), Solution::U16(68 * 29));
    }
    #[test]
    fn example1_2() {
        assert_eq!(solve_1("980A"), Solution::U16(60 * 980));
    }
    #[test]
    fn example1_3() {
        assert_eq!(solve_1("179A"), Solution::U16(68 * 179));
    }
    #[test]
    fn example1_4() {
        assert_eq!(solve_1("456A"), Solution::U16(64 * 456));
    }
    #[test]
    fn example1_5() {
        assert_eq!(solve_1("379A"), Solution::U16(64 * 379));
    }
}
