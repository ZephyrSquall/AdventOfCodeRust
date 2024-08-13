use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 17,
    title: "No Such Thing as Too Much",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    solve(input, 150)
}

fn solve(input: &str, target_size: u32) -> Solution {
    let containers = input
        .lines()
        .map(|line| line.parse().expect("Line should contain a valid number"))
        .collect::<Vec<_>>();

    Solution::U32(get_fitting_combinations(&containers, target_size, 0, 0))
}

// Checks every possible combination of containers to see how many can exactly hold the required
// litres of eggnog. This function uses recursion; depth and running_size should both be 0 when
// initially calling it.
fn get_fitting_combinations(
    containers: &[u32],
    target_size: u32,
    // Depth tracks (by index) which container is being considered to be added or not added to the
    // running size.
    depth: usize,
    // The running size is the total capacity of all containers in the current combination.
    running_size: u32,
) -> u32 {
    if running_size == target_size {
        // Adding containers always increases the running size, so once the running size exactly
        // equals the target size, this exact combination of containers works and all further
        // combinations that can be made by adding any of the remaining containers won't work. Thus
        // 1 is returned without doing any further recursion.
        1
    } else if running_size > target_size || depth >= containers.len() {
        // If the running size exceeds the target size, then this combination of containers and all
        // further combinations that can be made by adding any of the remaining containers won't
        // work, so return 0. Alternatively, if the depth equals the length of the containers list,
        // then no further containers can be added and it still hasn't reached the target size, so
        // still return 0.
        0
    } else {
        // In any combination of containers, each available container is either used or not used.
        // Thus all possible combination of containers can be checked by recursively calling this
        // function twice at each step, adding the container's size in one function and adding
        // nothing to the other.
        let container_size = containers[depth];
        get_fitting_combinations(
            containers,
            target_size,
            depth + 1,
            running_size + container_size,
        ) + get_fitting_combinations(containers, target_size, depth + 1, running_size)
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
                25
            ),
            Solution::U8(4)
        );
    }
}
