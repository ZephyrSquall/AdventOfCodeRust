use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 3,
    title: "Mull It Over",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let mut sum_of_multiplications = 0;

    // A beginning of a valid multiplication instruction always starts with "mul(".
    for mul_candidate in input.split("mul(") {
        // Get the text between the start of this text split and the next comma.
        let mut comma_split = mul_candidate.split(',');
        let first_operand_candidate = comma_split
            .next()
            .expect("A split always has a first element");
        // Check that this text forms a valid number.
        if let Ok(first_operand) = first_operand_candidate.parse::<u32>() {
            // Check that a comma was found and get the rest of the text after it.
            if let Some(comma_split_remainder) = comma_split.next() {
                // Get the text between the start of this text split and the next closing bracket.
                let mut bracket_split = comma_split_remainder.split(')');
                let second_operand_candidate = bracket_split
                    .next()
                    .expect("A split always has a first element");
                // Check that this text forms a valid number.
                if let Ok(second_operand) = second_operand_candidate.parse::<u32>() {
                    // All validity checks have passed, so multiply the values and add to the
                    // running sum.
                    sum_of_multiplications += first_operand * second_operand;
                }
            }
        }
    }

    Solution::U32(sum_of_multiplications)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            Solution::U8(161)
        );
    }
}
