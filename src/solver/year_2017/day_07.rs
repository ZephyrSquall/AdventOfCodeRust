use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2017,
    day: 7,
    title: "Recursive Circus",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let mut programs = Vec::new();
    let mut above_programs = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        programs.push(iter.next().expect("Error reading first value of line"));
        // call next() twice and discard the value to get past the bracketed number and the "->"
        // elements in the line.
        iter.next();
        iter.next();

        // After iterating three times, iter is at the first element of the list of above programs
        // (or None if this program has no programs above it). Loop over the rest of the iterator to
        // put these in the list of above programs and remove trailing commas.
        for above_program in iter {
            above_programs.push(above_program.trim_end_matches(','));
        }
    }

    // The bottom program is the only program that isn't above any other program, meaning it's the
    // only program that is in programs but not above_programs.
    for program in programs {
        if !above_programs.contains(&program) {
            return Solution::String(program.to_string());
        }
    }

    // If no value is returned from the above loop, either the input is malformed or this solver has
    // a logic error. Either way, no valid solution can be returned, so panic.
    panic!("No bottom program found");
}

fn solve_2(input: &str) -> Solution {
    struct Disc {
        weight: usize,            // Weight of just this disc.
        cumulative_weight: usize, // Weight of this disc plus the weights of all sub-towers on it.
        sub_towers: Vec<Disc>,
    }

    impl Disc {
        pub fn new(input: &str, program: &str) -> Disc {
            // The logic within building the disc and populating the cumulative weights must be
            // moved into separate functions so they can be called recursively.
            let mut disc = Disc::build_disc(input, program);
            disc.populate_cumulative_weights();
            disc
        }

        fn build_disc(input: &str, program: &str) -> Disc {
            // Search the input for the line starting with the given program name.
            for line in input.lines() {
                let mut iter = line.split_whitespace();
                if iter.next() == Some(program) {
                    let weight = iter
                        .next()
                        .expect("Error reading second value of line")
                        .trim_matches(|c| c == '(' || c == ')')
                        .parse()
                        .expect("Error reading number");

                    // Consume the "->".
                    iter.next();

                    let sub_towers = iter
                        .map(|program| Disc::build_disc(input, program.trim_end_matches(',')))
                        .collect();

                    return Disc {
                        weight,
                        cumulative_weight: 0,
                        sub_towers,
                    };
                }
            }

            // If no value is returned from the above loop, then the program name wasn't found in
            // any of the lines of the input. This can only happen if the input is malformed or this
            // solver has a logic error. Either way, no valid Disc can be returned, so panic.
            panic!("No matching program found");
        }

        fn populate_cumulative_weights(&mut self) -> usize {
            // Get the weight of this disc.
            let mut cumulative_weight = self.weight;

            // Add the weights of all sub-towers on this disc.
            for sub_tower in &mut self.sub_towers {
                cumulative_weight += sub_tower.populate_cumulative_weights();
            }

            self.cumulative_weight = cumulative_weight;
            cumulative_weight
        }
    }

    fn fix_wrong_weight(correct_cumulative_weight: usize, disc: Disc) -> usize {
        // We are only concerned with sub-towers that can have the incorrect value, so iterate over
        // all sub-towers to find the one with the wrong value.
        for sub_tower in disc.sub_towers {
            if sub_tower.cumulative_weight != correct_cumulative_weight
                && !sub_tower.sub_towers.is_empty()
            {
                // Check if all sub-towers have equal cumulative weights. If so, the current disc
                // must be the incorrect disc (as there is exactly one incorrect disc, none of the
                // sub-towers can be incorrect, and this function is only called on discs with
                // incorrect cumulative weights). It's assumed that no disk has exactly one
                // sub-tower, as then the puzzle would have an ambiguous answer as it would be
                // impossible to tell whether the incorrect value lies with that disc or its single
                // sub-tower. Unlike with the very bottom disc, tracking the correct cumulative
                // weight makes it possible to identify which sub-tower has the wrong weight when
                // there's exactly two of them.
                let first_sub_tower_cumulative_weight = sub_tower.sub_towers[0].cumulative_weight;
                let are_sub_towers_equal = sub_tower.sub_towers.iter().all(|sub_tower| {
                    sub_tower.cumulative_weight == first_sub_tower_cumulative_weight
                });

                if are_sub_towers_equal {
                    // This sub-tower has been identified as the disc with the incorrect weight, so
                    // calculate what its weight should be.
                    //
                    // Let the weight be w, cumulative weight be c, correct weight be wc, correct
                    // cumulative weight be cc, and the sum of weights of all sub-towers be s. The
                    // cumulative weight is obtained by adding the disc's weight to the weight of
                    // all sub-towers, so:
                    // c = w + s
                    // cc = wc + s
                    //
                    // These can be rearranged in terms of s:
                    // s = c - w
                    // s = cc - wc
                    //
                    // As these are both equal to s, they can be equated and rearranged in terms of
                    // wc:
                    // c - w = cc - wc
                    // wc = w + cc - c
                    //
                    // The correct weight wc is the solution to the puzzle, so calculate it using
                    // this equation and return it.
                    return (sub_tower.weight + correct_cumulative_weight)
                        - sub_tower.cumulative_weight;
                }

                // If code didn't return in above if statement, then this sub-tower has its own
                // sub-tower with a different weight than all its other sub-towers. As there is
                // exactly one incorrect weight, it must lie with that different sub-tower or one of
                // its sub-towers, so call this function recursively on it. Return the value from
                // the recursive function call to propagate the return value back up to the original
                // caller.
                //
                // As this own sub-tower's weight has been verified to be correct, it can be safely
                // subtracted from the correct cumulative weight so far to get the correct
                // cumulative weight of all sub-towers, and then divided by the number of sub-towers
                // to get the correct cumulative weight per sub-tower.
                return fix_wrong_weight(
                    (correct_cumulative_weight - sub_tower.weight) / sub_tower.sub_towers.len(),
                    sub_tower,
                );
            }
        }

        // If no value is returned from the above loop, then then no sub-tower was found to have an
        // erroneous weight. This can only happen if the input is malformed or this solver has a
        // logic error. Either way, no valid correct weight can be returned, so panic.
        panic!()
    }

    // Use the solution from part 1 to get the bottom program, from which the Disc tree can be
    // built.
    let bottom_program = solve_1(input).to_string();
    let bottom_disc = Disc::new(input, &bottom_program);

    // Find the correct cumulative weight for a sub-tower directly above the bottom program. This is
    // done by checking the first three sub-towers' cumulative weights for a duplicated element (it
    // is assumed that the base program has at least 3 sub-towers, as with only 2 it would be
    // ambiguous which sub-tower had the wrong weight and the puzzle might not have a single
    // solution).
    let cw0 = bottom_disc.sub_towers[0].cumulative_weight;
    let cw1 = bottom_disc.sub_towers[1].cumulative_weight;
    let cw2 = bottom_disc.sub_towers[2].cumulative_weight;

    let correct_cumulative_weight = if cw0 == cw1 || cw0 == cw2 {
        // If cw0 is equal to any other value, then it must be a correct cumulative weight (there is
        // only one incorrect weight).
        cw0
    } else {
        // If cw0 is not equal to two other values, then it must be the incorrect cumulative weight
        // (though not necessarily the incorrect weight as the incorrect cumulative weight can be
        // caused by any disc above it having an incorrect weight).
        cw1
    };

    let correct_weight = fix_wrong_weight(correct_cumulative_weight, bottom_disc);

    Solution::USize(correct_weight)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"
            ),
            Solution::String("tknk".to_string())
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"
            ),
            Solution::U8(60)
        );
    }
}
