use crate::solver::{Solution, AdventOfCode};
use itertools::Itertools;
use std::cmp::min;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2015,
    day: 24,
    title: "It Hangs in the Balance",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let (package_weights, package_weights_sum) = get_package_weights_and_sum(input);

    // Since the total weight never changes and the weight is divided evenly among the three groups,
    // the total weight of each group is always exactly a third of the total weight of all packages.
    // It is assumed the total weight of all packages is divisible by 3, as otherwise there would be
    // no way to divide the packages into groups of equal weight.
    let group_weight = package_weights_sum / 3;

    // To find the smallest possible first group, check all groups of the smallest possible size,
    // then check all groups of the next size up from that, until a valid group is found. A group is
    // valid if the sum of weights of all packages in that group equals group_weight, and there is
    // at least one way to divide up all remaining packages into two groups whose total weights are
    // also equal to group_weight. By starting with the smallest possible size and working up, the
    // first valid group found this way is guaranteed to have the minimum number of packages.
    let mut minimum_quantum_entanglement = u64::MAX;

    // First search all possible package combinations from the first group for any group whose total
    // weights equals group_weight, starting from the smallest group size (1). Subtract 2 from the
    // upper bound on size because the other two groups need at least one package.
    for first_group_size in 1..(package_weights.len() - 2) {
        let mut has_found_valid_group = false;
        'first_group: for first_group in package_weights.iter().combinations(first_group_size) {
            if first_group.iter().copied().sum::<u64>() == group_weight {
                // The total weight of this group equals group_size, so now check if all remaining
                // packages can be split into two groups of equal weight. This simply requires
                // finding any combination of packages whose total weight equals group_weight, since
                // once one such group is found, all remaining packages will also have a total
                // weight equalling group_weight.

                // Get all packages that are not in the first group.
                let mut remaining_package_weights = package_weights.clone();
                remaining_package_weights
                    .retain(|package_weight| !first_group.contains(&package_weight));

                // Repeat the search for a valid package combination.
                for second_group_size in 1..(remaining_package_weights.len() - 1) {
                    for second_group in remaining_package_weights
                        .iter()
                        .combinations(second_group_size)
                    {
                        if second_group.iter().copied().sum::<u64>() == group_weight {
                            // A second group with the correct weights has been found, so the first
                            // group is valid. Update the quantum entanglement.
                            minimum_quantum_entanglement = min(
                                minimum_quantum_entanglement,
                                first_group.iter().copied().product(),
                            );
                            has_found_valid_group = true;

                            // Once a single valid configuration of the second group is found, there
                            // is no need to search for any others as the quantum entanglement only
                            // depends on the first group, so skip ahead to searching for more valid
                            // first groups.
                            continue 'first_group;
                        }
                    }
                }
            }
        }

        // When about to move to the next group size, first check if any valid groups were found at
        // the current group size. If so, all configurations with the minimum number of presents in
        // the first group have already been considered, so there is no need to check any other
        // groups.
        if has_found_valid_group {
            break;
        }
    }

    Solution::U64(minimum_quantum_entanglement)
}

fn solve_2(input: &str) -> Solution {
    let (package_weights, package_weights_sum) = get_package_weights_and_sum(input);

    // This time it is assumed the sum of all package weights is divisible by 4.
    let group_weight = package_weights_sum / 4;

    let mut minimum_quantum_entanglement = u64::MAX;

    // The same strategy as before can be used, however the search now needs to go down to the third
    // group to be certain valid groups exist (once the first three valid groups are found, the
    // remaining packages must form the fourth valid group).
    for first_group_size in 1..(package_weights.len() - 3) {
        let mut has_found_valid_group = false;
        'first_group: for first_group in package_weights.iter().combinations(first_group_size) {
            if first_group.iter().copied().sum::<u64>() == group_weight {
                // First valid group found, check for a second valid group.

                let mut first_remaining_package_weights = package_weights.clone();
                first_remaining_package_weights
                    .retain(|package_weight| !first_group.contains(&package_weight));

                for second_group_size in 1..(first_remaining_package_weights.len() - 2) {
                    for second_group in first_remaining_package_weights
                        .iter()
                        .combinations(second_group_size)
                    {
                        if second_group.iter().copied().sum::<u64>() == group_weight {
                            // Second valid group found, check for a third valid group.

                            let mut second_remaining_package_weights =
                                first_remaining_package_weights.clone();
                            second_remaining_package_weights
                                .retain(|package_weight| !second_group.contains(&package_weight));

                            for third_group_size in 1..(second_remaining_package_weights.len() - 1)
                            {
                                for third_group in second_remaining_package_weights
                                    .iter()
                                    .combinations(third_group_size)
                                {
                                    if third_group.iter().copied().sum::<u64>() == group_weight {
                                        // Third valid group found, so this is overall a valid
                                        // configuration. Update minimum_quantum_entanglement
                                        // accordingly.
                                        minimum_quantum_entanglement = min(
                                            minimum_quantum_entanglement,
                                            first_group.iter().copied().product(),
                                        );
                                        has_found_valid_group = true;

                                        continue 'first_group;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if has_found_valid_group {
            break;
        }
    }

    Solution::U64(minimum_quantum_entanglement)
}

fn get_package_weights_and_sum(input: &str) -> (Vec<u64>, u64) {
    let mut package_weights = Vec::new();
    let mut package_weights_sum = 0;

    for line in input.lines() {
        let package_weight = line.parse().expect("Line should contain a single number");
        package_weights.push(package_weight);
        package_weights_sum += package_weight;
    }

    (package_weights, package_weights_sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
1
2
3
4
5
7
8
9
10
11"
            ),
            Solution::U8(99)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
1
2
3
4
5
7
8
9
10
11"
            ),
            Solution::U8(44)
        );
    }
}
