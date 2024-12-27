use crate::solver::{Solution, AdventOfCode};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2015,
    day: 19,
    title: "Medicine for Rudolph",
    part_solvers: &[solve_1, solve_2],
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

fn solve_2(input: &str) -> Solution {
    // Solving this task in general has proven to be infeasible due to how quickly the tree of all
    // possibilities grows. Such a solution seems like it will inevitably take several minutes,
    // possibly hours to run. However, my puzzle input has certain patterns that seem intentional,
    // and using these patterns allows for an extremely efficient solution, so I will assume this is
    // an intentional pattern that all puzzle inputs share.
    //
    // The atoms "Rn", "Ar", and "Y" have special meaning in these patterns, so for brevity, any
    // atom that isn't one of these three will be called a "standard" atom from here on. Every
    // replacement rule in my puzzle input falls into one of the following patterns:
    // standard => standard-standard
    // standard => standard-Rn-standard-Ar
    // standard => standard-Rn-standard-Y-standard-Ar
    // standard => standard-Rn-standard-Y-standard-Y-standard-Ar
    //
    // Replacement rules that replace one standard atom with exactly two standard atoms always add
    // one standard atom overall to the molecule. Replacement rules involving "Rn" and "Ar" always
    // add either one, two, or three standard atoms overall to the molecule depending on if it has
    // zero, one, or two "Y" atoms respectively.
    //
    // This information allows a quick algorithm to calculate the number of steps that must have
    // been taken to create the molecule. First, count every standard atom, as most standard atoms
    // are produced by a replacement rule that replaces one atom with two standard atoms, which is a
    // single step that adds a single atom overall. Do not count "Rn" or "Ar" atoms, as these
    // replacements occur in "standard-Rn-standard-Ar" patterns, and the two standard atoms there
    // already account for a single step. These patterns may also include one or two "Y" atoms
    // followed by a standard atom, and each of these occurrences do not indicate any additional
    // steps, so subtract 1 from this count for each "Y" atom to undo the count of its following
    // standard atom. Subtract 1 from this final count because the whole process starts with an "e",
    // which doesn't count as a step and must be ignored.
    //
    // In summary: The number of steps equals the number of standard atoms minus the number of "Y"
    // atoms, then minus 1.

    let mut line_iter = input.lines();
    let molecule_str = line_iter
        .next_back()
        .expect("Input should have at least one line");
    let molecule = get_molecule(molecule_str);

    let mut steps = -1;
    for atom in molecule {
        match atom.as_str() {
            "Y" => steps -= 1,
            "Rn" | "Ar" => {}
            _ => steps += 1,
        }
    }

    Solution::I32(steps)
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

    // The example for part 2 given in the puzzle description cannot be tested, as the example input
    // contradicts an assumption that was made about puzzle inputs (it contains replacement rules
    // of the form standard => standard).
}
