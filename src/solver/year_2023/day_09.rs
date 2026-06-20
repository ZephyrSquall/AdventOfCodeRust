use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2023,
    day: 9,
    title: "Mirage Maintenance",
    part_solvers: &[solve_1, solve_2],
};

fn predict_next(values: &[i32]) -> i32 {
    if values.iter().all(|&x| x == 0) {
        return 0;
    }

    let difference_values = values
        .windows(2)
        .map(|value_pair| value_pair[1] - value_pair[0])
        .collect::<Vec<_>>();
    let next_difference = predict_next(&difference_values);

    values
        .last()
        .expect("Values should have at least one value")
        + next_difference
}

fn solve_1(input: &str) -> Solution {
    let extrapolated_value_sum = input.lines().fold(0, |acc, e| {
        let values = e
            .split(' ')
            .map(|value_str| {
                value_str
                    .parse()
                    .expect("Value string should be a valid number")
            })
            .collect::<Vec<_>>();

        acc + predict_next(&values)
    });

    Solution::I32(extrapolated_value_sum)
}

fn solve_2(input: &str) -> Solution {
    let extrapolated_value_sum = input.lines().fold(0, |acc, e| {
        let values = e
            .split(' ')
            .rev()
            .map(|value_str| {
                value_str
                    .parse()
                    .expect("Value string should be a valid number")
            })
            .collect::<Vec<_>>();

        acc + predict_next(&values)
    });

    Solution::I32(extrapolated_value_sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            Solution::U8(114)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            Solution::U8(2)
        );
    }
}
