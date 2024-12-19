use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 19,
    title: "Linen Layout",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    #[derive(PartialEq)]
    enum Stripe {
        White,
        Blue,
        Black,
        Red,
        Green,
    }
    impl Stripe {
        fn new(letter: char) -> Stripe {
            match letter {
                'w' => Stripe::White,
                'u' => Stripe::Blue,
                'b' => Stripe::Black,
                'r' => Stripe::Red,
                'g' => Stripe::Green,
                _ => panic!("Stripe colour is incorrect"),
            }
        }
    }

    // Recursively check all arrangement of towels to see if the pattern can be created from the
    // available towels.
    fn can_pattern_come_from_towels(
        pattern: &[Stripe],
        towels: &[Vec<Stripe>],
        pattern_index: usize,
    ) -> bool {
        for towel in towels {
            // Check if every stripe in the towel matches the corresponding stripe in the pattern.
            if towel.iter().enumerate().all(|(towel_index, towel_stripe)| {
                pattern
                    // The towel might be longer than the pattern, so use get to return a None if
                    // the towel is too long.
                    .get(pattern_index + towel_index)
                    .is_some_and(|pattern_stripe| *pattern_stripe == *towel_stripe)
            }) {
                let new_pattern_index = pattern_index + towel.len();
                // If the new pattern index matches the length of the pattern, the pattern has been
                // matched perfectly, so return true.
                if new_pattern_index == pattern.len() {
                    return true;
                }
                // Otherwise, recursively check more towels to see if an arrangement of towels to
                // the end of the pattern exists.
                if can_pattern_come_from_towels(pattern, towels, new_pattern_index) {
                    return true;
                }
            }
        }

        // No arrangement of towels to the end of the pattern was found, so return false.
        false
    }

    let mut towels = Vec::new();
    let mut line_iter = input.lines();
    let towel_iter = line_iter
        .next()
        .expect("Input should have first line")
        .split(", ");

    // Get every available towel.
    for towel_string in towel_iter {
        let towel = towel_string.chars().map(Stripe::new).collect::<Vec<_>>();
        towels.push(towel);
    }

    // For every remaining pattern, check if it can be made with the available towels. Skip the
    // first line of line_iter as it is the blank line between the towels and the patterns.
    let mut possible_patterns = 0;
    for line in line_iter.skip(1) {
        let pattern = line.chars().map(Stripe::new).collect::<Vec<_>>();
        if can_pattern_come_from_towels(&pattern, &towels, 0) {
            possible_patterns += 1;
        }
    }
    Solution::U32(possible_patterns)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
            ),
            Solution::U8(6)
        );
    }
}
