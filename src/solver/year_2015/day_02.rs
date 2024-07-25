use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 2,
    title: "I Was Told There Would Be No Math",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let mut total_wrapping_paper = 0;
    for line in input.lines() {
        let dimensions = line
            .split('x')
            .map(|num| {
                num.parse::<u32>()
                    .expect("All dimensions should be numbers")
            })
            .collect::<Vec<u32>>();
        let side_areas = [
            dimensions[0] * dimensions[1],
            dimensions[0] * dimensions[2],
            dimensions[1] * dimensions[2],
        ];
        total_wrapping_paper += 2 * side_areas.iter().sum::<u32>()
            + side_areas.iter().min().expect("Should have sides");
    }
    Solution::U32(total_wrapping_paper)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("2x3x4"), Solution::U8(58));
    }
    #[test]
    fn example1_2() {
        assert_eq!(solve_1("1x1x10"), Solution::U8(43));
    }
}
