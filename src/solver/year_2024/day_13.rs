use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 13,
    title: "Claw Contraption",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    // This refers to a two-dimensional mathematical vector, not Rust's vector type (which is
    // denoted Vec).
    #[derive(Clone, PartialEq)]
    struct Vector {
        x: u32,
        y: u32,
    }

    struct ClawMachine {
        button_a: Vector,
        button_b: Vector,
        prize: Vector,
    }
    impl ClawMachine {
        // Button A's and Button B's similar names comes from the puzzle description. There is no
        // apparent way to differentiate their names further without being overly wordy or
        // inaccurate.
        #[allow(clippy::similar_names)]
        fn new(string: &str) -> ClawMachine {
            let mut line_iter = string.lines();

            let button_a_string = line_iter
                .next()
                .expect("Claw machine string should have first line");
            let button_a_string = button_a_string.trim_start_matches("Button A: X+");
            let mut button_a_iter = button_a_string.split(", Y+");
            let button_a_x = button_a_iter
                .next()
                .expect("Button A should have a first value after trimming")
                .parse()
                .expect("Button A's first value should be a number");
            let button_a_y = button_a_iter
                .next()
                .expect("Button A should have a second value after trimming")
                .parse()
                .expect("Button A's second value should be a number");
            let button_a = Vector {
                x: button_a_x,
                y: button_a_y,
            };

            let button_b_string = line_iter
                .next()
                .expect("Claw machine string should have second line");
            let button_b_string = button_b_string.trim_start_matches("Button B: X+");
            let mut button_b_iter = button_b_string.split(", Y+");
            let button_b_x = button_b_iter
                .next()
                .expect("Button B should have a first value after trimming")
                .parse()
                .expect("Button B's first value should be a number");
            let button_b_y = button_b_iter
                .next()
                .expect("Button B should have a second value after trimming")
                .parse()
                .expect("Button B's second value should be a number");
            let button_b = Vector {
                x: button_b_x,
                y: button_b_y,
            };

            let prize_string = line_iter
                .next()
                .expect("Claw machine string should have third line");
            let prize_string = prize_string.trim_start_matches("Prize: X=");
            let mut prize_iter = prize_string.split(", Y=");
            let prize_x = prize_iter
                .next()
                .expect("Prize should have a first value after trimming")
                .parse()
                .expect("Prize's first value should be a number");
            let prize_y = prize_iter
                .next()
                .expect("Prize should have a second value after trimming")
                .parse()
                .expect("Prize's second value should be a number");
            let prize = Vector {
                x: prize_x,
                y: prize_y,
            };

            ClawMachine {
                button_a,
                button_b,
                prize,
            }
        }

        // Gets the number of tokens needed to reach the prize position using the available button
        // presses. If the prize position is unreachable, returns 0 instead to indicate that
        // spending no tokens on this claw machine is the best option.
        //
        // This is essentially a problem of finding out how to add two vectors to make a third
        // vector. As vector addition is commutative and we are only concerned with the total number
        // of times each button is pressed (i.e. the order the buttons are pressed is irrelevant),
        // there is only one possible number of Button A and Button B presses that will reach the
        // Prize position. Hence the puzzle description asking for the "minimum" number of button
        // presses is misleading - there is only one possible amount of button presses that works
        // (or none if the prize position is unreachable).
        #[allow(clippy::similar_names)]
        fn tokens_to_win(&self) -> usize {
            // The puzzle description specifies that each button will not need to be pressed more
            // than 100 times to win. So create a list of every position that is reached in the
            // process of pressing Button A 100 times. Then, start from the position of the prize,
            // and work backwards for each B button press, and check if it ever matches an A button
            // position within 100 presses.
            let mut button_a_positions = Vec::with_capacity(101);
            let mut button_a_position = Vector { x: 0, y: 0 };
            // Add <0,0>, the result of pushing the A button 0 times, to the list.
            button_a_positions.push(button_a_position.clone());
            for _ in 0..100 {
                button_a_position.x += self.button_a.x;
                button_a_position.y += self.button_a.y;
                button_a_positions.push(button_a_position.clone());
            }

            let mut button_b_position = self.prize.clone();
            for button_b_pushes in 0..=100 {
                if let Some(button_a_pushes) = button_a_positions
                    .iter()
                    .position(|button_a_position| *button_a_position == button_b_position)
                {
                    return 3 * button_a_pushes + button_b_pushes;
                }

                // Subtract Button B's vector from its current position. If either index becomes
                // negative, it has gone "behind" the origin and can no longer possibly meet up with
                // any Button A position, so return 0.
                if let Some((button_b_position_x, button_b_position_y)) = button_b_position
                    .x
                    .checked_sub(self.button_b.x)
                    .zip(button_b_position.y.checked_sub(self.button_b.y))
                {
                    button_b_position.x = button_b_position_x;
                    button_b_position.y = button_b_position_y;
                } else {
                    return 0;
                }
            }

            0
        }
    }

    let mut tokens_to_win_all = 0;
    // The input may have newlines represented by "\r\n" or "\n". For consecutive newlines to be
    // detected properly, they must be standardized to only be represented by "\n", so remove all
    // '\r' characters from the input.
    let input = input.replace('\r', "");

    for claw_machine in input.split("\n\n") {
        let claw_machine = ClawMachine::new(claw_machine);
        tokens_to_win_all += claw_machine.tokens_to_win();
    }
    Solution::USize(tokens_to_win_all)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
            ),
            Solution::U16(480)
        );
    }
}
