use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2024,
    day: 2,
    title: "Red-Nosed Reports",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let mut safe_reports = 0;

    for line in input.lines() {
        let values = line
            .split_whitespace()
            .map(|value| value.parse::<u32>().expect("Value should be a number"));

        if is_report_safe(values, None) {
            safe_reports += 1;
        }
    }

    Solution::U32(safe_reports)
}

fn solve_2(input: &str) -> Solution {
    let mut safe_reports = 0;

    for line in input.lines() {
        // The values in this iterator must be traversed an unknown number of times, so it must
        // always be cloned before it is used.
        let values = line
            .split_whitespace()
            .map(|value| value.parse::<u32>().expect("Value should be a number"));

        if is_report_safe(values.clone(), None) {
            safe_reports += 1;
        } else {
            // If the report isn't inherently safe, then for each value, try removing it and see if
            // the rest of the report is safe. If so, the problem dampener can make the report safe.
            for i in 0..values.clone().count() {
                if is_report_safe(values.clone(), Some(i)) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }
    Solution::U32(safe_reports)
}

fn is_report_safe(values: impl Iterator<Item = u32>, ignore_index: Option<usize>) -> bool {
    let mut is_safe = true;
    // is_ascending is None until the second element determines the direction of the sequence.
    let mut is_ascending = None;
    // previous_value is None until after the first element is checked.
    let mut previous_value = None;

    for (index, value) in values.enumerate() {
        // Immediately skip to the next value if this value has the index of the value to be
        // skipped. Note that this will not update previous_value, making sure this value is
        // completely ignored.
        if Some(index) != ignore_index {
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
        } // end if Some(index) != ignore_index
    }

    is_safe
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

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            ),
            Solution::U8(4)
        );
    }
}
