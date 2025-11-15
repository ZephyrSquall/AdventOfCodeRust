use std::ops::IndexMut;

use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2016,
    day: 8,
    title: "Two-Factor Authentication",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let screen = generate_screen::<50, 6>(input);
    let lit_pixels = count_lit_pixels(&screen);

    Solution::U32(lit_pixels)
}

fn generate_screen<const X: usize, const Y: usize>(input: &str) -> [[bool; X]; Y] {
    let mut screen = [[false; X]; Y];

    for line in input.lines() {
        let mut space_iter = line.split(' ');
        let keyword = space_iter.next().expect("Line should have keyword");
        if keyword == "rect" {
            let dimensions = space_iter
                .next()
                .expect("Rect instruction should have dimensions");
            let mut dimensions_iter = dimensions.split('x');
            let width = dimensions_iter
                .next()
                .expect("Dimensions should have width")
                .parse::<usize>()
                .expect("Width should be a number");
            let height = dimensions_iter
                .next()
                .expect("Dimensions should have height")
                .parse::<usize>()
                .expect("Height should be a number");

            for row in screen.iter_mut().take(height) {
                for pixel in row.iter_mut().take(width) {
                    *pixel = true;
                }
            }
        } else {
            // If not "rect", then the keyword must be "rotate"
            let direction = space_iter.next().expect("Line should have direction");
            let instruction = line
                .split('=')
                .next_back()
                .expect("Line should have instruction");
            let mut instruction_iter = instruction.split(" by ");
            let index = instruction_iter
                .next()
                .expect("Instruction should have index")
                .parse::<usize>()
                .expect("Index should be a number");
            let distance = instruction_iter
                .next()
                .expect("Instruction should have distance")
                .parse::<usize>()
                .expect("Distance should be a number");

            if direction == "row" {
                screen[index].rotate_right(distance);
            } else {
                // If not "row", then the keyword must be "column"

                // Get a slice along a column.
                let mut column = screen
                    .iter_mut()
                    .map(|row| row.index_mut(index))
                    .collect::<Vec<_>>();

                // The rotate_right method does not work on this column vector. It rotates the
                // references, not the data underneath the references like we want. Use the custom
                // rotate_underlying function instead to rotate the bools themselves.
                for _ in 0..distance {
                    rotate_underlying(&mut column);
                }
            }
        }
    }

    screen
}

fn count_lit_pixels<const X: usize, const Y: usize>(screen: &[[bool; X]; Y]) -> u32 {
    let mut lit_pixels = 0;
    for row in screen {
        for pixel in row {
            if *pixel {
                lit_pixels += 1;
            }
        }
    }

    lit_pixels
}

// Rotates the underlying boolean value under a mutable reference once. Call this function multiple
// times for bigger rotations (optimizing rotations for larger numbers while respecting Rust's
// borrowing rules is very difficult without using unsafe).
fn rotate_underlying(column: &mut [&mut bool]) {
    let temp = *column[0];
    *column[0] = *column[column.len() - 1];
    for i in (0..column.len()).rev().skip(1) {
        let j = i + 1;
        *column[j] = *column[i];
    }
    *column[1] = temp;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        let screen = generate_screen::<7, 3>(
            "\
rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1",
        );
        let lit_pixels = count_lit_pixels(&screen);
        assert_eq!(lit_pixels, 6);
    }
}
