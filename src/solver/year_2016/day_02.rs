use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2016,
    day: 2,
    title: "Bathroom Security",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    const KEYPAD: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
    const X_LEN: usize = KEYPAD[0].len();
    const Y_LEN: usize = KEYPAD.len();

    let mut code = Vec::new();
    // KEYPAD[1][1] is the '5', the starting position for the first line.
    let mut x: usize = 1;
    let mut y: usize = 1;
    for line in input.lines() {
        for direction in line.chars() {
            match direction {
                'U' => {
                    // Saturating subtraction stops at 0, preventing y from going out of the bounds
                    // of the keypad.
                    y = y.saturating_sub(1);
                }
                'R' => {
                    if x + 1 < X_LEN {
                        x += 1;
                    }
                }
                'D' => {
                    if y + 1 < Y_LEN {
                        y += 1;
                    }
                }
                'L' => {
                    x = x.saturating_sub(1);
                }
                _ => panic!("Unsupported direction"),
            }
        }
        code.push(KEYPAD[y][x]);
    }

    Solution::String(code.into_iter().collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
ULL
RRDDD
LURDL
UUUUD"
            ),
            Solution::U16(1985)
        );
    }
}
