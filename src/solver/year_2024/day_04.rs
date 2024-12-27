use crate::solver::{Solution, AdventOfCode};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2024,
    day: 4,
    title: "Ceres Search",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let letters = get_letters(input);

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

fn solve_2(input: &str) -> Solution {
    let letters = get_letters(input);

    // Iterate over every character in the letters array, except for those in the first row, last
    // row, first column, or last column as the center of an X-MAS cannot appear on the edges.
    // Ignore all characters other than 'A'. When an 'A' is found, check the four
    // diagonally-adjacent characters to see if they consist of exactly two 'M' and two 'S', and
    // that the upper-left and lower-right characters aren't both 'M' or the upper-right and
    // lower-left characters aren't both 'M' to avoid any "MAM" patterns (a "SAS" pattern only
    // exists alongside a "MAM" pattern so there is no need to check for "SAS" patterns too).
    let mut cross_mas_appearances = 0;
    let x_len = letters[0].len();
    let y_len = letters.len();
    for (y, letter_line) in letters.iter().enumerate().skip(1) {
        if y + 1 == y_len {
            break;
        }
        for (x, letter) in letter_line.iter().enumerate().skip(1) {
            if x + 1 == x_len {
                break;
            }
            if *letter == 'A' {
                // By skipping the edges of the array, bounds checks are not required.
                let mut m_count = 0;
                let mut s_count = 0;

                // Clippy is unable to determine that that proper checks are in place to make sure
                // accessing these indexes don't panic.
                #[allow(clippy::match_on_vec_items)]
                match letters[y - 1][x - 1] {
                    'M' => m_count += 1,
                    'S' => s_count += 1,
                    // If an 'A' or 'X' is ever encountered, there's no way for this to be an X-MAS,
                    // so skip to the next letter.
                    _ => continue,
                }
                #[allow(clippy::match_on_vec_items)]
                match letters[y - 1][x + 1] {
                    'M' => m_count += 1,
                    'S' => s_count += 1,
                    _ => continue,
                }
                #[allow(clippy::match_on_vec_items)]
                match letters[y + 1][x - 1] {
                    'M' => m_count += 1,
                    'S' => s_count += 1,
                    _ => continue,
                }
                #[allow(clippy::match_on_vec_items)]
                match letters[y + 1][x + 1] {
                    'M' => m_count += 1,
                    'S' => s_count += 1,
                    _ => continue,
                }

                if m_count == 2
                    && s_count == 2
                    && !(letters[y - 1][x - 1] == 'M' && letters[y + 1][x + 1] == 'M')
                    && !(letters[y - 1][x + 1] == 'M' && letters[y + 1][x - 1] == 'M')
                {
                    cross_mas_appearances += 1;
                }
            }
        }
    }

    Solution::U32(cross_mas_appearances)
}

// Create a vector of vectors (an array) to store each character in its original position.
fn get_letters(input: &str) -> Vec<Vec<char>> {
    let mut letters = Vec::new();
    for line in input.lines() {
        let mut letters_line = Vec::new();
        for character in line.chars() {
            letters_line.push(character);
        }
        letters.push(letters_line);
    }

    letters
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

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
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
            Solution::U8(9)
        );
    }
}
