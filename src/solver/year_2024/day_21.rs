use crate::solver::{Solution, Solver};

// Temporarily allow dead code, as this solver is disabled for now due to being incomplete.
#[allow(dead_code)]
pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 21,
    title: "Keypad Conundrum",
    part_solvers: &[solve_1],
};

// This solver currently produces the correct answer for the examples in the puzzle description, but
// the incorrect answer for my puzzle input. I am unable to discern any kind of fundamental
// difference between the examples and my puzzle input, so I am currently clueless about how any
// kind of error could be introduced only in my puzzle input. As such, this puzzle has me stumped,
// so I'm moving on for now.
fn solve_1(input: &str) -> Solution {
    struct Position {
        x: i32,
        y: i32,
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
        // Find the difference between this position and the next one, determine a sequence of
        // button presses required to go to the next position, and append them to the end of the
        // current button sequence. Ensure that the minimum distance is traveled and that
        // horizontal/vertical button presses don't alternate except at a single corner in the path
        // so that the minimum button presses are added.
        fn add_directional_buttons_to_go_to(
            &self,
            next: &NumericButton,
            directional_button_sequence: &mut Vec<DirectionalButton>,
        ) {
            let self_position = self.get_position();
            let next_position = next.get_position();
            let position_diff = Position {
                x: next_position.x - self_position.x,
                y: next_position.y - self_position.y,
            };

            let horizontal_direction = if position_diff.x < 0 {
                DirectionalButton::Left
            } else {
                DirectionalButton::Right
            };
            let vertical_direction = if position_diff.y < 0 {
                DirectionalButton::Up
            } else {
                DirectionalButton::Down
            };

            // Move horizontally first, unless the arm is currently on the last row, in which case
            // move vertically first to avoid ever aiming at the blank space.
            if self_position.y == 3 {
                for _ in 0..position_diff.y.abs() {
                    directional_button_sequence.push(vertical_direction.clone());
                }
                for _ in 0..position_diff.x.abs() {
                    directional_button_sequence.push(horizontal_direction.clone());
                }
            } else {
                for _ in 0..position_diff.x.abs() {
                    directional_button_sequence.push(horizontal_direction.clone());
                }
                for _ in 0..position_diff.y.abs() {
                    directional_button_sequence.push(vertical_direction.clone());
                }
            }
            directional_button_sequence.push(DirectionalButton::A);
        }
    }

    #[derive(Clone)]
    enum DirectionalButton {
        Up,
        Right,
        Down,
        Left,
        A,
    }
    impl DirectionalButton {
        fn get_position(&self) -> Position {
            match self {
                DirectionalButton::Up => Position { x: 1, y: 0 },
                DirectionalButton::Right => Position { x: 2, y: 1 },
                DirectionalButton::Down => Position { x: 1, y: 1 },
                DirectionalButton::Left => Position { x: 0, y: 1 },
                DirectionalButton::A => Position { x: 2, y: 0 },
            }
        }
        // This is almost the same as the corresponding method for numeric buttons, except the type
        // of the buttons and the position of the blank space are different.
        fn add_directional_buttons_to_go_to(
            &self,
            next: &DirectionalButton,
            directional_button_sequence: &mut Vec<DirectionalButton>,
        ) {
            let self_position = self.get_position();
            let next_position = next.get_position();
            let position_diff = Position {
                x: next_position.x - self_position.x,
                y: next_position.y - self_position.y,
            };

            let horizontal_direction = if position_diff.x < 0 {
                DirectionalButton::Left
            } else {
                DirectionalButton::Right
            };
            let vertical_direction = if position_diff.y < 0 {
                DirectionalButton::Up
            } else {
                DirectionalButton::Down
            };

            // Move horizontally first, unless the arm is currently on the first row, in which case
            // move vertically first to avoid ever aiming at the blank space.
            if self_position.y == 0 {
                for _ in 0..position_diff.y.abs() {
                    directional_button_sequence.push(vertical_direction.clone());
                }
                for _ in 0..position_diff.x.abs() {
                    directional_button_sequence.push(horizontal_direction.clone());
                }
            } else {
                for _ in 0..position_diff.x.abs() {
                    directional_button_sequence.push(horizontal_direction.clone());
                }
                for _ in 0..position_diff.y.abs() {
                    directional_button_sequence.push(vertical_direction.clone());
                }
            }
            directional_button_sequence.push(DirectionalButton::A);
        }
    }

    let mut complexity_sum = 0;
    // For each line, determine its complexity.
    for line in input.lines() {
        // Get the desired sequence of numeric button presses.
        let target_numeric_buttons = line.chars().map(NumericButton::new).collect::<Vec<_>>();
        // Get the numeric part of the line.
        let numeric_part = line
            .replace('A', "")
            .parse::<usize>()
            .expect("Line should only contain digits after removing \"A\"s");

        // Get a sequence of button presses on the first directional keypad that inputs the numeric
        // sequence on the numeric keypad.
        let mut last_numeric_button = NumericButton::A;
        let mut first_target_directional_buttons = Vec::new();
        for target_numeric_button in target_numeric_buttons {
            last_numeric_button.add_directional_buttons_to_go_to(
                &target_numeric_button,
                &mut first_target_directional_buttons,
            );

            last_numeric_button = target_numeric_button;
        }

        // Get a sequence of button presses on the second directional keypad that inputs the
        // directional sequence on the first directional keypad.
        let mut last_directional_button = DirectionalButton::A;
        let mut second_target_directional_buttons = Vec::new();
        for target_directional_button in first_target_directional_buttons {
            last_directional_button.add_directional_buttons_to_go_to(
                &target_directional_button,
                &mut second_target_directional_buttons,
            );

            last_directional_button = target_directional_button;
        }

        // Get a sequence of button presses on the third directional keypad that inputs the
        // directional sequence on the second directional keypad.
        let mut last_directional_button = DirectionalButton::A;
        let mut third_target_directional_buttons = Vec::new();
        for target_directional_button in second_target_directional_buttons {
            last_directional_button.add_directional_buttons_to_go_to(
                &target_directional_button,
                &mut third_target_directional_buttons,
            );

            last_directional_button = target_directional_button;
        }

        // Get the length of the third directional button sequence
        let shortest_sequence_length = third_target_directional_buttons.len();
        // Get the complexity by multiplying the numeric part with the shortest sequence length.
        complexity_sum += shortest_sequence_length * numeric_part;
    }

    Solution::USize(complexity_sum)
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
