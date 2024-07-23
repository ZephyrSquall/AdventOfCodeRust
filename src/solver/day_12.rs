use super::{Solution, Solver};
use rustc_hash::FxHashMap;

pub const SOLVER: Solver = Solver {
    day: 12,
    title: "Digital Plumber",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let pipes = get_pipes(input);

    Solution::USize(get_pipe_group(0, &pipes).len())
}

fn solve_2(input: &str) -> Solution {
    let pipes = get_pipes(input);

    let mut groups: u16 = 0;
    let mut found_pipes = Vec::new();

    // Iterate over every pipe. Check if it's been previously found as a member of a group (tracked
    // by found_pipes). If not, find all connected pipes and add them to found_pipes, then increment
    // group count.
    for starting_pipe in pipes.keys() {
        if !found_pipes.contains(starting_pipe) {
            found_pipes.append(&mut get_pipe_group(*starting_pipe, &pipes));
            groups += 1;
        }
    }

    Solution::U16(groups)
}

// Get hashmap of pipe IDs to all pipes its connected to.
fn get_pipes(input: &str) -> FxHashMap<u16, Vec<u16>> {
    let mut pipes: FxHashMap<u16, Vec<u16>> = FxHashMap::default();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let current_pipe = iter
            .next()
            .expect("Line shouldn't be empty")
            .parse()
            .expect("Error parsing number");

        // Consume the "<->".
        iter.next();

        let connected_pipes = iter
            .map(|pipe| {
                pipe.trim_end_matches(',')
                    .parse()
                    .expect("Error parsing number")
            })
            .collect();

        pipes.insert(current_pipe, connected_pipes);
    }

    pipes
}

// Get vector of all pipes connected to starting_pipe.
fn get_pipe_group(starting_pipe: u16, pipes: &FxHashMap<u16, Vec<u16>>) -> Vec<u16> {
    let mut pipe_stack = vec![starting_pipe];
    let mut found_pipes = vec![starting_pipe];

    while let Some(pipe) = pipe_stack.pop() {
        let connected_pipes = pipes.get(&pipe).expect("Pipe missing from hashmap");

        for connected_pipe in connected_pipes {
            if !found_pipes.contains(connected_pipe) {
                found_pipes.push(*connected_pipe);
                pipe_stack.push(*connected_pipe);
            }
        }
    }

    found_pipes
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"
            ),
            Solution::U8(6)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"
            ),
            Solution::U8(2)
        );
    }
}
