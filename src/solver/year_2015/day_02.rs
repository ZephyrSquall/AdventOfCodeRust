use crate::solver::{Solution, AdventOfCode};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2015,
    day: 2,
    title: "I Was Told There Would Be No Math",
    part_solvers: &[solve_1, solve_2],
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

fn solve_2(input: &str) -> Solution {
    let mut total_wrapping_paper = 0;
    for line in input.lines() {
        let dimensions = line
            .split('x')
            .map(|num| {
                num.parse::<u32>()
                    .expect("All dimensions should be numbers")
            })
            .collect::<Vec<u32>>();
        // To prevent needless doubling, half of the perimeter is calculated here and the smallest
        // value is later doubled when adding to total_wrapping_paper.
        let perimeters = [
            dimensions[0] + dimensions[1],
            dimensions[0] + dimensions[2],
            dimensions[1] + dimensions[2],
        ];
        total_wrapping_paper += 2 * perimeters.iter().min().expect("Should have sides")
            + dimensions.iter().product::<u32>();
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

    #[test]
    fn example2_1() {
        assert_eq!(solve_2("2x3x4"), Solution::U8(34));
    }
    #[test]
    fn example2_2() {
        assert_eq!(solve_2("1x1x10"), Solution::U8(14));
    }
}
