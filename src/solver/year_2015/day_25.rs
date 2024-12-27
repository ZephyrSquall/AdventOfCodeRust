use crate::solver::{Solution, AdventOfCode};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2015,
    day: 25,
    title: "Let It Snow",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    // Get the target row and column from the puzzle input.
    let input_start_trimmed = input.trim_start_matches(
        "To continue, please consult the code grid in the manual.  Enter the code at row ",
    );
    let input_end_trimmed = input_start_trimmed.trim_end_matches('.');
    let mut input_iter = input_end_trimmed.split(", column ");
    let target_row = input_iter
        .next()
        .expect("Input should have a first value after trimming")
        .parse::<u32>()
        .expect("Input's first value should be a number after trimming");
    let target_column = input_iter
        .next()
        .expect("Input should have a second value after trimming")
        .parse::<u32>()
        .expect("Input's second value should be a number after trimming");

    let mut code = 20_151_125;
    let mut row = 1;
    let mut column = 1;

    // Keep generating new codes until the row and column numbers match the target row and column.
    while !(row == target_row && column == target_column) {
        code = (code * 252_533) % 33_554_393;

        if row == 1 {
            row = column + 1;
            column = 1;
        } else {
            row -= 1;
            column += 1;
        }
    }

    Solution::U64(code)
}

#[cfg(test)]
mod test {
    use super::*;

    // This test is created from the last example value given in the table of examples.
    #[test]
    fn example1_1() {
        assert_eq!(solve_1("To continue, please consult the code grid in the manual.  Enter the code at row 6, column 6."), Solution::U32(27_995_004));
    }
}
