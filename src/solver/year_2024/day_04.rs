use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 4,
    title: "Ceres Search",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    // Create a vector of vectors (an array) to store each character in its original position.
    let mut letters = Vec::new();
    for line in input.lines() {
        let mut letters_line = Vec::new();
        for character in line.chars() {
            letters_line.push(character);
        }
        letters.push(letters_line);
    }

    // Iterate over every character in the letters array. Ignore all characters other than 'X'. When
    // an 'X' is found, check in all 8 directions from that 'X' if the rest of the word "XMAS" is in
    // that direction.
    let mut xmas_appearances = 0;
    let x_len = letters[0].len();
    let y_len = letters.len();
    for (y, letter_line) in letters.iter().enumerate() {
        for (x, letter) in letter_line.iter().enumerate() {
            if *letter == 'X' {
                // Only search up-left, up, and up-right if y >= 3 (as otherwise there isn't enough
                // room for the full word in any of those directions)
                if y >= 3 {
                    // Search up-left.
                    if x >= 3
                        && letters[y - 1][x - 1] == 'M'
                        && letters[y - 2][x - 2] == 'A'
                        && letters[y - 3][x - 3] == 'S'
                    {
                        xmas_appearances += 1;
                    }
                    // Search up.
                    if letters[y - 1][x] == 'M'
                        && letters[y - 2][x] == 'A'
                        && letters[y - 3][x] == 'S'
                    {
                        xmas_appearances += 1;
                    }
                    // Search up-right.
                    if x + 3 < x_len
                        && letters[y - 1][x + 1] == 'M'
                        && letters[y - 2][x + 2] == 'A'
                        && letters[y - 3][x + 3] == 'S'
                    {
                        xmas_appearances += 1;
                    }
                }
                // Search left.
                if x >= 3
                    && letters[y][x - 1] == 'M'
                    && letters[y][x - 2] == 'A'
                    && letters[y][x - 3] == 'S'
                {
                    xmas_appearances += 1;
                }
                // Search right.
                if x + 3 < x_len
                    && letters[y][x + 1] == 'M'
                    && letters[y][x + 2] == 'A'
                    && letters[y][x + 3] == 'S'
                {
                    xmas_appearances += 1;
                }
                // Only search down-left, down, and down-right if y + 3 < y_len (as otherwise there isn't enough
                // room for the full word in any of those directions)
                if y + 3 < y_len {
                    // Search down-left.
                    if x >= 3
                        && letters[y + 1][x - 1] == 'M'
                        && letters[y + 2][x - 2] == 'A'
                        && letters[y + 3][x - 3] == 'S'
                    {
                        xmas_appearances += 1;
                    }
                    // Search down.
                    if letters[y + 1][x] == 'M'
                        && letters[y + 2][x] == 'A'
                        && letters[y + 3][x] == 'S'
                    {
                        xmas_appearances += 1;
                    }
                    // Search down-right.
                    if x + 3 < x_len
                        && letters[y + 1][x + 1] == 'M'
                        && letters[y + 2][x + 2] == 'A'
                        && letters[y + 3][x + 3] == 'S'
                    {
                        xmas_appearances += 1;
                    }
                }
            }
        }
    }

    Solution::U32(xmas_appearances)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            ),
            Solution::U8(18)
        );
    }
}
