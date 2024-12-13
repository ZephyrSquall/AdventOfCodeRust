use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 13,
    title: "Claw Contraption",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    solve(input, false)
}

fn solve_2(input: &str) -> Solution {
    solve(input, true)
}

fn solve(input: &str, has_fixed_unit_conversion_error: bool) -> Solution {
    // This refers to a two-dimensional mathematical vector, not Rust's vector type (which is
    // denoted Vec).
    #[derive(Clone, PartialEq)]
    struct Vector {
        x: i64,
        y: i64,
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
        fn new(string: &str, has_fixed_unit_conversion_error: bool) -> ClawMachine {
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
            let prize = if has_fixed_unit_conversion_error {
                Vector {
                    x: prize_x + 10_000_000_000_000,
                    y: prize_y + 10_000_000_000_000,
                }
            } else {
                Vector {
                    x: prize_x,
                    y: prize_y,
                }
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
        fn tokens_to_win(&self) -> i64 {
            // The following formula required a mathematical derivation using linear algebra that is
            // too complex to reproduce in code comments. This derivation was made by considering
            // the button vectors to be an alternate basis for a two-dimensional vector space, and
            // then computing the change-of-basis matrix from the basis of unit vectors <0, 1> and
            // <1, 0> to the basis of the button vectors.
            //
            // With this change-of-basis matrix, the prize vector is transformed from the basis of
            // unit vectors to the basis of button vectors. The coefficients after transformation
            // indicate how many times each button has to be pressed to reach the prize position. If
            // the coefficients aren't integers, then it is not possible to reach the prize position
            // with the given button vectors.
            //
            // The change of basis vector from unit vectors to button vectors, (with the button
            // basis vectors A and B and prize vector P, with "_x" denoting x_coefficient and "_y"
            // denoting y coefficient in the basis of unit vectors) was calculated to be:
            // (1 / (A_x * B_y - A_y * B_x)) * [ B_y * P_x - B_x * P_y ]
            //                                 [ A_x * P_y - A_y * P_x ]
            //
            // Using this matrix, first modulo operations are used to check that both of the new
            // coefficients in the button basis are integers, and if so, division is then used to
            // calculate the actual coefficients and therefore the number of button pushes, from
            // which the token cost is calculated and returned.
            if (self.button_b.y * self.prize.x - self.button_b.x * self.prize.y)
                % (self.button_a.x * self.button_b.y - self.button_a.y * self.button_b.x)
                == 0
                && (self.button_a.x * self.prize.y - self.button_a.y * self.prize.x)
                    % (self.button_a.x * self.button_b.y - self.button_a.y * self.button_b.x)
                    == 0
            {
                // The token cost is three times the button A pushes, plus the button B pushes.
                return 3
                    * ((self.button_b.y * self.prize.x - self.button_b.x * self.prize.y)
                        / (self.button_a.x * self.button_b.y - self.button_a.y * self.button_b.x))
                    + ((self.button_a.x * self.prize.y - self.button_a.y * self.prize.x)
                        / (self.button_a.x * self.button_b.y - self.button_a.y * self.button_b.x));
            }

            // Return 0 if the coefficients in the button basis are not integers, as this means the
            // prize is unwinnable so the best option is to spend no tokens.
            0
        }
    }

    let mut tokens_to_win_all = 0;
    // The input may have newlines represented by "\r\n" or "\n". For consecutive newlines to be
    // detected properly, they must be standardized to only be represented by "\n", so remove all
    // '\r' characters from the input.
    let input = input.replace('\r', "");

    for claw_machine in input.split("\n\n") {
        let claw_machine = ClawMachine::new(claw_machine, has_fixed_unit_conversion_error);
        tokens_to_win_all += claw_machine.tokens_to_win();
    }
    Solution::I64(tokens_to_win_all)
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
