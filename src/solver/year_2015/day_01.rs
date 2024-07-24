use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 1,
    title: "Not Quite Lisp",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let mut floor = 0;
    for bracket in input.chars() {
        if bracket == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
    }
    Solution::I32(floor)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("(())"), Solution::U8(0));
    }
    #[test]
    fn example1_2() {
        assert_eq!(solve_1("()()"), Solution::U8(0));
    }
    #[test]
    fn example1_3() {
        assert_eq!(solve_1("((("), Solution::U8(3));
    }
    #[test]
    fn example1_4() {
        assert_eq!(solve_1("(()(()("), Solution::U8(3));
    }
    #[test]
    fn example1_5() {
        assert_eq!(solve_1("))((((("), Solution::U8(3));
    }
    #[test]
    fn example1_6() {
        assert_eq!(solve_1("())"), Solution::I8(-1));
    }
    #[test]
    fn example1_7() {
        assert_eq!(solve_1("))("), Solution::I8(-1));
    }
    #[test]
    fn example1_8() {
        assert_eq!(solve_1(")))"), Solution::I8(-3));
    }
    #[test]
    fn example1_9() {
        assert_eq!(solve_1(")())())"), Solution::I8(-3));
    }
}
