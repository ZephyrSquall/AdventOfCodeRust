use crate::solver::{Solution, AdventOfCode};
use rustc_hash::FxHashMap;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2016,
    day: 4,
    title: "Security Through Obscurity",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let mut sector_id_sum = 0;

    for line in input.lines() {
        let mut letter_counts = FxHashMap::default();
        let mut sector_id_digits = Vec::new();

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

// Assume that the room we're looking for is the only room that contains the words "north-pole".
fn solve_2(input: &str) -> Solution {
    for line in input.lines() {
        let mut message = Vec::new();
        let mut letter_counts = FxHashMap::default();
        let mut sector_id_digits = Vec::new();

        let mut character_iter = line.chars();
        loop {
            let character = character_iter
                .next()
                .expect("Line should not end before the checksum is reached");
            if character == '[' {
                break;
            }

            if character == '-' {
                // Insert dashes into the message
                message.push(character);
            } else {
                if character.is_ascii_digit() {
                    sector_id_digits.push(character);
                    // Don't insert numbers into the message.
                } else {
                    letter_counts
                        .entry(character)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                    // Insert letters into the message.
                    message.push(character);
                }
            }
        }

        let mut letter_counts = letter_counts.into_iter().collect::<Vec<_>>();
        letter_counts.sort_unstable_by(|a, b| {
            if a.1 == b.1 {
                return a.0.cmp(&b.0);
            }
            a.1.cmp(&b.1).reverse()
        });

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

        // If the checksum is valid, shift the characters in the message by the sector id, then
        // check if the room contains the north pole objects. The puzzle description isn't explicit
        // about exactly how it is determined that a room contains the north pole objects, so I
        // assume the puzzle input is structured so that this room is the only room in the puzzle
        // input that contains the words "north" and "pole", i.e. I assume the room whose sector ID
        // we want is the only room that contains the substrings "north" and "pole".
        if is_checksum_valid {
            let sector_id = sector_id_digits
                .iter()
                .collect::<String>()
                .parse::<u32>()
                .expect("Sector ID should be a number");

            // Every 26 shifts puts a letter back to its original value, so only the remainder after
            // the last multiple of 26 matters for shifting.
            let sector_id_shift = (sector_id % 26) as u8;
            for letter in &mut message {
                if *letter != '-' {
                    let mut letter_ascii = *letter as u8 + sector_id_shift;
                    // 122 is the ascii value for 'z'. If this value is exceeded, subtract 26 to
                    // wrap around to the start of the alphabet. It is assumed that all letters in
                    // the input are lowercase.
                    if letter_ascii > 122 {
                        letter_ascii -= 26;
                    }

                    *letter = letter_ascii as char;
                }
            }

            let message = message.into_iter().collect::<String>();
            if message.contains("north") && message.contains("pole") {
                return Solution::U32(sector_id);
            }
        }
    }
    panic!("Should have found the room with the North Pole objects")
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
