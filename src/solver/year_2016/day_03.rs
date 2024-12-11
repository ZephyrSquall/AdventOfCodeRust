use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2016,
    day: 3,
    title: "Squares With Three Sides",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let mut possible_triangles = 0;

    for line in input.lines() {
        let mut side_lengths = line
            .split_whitespace()
            .map(|str| {
                str.parse::<u32>()
                    .expect("Each line should have only numbers")
            })
            .collect::<Vec<_>>();
        side_lengths.sort_unstable();

        // After sorting, the longest side is always in position 2.
        if side_lengths[0] + side_lengths[1] > side_lengths[2] {
            possible_triangles += 1;
        }
    }

    Solution::U32(possible_triangles)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("5 10 25"), Solution::U8(0));
    }
}
