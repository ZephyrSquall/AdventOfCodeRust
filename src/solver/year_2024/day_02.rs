use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 2,
    title: "Red-Nosed Reports",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let mut safe_reports = 0;

    for line in input.lines() {
        let mut is_safe = true;
        // is_ascending is None until the second element determines the direction of the sequence.
        let mut is_ascending = None;
        // previous_value is None until after the first element is checked.
        let mut previous_value = None;

        for value in line.split_whitespace() {
            let value = value.parse::<u32>().expect("Value should be a number");

            // If previous_value is None, then this is the first value of the line, so there's
            // nothing to compare it to. Within this if statement, previous_value is shadowed so it
            // can be treated as a u32 instead of an Option<u32>.
            if let Some(previous_value) = previous_value {
                match is_ascending {
                    None => {
                        // If is_ascending is None, then this is the second value of the line, so
                        // there's no direction to take into account. All that matters is the
                        // elements aren't equal and don't differ by more than 3.
                        if value == previous_value || value.abs_diff(previous_value) > 3 {
                            is_safe = false;
                            break;
                        }
                        // This is the second element and so far the sequence is safe, so determine
                        // its direction.
                        is_ascending = Some(value > previous_value);
                    }
                    Some(true) => {
                        if value <= previous_value || value > previous_value + 3 {
                            is_safe = false;
                            break;
                        }
                    }
                    Some(false) => {
                        if value >= previous_value || value + 3 < previous_value {
                            is_safe = false;
                            break;
                        }
                    }
                }
            } // end if let Some(previous_value) = previous_value

            previous_value = Some(value);
        } // end for value in line.split_whitespace()

        if is_safe {
            safe_reports += 1;
        }
    }

    Solution::U32(safe_reports)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            ),
            Solution::U8(2)
        );
    }
}
