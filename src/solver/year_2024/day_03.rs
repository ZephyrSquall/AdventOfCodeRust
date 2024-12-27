use crate::solver::{Solution, AdventOfCode};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2024,
    day: 3,
    title: "Mull It Over",
    part_solvers: &[solve_1, solve_2],
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

fn solve_2(input: &str) -> Solution {
    // The following enums track the next expected character in "mul(XXX,XXX)", "do()", and
    // "don't()".
    enum MulNext {
        M,
        U,
        L,
        OpenBracket,
        Comma,
        CloseBracket,
    }
    enum DoNext {
        D,
        O,
        OpenBracket,
        CloseBracket,
    }
    enum DontNext {
        D,
        O,
        N,
        Apostrophe,
        T,
        OpenBracket,
        CloseBracket,
    }
    let mut mul_next = MulNext::M;
    let mut do_next = DoNext::D;
    let mut dont_next = DontNext::D;

    let mut sum_of_multiplications = 0;
    let mut are_multiplications_enabled = true;
    let mut first_operand_characters = Vec::with_capacity(3);
    let mut second_operand_characters = Vec::with_capacity(3);

    for character in input.chars() {
        // if multiplications are enabled, look for mul and don't statements.
        if are_multiplications_enabled {
            match mul_next {
                MulNext::M => {
                    if character == 'm' {
                        mul_next = MulNext::U;
                    }
                }
                MulNext::U => {
                    if character == 'u' {
                        mul_next = MulNext::L;
                    } else {
                        mul_next = MulNext::M;
                    }
                }
                MulNext::L => {
                    if character == 'l' {
                        mul_next = MulNext::OpenBracket;
                    } else {
                        mul_next = MulNext::M;
                    }
                }
                MulNext::OpenBracket => {
                    if character == '(' {
                        mul_next = MulNext::Comma;
                    } else {
                        mul_next = MulNext::M;
                    }
                }
                MulNext::Comma => {
                    // While looking for a comma, also accept digits.
                    if character == ',' {
                        mul_next = MulNext::CloseBracket;
                    } else if character.is_ascii_digit() {
                        first_operand_characters.push(character);
                    } else {
                        first_operand_characters.clear();
                        mul_next = MulNext::M;
                    }
                }
                MulNext::CloseBracket => {
                    if character == ')' {
                        // Collect the vectors of characters into strings, then parse those strings
                        // into numbers.
                        let first_operand = first_operand_characters.iter().collect::<String>().parse::<u32>().expect("Vector should only contain characters which can be parsed to a number");
                        let second_operand = second_operand_characters.iter().collect::<String>().parse::<u32>().expect("Vector should only contain characters which can be parsed to a number");

                        sum_of_multiplications += first_operand * second_operand;
                        first_operand_characters.clear();
                        second_operand_characters.clear();
                        mul_next = MulNext::M;
                    } else if character.is_ascii_digit() {
                        second_operand_characters.push(character);
                    } else {
                        first_operand_characters.clear();
                        second_operand_characters.clear();
                        mul_next = MulNext::M;
                    }
                }
            }

            match dont_next {
                DontNext::D => {
                    if character == 'd' {
                        dont_next = DontNext::O;
                    }
                }
                DontNext::O => {
                    if character == 'o' {
                        dont_next = DontNext::N;
                    } else {
                        dont_next = DontNext::D;
                    }
                }
                DontNext::N => {
                    if character == 'n' {
                        dont_next = DontNext::Apostrophe;
                    } else {
                        dont_next = DontNext::D;
                    }
                }
                DontNext::Apostrophe => {
                    if character == '\'' {
                        dont_next = DontNext::T;
                    } else {
                        dont_next = DontNext::D;
                    }
                }
                DontNext::T => {
                    if character == 't' {
                        dont_next = DontNext::OpenBracket;
                    } else {
                        dont_next = DontNext::D;
                    }
                }
                DontNext::OpenBracket => {
                    if character == '(' {
                        dont_next = DontNext::CloseBracket;
                    } else {
                        dont_next = DontNext::D;
                    }
                }
                DontNext::CloseBracket => {
                    if character == ')' {
                        are_multiplications_enabled = false;
                    }
                    dont_next = DontNext::D;
                }
            }
        } else {
            match do_next {
                DoNext::D => {
                    if character == 'd' {
                        do_next = DoNext::O;
                    }
                }
                DoNext::O => {
                    if character == 'o' {
                        do_next = DoNext::OpenBracket;
                    } else {
                        do_next = DoNext::D;
                    }
                }
                DoNext::OpenBracket => {
                    if character == '(' {
                        do_next = DoNext::CloseBracket;
                    } else {
                        do_next = DoNext::D;
                    }
                }
                DoNext::CloseBracket => {
                    if character == ')' {
                        are_multiplications_enabled = true;
                    }
                    do_next = DoNext::D;
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

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            Solution::U8(48)
        );
    }
}
