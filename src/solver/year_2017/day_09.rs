use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2017,
    day: 9,
    title: "Stream Processing",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let mut score: u32 = 0;
    let mut level: u32 = 0;
    let mut is_inside_garbage = false;

    let mut iter = input.chars();

    while let Some(char) = iter.next() {
        if is_inside_garbage {
            match char {
                '>' => {
                    is_inside_garbage = false;
                }
                '!' => {
                    // Call next() on the iterator and ignore the value to skip the next character.
                    iter.next();
                }
                _ => {}
            }
        } else {
            match char {
                '{' => {
                    level += 1;
                    score += level;
                }
                '}' => {
                    level -= 1;
                }
                '<' => {
                    is_inside_garbage = true;
                }
                _ => {}
            }
        }
    }

    Solution::U32(score)
}

fn solve_2(input: &str) -> Solution {
    let mut garbage_count: u32 = 0;
    let mut is_inside_garbage = false;

    let mut iter = input.chars();

    while let Some(char) = iter.next() {
        if is_inside_garbage {
            match char {
                '>' => {
                    is_inside_garbage = false;
                }
                '!' => {
                    // Call next() on the iterator and ignore the value to skip the next character.
                    iter.next();
                }
                _ => {
                    garbage_count += 1;
                }
            }
        } else if char == '<' {
            is_inside_garbage = true;
        }
    }

    Solution::U32(garbage_count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("{}"), Solution::U8(1));
    }
    #[test]
    fn example1_2() {
        assert_eq!(solve_1("{{{}}}"), Solution::U8(6));
    }
    #[test]
    fn example1_3() {
        assert_eq!(solve_1("{{},{}}"), Solution::U8(5));
    }
    #[test]
    fn example1_4() {
        assert_eq!(solve_1("{{{},{},{{}}}}"), Solution::U8(16));
    }
    #[test]
    fn example1_5() {
        assert_eq!(solve_1("{<a>,<a>,<a>,<a>}"), Solution::U8(1));
    }
    #[test]
    fn example1_6() {
        assert_eq!(solve_1("{{<ab>},{<ab>},{<ab>},{<ab>}}"), Solution::U8(9));
    }
    #[test]
    fn example1_7() {
        assert_eq!(solve_1("{{<!!>},{<!!>},{<!!>},{<!!>}}"), Solution::U8(9));
    }
    #[test]
    fn example1_8() {
        assert_eq!(solve_1("{{<a!>},{<a!>},{<a!>},{<ab>}}"), Solution::U8(3));
    }

    #[test]
    fn example2_1() {
        assert_eq!(solve_2("<>"), Solution::U8(0));
    }
    #[test]
    fn example2_2() {
        assert_eq!(solve_2("<random characters>"), Solution::U8(17));
    }
    #[test]
    fn example2_3() {
        assert_eq!(solve_2("<<<<>"), Solution::U8(3));
    }
    #[test]
    fn example2_4() {
        assert_eq!(solve_2("<{!>}>"), Solution::U8(2));
    }
    #[test]
    fn example2_5() {
        assert_eq!(solve_2("<!!>"), Solution::U8(0));
    }
    #[test]
    fn example2_6() {
        assert_eq!(solve_2("<!!!>>"), Solution::U8(0));
    }
    #[test]
    fn example2_7() {
        assert_eq!(solve_2("<{o\"i!a,<{i<a>"), Solution::U8(10));
    }
}
