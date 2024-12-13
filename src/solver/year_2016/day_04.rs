use crate::solver::{Solution, Solver};
use rustc_hash::FxHashMap;

pub const SOLVER: Solver = Solver {
    year: 2016,
    day: 4,
    title: "Security Through Obscurity",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let mut sector_id_sum = 0;

    for line in input.lines() {
        let mut letter_counts = FxHashMap::default();
        let mut sector_id_digits = Vec::default();

        let mut character_iter = line.chars();
        // Loop over all characters until the checksum, indicated by the '[' character.
        loop {
            let character = character_iter
                .next()
                .expect("Line should not end before the checksum is reached");
            if character == '[' {
                break;
            }
            // Ignore dashes.
            if character != '-' {
                if character.is_ascii_digit() {
                    // Track numeric digits so the sector id can be calculated if this is a real
                    // room.
                    sector_id_digits.push(character);
                } else {
                    // Track how many of each letter is seen.
                    letter_counts
                        .entry(character)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        }

        // Convert letter_counts from a map to a vector so it can be sorted.
        let mut letter_counts = letter_counts.into_iter().collect::<Vec<_>>();
        letter_counts.sort_unstable_by(|a, b| {
            if a.1 == b.1 {
                // If the counts are the same, sort by alphabetical order (Rust sorts characters
                // alphabetically by default).
                return a.0.cmp(&b.0);
            }
            // Numeric comparisons need to be reversed as this array should be sorted largest to
            // smallest.
            a.1.cmp(&b.1).reverse()
        });

        // For the five checksum digits, check that they match the first five elements of the
        // now-sorted letter_counts.
        let mut is_checksum_valid = true;
        for ordered_letter in letter_counts
            .iter()
            .take(5)
            .map(|letter_count| letter_count.0)
        {
            if ordered_letter
                != character_iter
                    .next()
                    .expect("Checksum should have 5 characters")
            {
                is_checksum_valid = false;
                break;
            }
        }

        // If the checksum is valid, convert the sector id into an int and add it to the sum.
        if is_checksum_valid {
            let sector_id = sector_id_digits
                .iter()
                .collect::<String>()
                .parse::<u32>()
                .expect("Sector ID should be a number");
            sector_id_sum += sector_id;
        }
    }
    Solution::U32(sector_id_sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]"
            ),
            Solution::U16(1514)
        );
    }
}
