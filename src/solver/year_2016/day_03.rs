use crate::solver::{Solution, AdventOfCode};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2016,
    day: 3,
    title: "Squares With Three Sides",
    part_solvers: &[solve_1, solve_2],
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

fn solve_2(input: &str) -> Solution {
    let mut possible_triangles = 0;

    // The input is still explored line-by-line, which causes three triangles to be built
    // simultaneously, so use an array to keep track of three triangles at once.
    let mut triangles = [
        Vec::with_capacity(3),
        Vec::with_capacity(3),
        Vec::with_capacity(3),
    ];
    let mut triangle_len = 0;

    for line in input.lines() {
        for (triangle_index, side_length) in line
            .split_whitespace()
            .map(|str| {
                str.parse::<u32>()
                    .expect("Each line should have only numbers")
            })
            .enumerate()
        {
            triangles[triangle_index].push(side_length);
        }
        triangle_len += 1;

        // When the triangles are fully-built, check all three to see if they are valid triangles,
        // then clear them so they're ready to build the next three triangles.
        if triangle_len == 3 {
            for side_lengths in &mut triangles {
                side_lengths.sort_unstable();
                if side_lengths[0] + side_lengths[1] > side_lengths[2] {
                    possible_triangles += 1;
                }
                side_lengths.clear();
            }
            triangle_len = 0;
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
