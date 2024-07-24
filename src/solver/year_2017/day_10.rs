use crate::solver::{Solution, Solver};
use std::fmt::Write;

pub const SOLVER: Solver = Solver {
    year: 2017,
    day: 10,
    title: "Knot Hash",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let lengths = input
        .split(',')
        .map(|s| s.parse::<u8>().expect("Error parsing number"))
        .collect::<Vec<u8>>();
    let mut knot_hash = KnotHashResult {
        list: (0..=255).collect(),
        position: 0,
        skip_size: 0,
    };
    knot_hash_round(&mut knot_hash, &lengths);

    Solution::U16(u16::from(knot_hash.list[0]) * u16::from(knot_hash.list[1]))
}

fn solve_2(input: &str) -> Solution {
    let lengths = [input.as_bytes(), &[17, 31, 73, 47, 23]].concat();
    let mut result = KnotHashResult {
        list: (0..=255).collect(),
        position: 0,
        skip_size: 0,
    };
    for _ in 0..64 {
        knot_hash_round(&mut result, &lengths);
    }

    let mut dense_hash = Vec::new();
    for block in result.list.chunks(16) {
        let mut dense_number = 0;
        for number in block {
            dense_number ^= number;
        }

        dense_hash.push(dense_number);
    }

    let dense_hash = dense_hash.iter().fold(String::new(), |mut acc, dh| {
        write!(acc, "{dh:0>2x}").expect("Writing to String can't fail");
        acc
    });
    Solution::String(dense_hash)
}

struct KnotHashResult {
    position: usize,
    skip_size: usize,
    list: Vec<u8>,
}

fn knot_hash_round(knot_hash: &mut KnotHashResult, lengths: &Vec<u8>) {
    for length in lengths {
        let length = *length as usize;

        // A length of 0 needs to be explicitly skipped as it will set up invalid initial
        // conditions. Fortunately, a length of 0 is a no-op anyway so no additional handling is
        // needed.
        if length != 0 {
            let mut start_position = knot_hash.position;
            let mut end_position = (start_position + length - 1) % knot_hash.list.len();

            while start_position != end_position {
                knot_hash.list.swap(start_position, end_position);

                start_position = (start_position + 1) % knot_hash.list.len();
                // If length is even, the positions will be equal at this intermediate step instead
                // of when both start_position and end_position are updated, so add an extra check
                // here to end the loop if they're equal.
                if start_position == end_position {
                    break;
                }
                // In mod n arithmetic, adding n - 1 is equivalent to subtracting 1. This trick is
                // used here to prevent the possibility of subtracting 1 from a usize with value 0
                // which would cause an overflow.
                end_position = (end_position + knot_hash.list.len() - 1) % knot_hash.list.len();
            }
        }

        knot_hash.position =
            (knot_hash.position + length + knot_hash.skip_size) % knot_hash.list.len();
        knot_hash.skip_size += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        let input = "3, 4, 1, 5";
        let lengths = input
            .split(", ")
            .map(|s| s.parse::<u8>().expect("Error parsing number"))
            .collect();
        let mut knot_hash = KnotHashResult {
            list: (0..5).collect(),
            position: 0,
            skip_size: 0,
        };
        knot_hash_round(&mut knot_hash, &lengths);
        assert_eq!(
            Solution::U16(u16::from(knot_hash.list[0]) * u16::from(knot_hash.list[1])),
            Solution::U8(12)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(""),
            Solution::Str("a2582a3a0e66e6e86e3812dcb672a272")
        );
    }
    #[test]
    fn example2_2() {
        assert_eq!(
            solve_2("AoC 2017"),
            Solution::Str("33efeb34ea91902bb2f59c9920caa6cd")
        );
    }
    #[test]
    fn example2_3() {
        assert_eq!(
            solve_2("1,2,3"),
            Solution::Str("3efbe78a8d82f29979031a4aa0b16a9d")
        );
    }
    #[test]
    fn example2_4() {
        assert_eq!(
            solve_2("1,2,4"),
            Solution::Str("63960835bcdc130f0b66d7ff4f6a5a8e")
        );
    }
}
