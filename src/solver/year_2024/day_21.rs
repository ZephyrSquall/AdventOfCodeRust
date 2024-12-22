use crate::solver::{Solution, Solver};
use std::cmp::min;

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 21,
    title: "Keypad Conundrum",
    part_solvers: &[solve_1],
};

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
        // Find a sequence of directional button presses that will cause the robot in front of the
        // numeric keypad to press the next numeric button in the minimum number of moves, and
        // append this to each possible input sequence. To minimize the button presses, the robot in
        // front of the numeric keypad always moves vertically and moves horizontally at most once
        // between button presses. This still allows for two possibilities of how to reach the
        // button when the robot in front of the numeric keypad needs to move diagonally (unless one
        // option would pass over the gap), so if this is the case, each possible input sequence
        // will be duplicated, with one of each duplicated input sequence extended with the first
        // possibility and the other duplicated input sequence extended with the other possibility.
        fn extend_input_sequences_to_go_to(
            &self,
            next: &NumericButton,
            input_sequences: &mut Vec<Vec<DirectionalButton>>,
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

            let mut input_extensions = Vec::with_capacity(2);
            // If the next position is exactly aligned horizontally or vertically with the current
            // position, simply add the straight line steps there to every possible input sequence.
            if position_diff.x == 0 {
                let mut input_extension = Vec::with_capacity(4);
                for _ in 0..position_diff.y.abs() {
                    input_extension.push(vertical_direction.clone());
                }
                input_extension.push(DirectionalButton::A);
                input_extensions.push(input_extension);
            } else if position_diff.y == 0 {
                let mut input_extension = Vec::with_capacity(3);
                for _ in 0..position_diff.x.abs() {
                    input_extension.push(horizontal_direction.clone());
                }
                input_extension.push(DirectionalButton::A);
                input_extensions.push(input_extension);
            } else {
                // Otherwise, add both the possibility of going horizontally then vertically, or
                // vertically then horizontally, unless this would cause the path to go over the
                // gap.

                // Add the sequence when moving horizontally then vertically, unless starting in the
                // last row and ending in the first column.
                if !(self_position.y == 3 && next_position.x == 0) {
                    let mut input_extension = Vec::with_capacity(6);
                    for _ in 0..position_diff.x.abs() {
                        input_extension.push(horizontal_direction.clone());
                    }
                    for _ in 0..position_diff.y.abs() {
                        input_extension.push(vertical_direction.clone());
                    }
                    input_extension.push(DirectionalButton::A);
                    input_extensions.push(input_extension);
                }

                // Add the sequence when moving vertically then horizontally, unless starting in the
                // first column and ending in the last row.
                if !(self_position.x == 0 && next_position.y == 3) {
                    let mut input_extension = Vec::with_capacity(6);
                    for _ in 0..position_diff.y.abs() {
                        input_extension.push(vertical_direction.clone());
                    }
                    for _ in 0..position_diff.x.abs() {
                        input_extension.push(horizontal_direction.clone());
                    }

                    input_extension.push(DirectionalButton::A);
                    input_extensions.push(input_extension);
                }
            }

            // There can either be one or two input extensions. If there is one, simply append it to
            // each input sequence. If there are two, clone the input sequences, append the first
            // input extension to each original input sequence, append the second input extension to
            // each cloned input sequence, then combine the original and cloned input sequences.
            if input_extensions.len() == 2 {
                let mut input_sequences_clone = input_sequences.clone();

                for input_sequence in &mut *input_sequences {
                    input_sequence.extend_from_slice(&input_extensions[0]);
                }

                for input_sequence_clone in &mut input_sequences_clone {
                    input_sequence_clone.extend_from_slice(&input_extensions[1]);
                    input_sequences.push(input_sequence_clone.clone());
                }
            } else {
                for input_sequence in input_sequences {
                    input_sequence.extend_from_slice(&input_extensions[0]);
                }
            }
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
        fn extend_input_sequences_to_go_to(
            &self,
            next: &DirectionalButton,
            input_sequences: &mut Vec<Vec<DirectionalButton>>,
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

            let mut input_extensions = Vec::with_capacity(2);
            if position_diff.x == 0 {
                let mut input_extension = Vec::with_capacity(2);
                for _ in 0..position_diff.y.abs() {
                    input_extension.push(vertical_direction.clone());
                }
                input_extension.push(DirectionalButton::A);
                input_extensions.push(input_extension);
            } else if position_diff.y == 0 {
                let mut input_extension = Vec::with_capacity(3);
                for _ in 0..position_diff.x.abs() {
                    input_extension.push(horizontal_direction.clone());
                }
                input_extension.push(DirectionalButton::A);
                input_extensions.push(input_extension);
            } else {
                if !(self_position.y == 0 && next_position.x == 0) {
                    let mut input_extension = Vec::with_capacity(4);
                    for _ in 0..position_diff.x.abs() {
                        input_extension.push(horizontal_direction.clone());
                    }
                    for _ in 0..position_diff.y.abs() {
                        input_extension.push(vertical_direction.clone());
                    }
                    input_extension.push(DirectionalButton::A);
                    input_extensions.push(input_extension);
                }
                if !(self_position.x == 0 && next_position.y == 0) {
                    let mut input_extension = Vec::with_capacity(4);
                    for _ in 0..position_diff.y.abs() {
                        input_extension.push(vertical_direction.clone());
                    }
                    for _ in 0..position_diff.x.abs() {
                        input_extension.push(horizontal_direction.clone());
                    }

                    input_extension.push(DirectionalButton::A);
                    input_extensions.push(input_extension);
                }
            }

            if input_extensions.len() == 2 {
                let mut input_sequences_clone = input_sequences.clone();

                for input_sequence in &mut *input_sequences {
                    input_sequence.extend_from_slice(&input_extensions[0]);
                }

                for input_sequence_clone in &mut input_sequences_clone {
                    input_sequence_clone.extend_from_slice(&input_extensions[1]);
                    input_sequences.push(input_sequence_clone.clone());
                }
            } else {
                for input_sequence in input_sequences {
                    input_sequence.extend_from_slice(&input_extensions[0]);
                }
            }
        }
    }

    let mut complexity_sum = 0;
    // For each line, determine its complexity.
    for line in input.lines() {
        // Get the desired sequence of numeric button presses.
        let output_numeric_buttons = line.chars().map(NumericButton::new).collect::<Vec<_>>();
        // Get the numeric part of the line.
        let numeric_part = line
            .replace('A', "")
            .parse::<usize>()
            .expect("Line should only contain digits after removing \"A\"s");

        // Get each possible sequence of button presses on the first directional keypad that inputs
        // the numeric sequence on the numeric keypad.
        let mut last_numeric_button = NumericButton::A;
        let mut numeric_input_sequences = vec![Vec::new()];
        for target_numeric_button in output_numeric_buttons {
            last_numeric_button.extend_input_sequences_to_go_to(
                &target_numeric_button,
                &mut numeric_input_sequences,
            );

            last_numeric_button = target_numeric_button;
        }

        // Get each possible sequence of button presses on the second directional keypad that inputs
        // one of the directional sequences on the first directional keypad.
        let mut first_directional_input_sequences = Vec::new();
        for numeric_input_sequence in numeric_input_sequences {
            let mut last_directional_button = DirectionalButton::A;
            let mut directional_input_sequences = vec![Vec::new()];

            for target_directional_button in numeric_input_sequence {
                last_directional_button.extend_input_sequences_to_go_to(
                    &target_directional_button,
                    &mut directional_input_sequences,
                );

                last_directional_button = target_directional_button;
            }

            for directional_input_sequence in directional_input_sequences {
                first_directional_input_sequences.push(directional_input_sequence);
            }
        }

        // Get each possible sequence of button presses on the third directional keypad that inputs
        // one of the directional sequences on the second directional keypad.
        let mut second_directional_input_sequences = Vec::new();
        for first_directional_input_sequence in first_directional_input_sequences {
            let mut last_directional_button = DirectionalButton::A;
            let mut directional_input_sequences = vec![Vec::new()];

            for target_directional_button in first_directional_input_sequence {
                last_directional_button.extend_input_sequences_to_go_to(
                    &target_directional_button,
                    &mut directional_input_sequences,
                );

                last_directional_button = target_directional_button;
            }

            for directional_input_sequence in directional_input_sequences {
                second_directional_input_sequences.push(directional_input_sequence);
            }
        }

        // Get the shortest length of the second directional input sequence (the input that is typed
        // in at the third directional keypad)
        let shortest_sequence_length = second_directional_input_sequences
            .iter()
            .fold(usize::MAX, |acc, input_sequence| {
                min(acc, input_sequence.len())
            });
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
