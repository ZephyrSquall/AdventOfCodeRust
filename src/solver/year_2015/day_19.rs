use crate::solver::{Solution, Solver};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 19,
    title: "Medicine for Rudolph",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let (replacements, molecule) = get_replacements_and_molecule(input);
    let mut molecules_with_replacement = FxHashSet::default();

    for (index, atom) in molecule.iter().enumerate() {
        if let Some(replacements) = replacements.get(atom) {
            for replacement in replacements {
                // Clone the original molecule so the rest of the replacements still operate on the
                // original molecule.
                let mut new_molecule = molecule.clone();
                // Splice to remove the range with the original element (which is just a range with
                // one element, hence it both starts and ends on index). Splice consumes the
                // replacement vector, but this vector needs to remain available for future
                // replacements, hence the replacement vector must be cloned.
                new_molecule.splice(index..=index, replacement.clone());
                molecules_with_replacement.insert(new_molecule);
            }
        }
    }

    Solution::USize(molecules_with_replacement.len())
}

// Parse the input into a hash map of replacement rules (keyed by the input atom with the value
// being a vector of all possible replacement molecules from that input atom) and the molecule given
// at the bottom of the puzzle input. Molecules are a vector of strings containing the atom names in
// order.
fn get_replacements_and_molecule(
    input: &str,
) -> (FxHashMap<String, Vec<Vec<String>>>, Vec<String>) {
    let mut replacements = FxHashMap::default();

    let mut line_iter = input.lines();
    loop {
        let line = line_iter
            .next()
            .expect("Input shouldn't run out of lines before this loop is exited");

        // An empty line indicates the end of the replacement rules, and that the following line is
        // the molecule.
        if line.is_empty() {
            break;
        }

        let mut word_iter = line.split(' ');
        let input = word_iter
            .next()
            .expect("Line should have first word")
            .to_string();

        let output_str = word_iter.next_back().expect("Line should have last word");
        let output = get_molecule(output_str);

        // Get the atom's replacement rules vector from the hashmap. If it isn't already in the
        // hashmap, add it with an empty vector. Then push the new replacement molecule to that
        // vector.
        let replacement_entry = replacements.entry(input).or_insert(Vec::new());
        replacement_entry.push(output);
    }

    let molecule_str = line_iter
        .next()
        .expect("Input should have one final line containing the molecule");
    let molecule = get_molecule(molecule_str);

    (replacements, molecule)
}

// Turns a string of concatenated atom names into a vector with each element being the name of a
// different atom. This method assumes that all atoms have either a one-character name which is
// uppercase, or a two-character name in which the first character is uppercase and the second
// character is lowercase. In other words, an atom always starts with one uppercase letter, and
// optionally has one additional lowercase letter.
fn get_molecule(molecule_str: &str) -> Vec<String> {
    let mut molecule = Vec::new();

    for (char_1, char_2) in molecule_str.chars().tuple_windows() {
        // If the first character is not uppercase, then this window landed in the middle of a
        // two-character atom, so do nothing this loop.
        if char_1.is_ascii_uppercase() {
            let atom = if char_2.is_ascii_lowercase() {
                // If the second character is lowercase, then both characters together represent one
                // atom, so combine them into a single string.
                [char_1, char_2].iter().collect::<String>()
            } else {
                // Otherwise both characters are capital letters, which means the first character is
                // a single-character atom and the second character is the start of the next atom,
                // so only put the first character into a string.
                char_1.to_string()
            };
            molecule.push(atom);
        }
    }

    // Check last character in case the molecule ends with a single-character atom.
    let last_char = molecule_str
        .chars()
        .next_back()
        .expect("Molecule should have a final character");
    if last_char.is_ascii_uppercase() {
        molecule.push(last_char.to_string());
    }

    molecule
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
H => HO
H => OH
O => HH

HOH"
            ),
            Solution::U8(4)
        );
    }

    #[test]
    fn example1_2() {
        assert_eq!(
            solve_1(
                "\
H => HO
H => OH
O => HH

HOHOHO"
            ),
            Solution::U8(7)
        );
    }
}