use crate::solver::{Solution, AdventOfCode};
use std::cmp::min_by_key;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2015,
    day: 17,
    title: "No Such Thing as Too Much",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    solve(input, 150, false)
}

fn solve_2(input: &str) -> Solution {
    solve(input, 150, true)
}

fn solve(input: &str, target_litres: u32, is_min_containers: bool) -> Solution {
    let containers = input
        .lines()
        .map(|line| line.parse().expect("Line should contain a valid number"))
        .collect::<Vec<_>>();

    if is_min_containers {
        Solution::U32(get_fitting_min_combinations(&containers, target_litres, 0, 0, 0).count)
    } else {
        Solution::U32(get_fitting_combinations(&containers, target_litres, 0, 0))
    }
}

// Checks every possible combination of containers to see how many can exactly hold the required
// litres of eggnog. This function uses recursion; depth and running_litres should both be 0 when
// initially calling it.
fn get_fitting_combinations(
    containers: &[u32],
    target_litres: u32,
    // Depth tracks (by index) which container is being considered to be added or not added to the
    // running litres.
    depth: usize,
    // The running litres is the total capacity of all containers in the current combination.
    running_litres: u32,
) -> u32 {
    if running_litres == target_litres {
        // Adding containers always increases the running litres, so once the running litres exactly
        // equals the target litres, this exact combination of containers works and all further
        // combinations that can be made by adding any of the remaining containers won't work. Thus
        // 1 is returned without doing any further recursion.
        1
    } else if running_litres > target_litres || depth >= containers.len() {
        // If the running litres exceeds the target litres, then this combination of containers and all
        // further combinations that can be made by adding any of the remaining containers won't
        // work, so return 0. Alternatively, if the depth equals the length of the containers list,
        // then no further containers can be added and it still hasn't reached the target litres, so
        // still return 0.
        0
    } else {
        // In any combination of containers, each available container is either used or not used.
        // Thus all possible combination of containers can be checked by recursively calling this
        // function twice at each step, adding the container's litres in one function and adding
        // nothing to the other.
        let container_litres = containers[depth];
        get_fitting_combinations(
            containers,
            target_litres,
            depth + 1,
            running_litres + container_litres,
        ) + get_fitting_combinations(containers, target_litres, depth + 1, running_litres)
    }
}

struct Combination {
    // How many valid combinations of this size have been found.
    count: u32,
    // The size (number of containers) in these combinations.
    size: u32,
}

// Checks every possible combination of containers to determine the fewest number of containers that
// can be used to exactly hold the required litres of eggnog, then get the number of combinations of
// that size that exactly hold the required litres of eggnog. This function uses recursion; depth
// and running_litres should both be 0 when initially calling it.
fn get_fitting_min_combinations(
    containers: &[u32],
    target_litres: u32,
    depth: usize,
    size: u32,
    running_litres: u32,
) -> Combination {
    if running_litres == target_litres {
        Combination { count: 1, size }
    } else if running_litres > target_litres || depth >= containers.len() {
        // Simply returning a count of 0 is no longer sufficient, as now we also need to make sure
        // the size resulting from this invalid combination doesn't get used for anything (otherwise
        // the minimum size may be set lower than what it actually is). By setting the size to the
        // maximum value, this ensures this result has a greater size than any valid combination.
        Combination {
            count: 0,
            size: u32::MAX,
        }
    } else {
        let container_litres = containers[depth];

        let combinations_if_included = get_fitting_min_combinations(
            containers,
            target_litres,
            depth + 1,
            size + 1,
            running_litres + container_litres,
        );
        let combinations_if_excluded = get_fitting_min_combinations(
            containers,
            target_litres,
            depth + 1,
            size,
            running_litres,
        );

        if combinations_if_included.size == combinations_if_excluded.size {
            // If both combinations have the same size, then combine their counts.
            Combination {
                count: combinations_if_included.count + combinations_if_excluded.count,
                size: combinations_if_included.size,
            }
        } else {
            // Otherwise, keep only the combination with the smallest count.
            min_by_key(
                combinations_if_included,
                combinations_if_excluded,
                |combination| combination.size,
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve(
                "\
20
15
10
5
5",
                25,
                false
            ),
            Solution::U8(4)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve(
                "\
20
15
10
5
5",
                25,
                true
            ),
            Solution::U8(3)
        );
    }
}
