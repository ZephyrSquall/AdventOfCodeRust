use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2016,
    day: 2,
    title: "Bathroom Security",
    part_solvers: &[solve_1, solve_2],
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

fn solve_2(input: &str) -> Solution {
    // None is used to represent space that can't be moved into. The whole keypad is surrounded by
    // edges of None so no other bound checking is required.
    const KEYPAD: [[Option<char>; 7]; 7] = [
        [None, None, None, None, None, None, None],
        [None, None, None, Some('1'), None, None, None],
        [None, None, Some('2'), Some('3'), Some('4'), None, None],
        [
            None,
            Some('5'),
            Some('6'),
            Some('7'),
            Some('8'),
            Some('9'),
            None,
        ],
        [None, None, Some('A'), Some('B'), Some('C'), None, None],
        [None, None, None, Some('D'), None, None, None],
        [None, None, None, None, None, None, None],
    ];

    let mut code = Vec::new();
    // KEYPAD[3][1] is the '5', the starting position for the first line.
    let mut x: usize = 1;
    let mut y: usize = 3;
    for line in input.lines() {
        for direction in line.chars() {
            let x_next;
            let y_next;
            match direction {
                'U' => {
                    x_next = x;
                    y_next = y - 1;
                }
                'R' => {
                    x_next = x + 1;
                    y_next = y;
                }
                'D' => {
                    x_next = x;
                    y_next = y + 1;
                }
                'L' => {
                    x_next = x - 1;
                    y_next = y;
                }
                _ => panic!("Unsupported direction"),
            }

            if KEYPAD[y_next][x_next].is_some() {
                x = x_next;
                y = y_next;
            }
        }
        code.push(KEYPAD[y][x].expect("The keypad position should never move onto a None"));
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

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
ULL
RRDDD
LURDL
UUUUD"
            ),
            Solution::String("5DB3".to_string())
        );
    }
}
