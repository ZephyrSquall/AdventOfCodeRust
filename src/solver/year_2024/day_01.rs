use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 1,
    title: "Historian Hysteria",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();

    for line in input.lines() {
        let mut list_entry_iter = line.split_whitespace();

        let list_1_entry = list_entry_iter
            .next()
            .expect("Line should have first list entry")
            .parse::<u32>()
            .expect("First list entry should be a number");
        let list_2_entry = list_entry_iter
            .next()
            .expect("Line should have second list entry")
            .parse::<u32>()
            .expect("Second list entry should be a number");

        list_1.push(list_1_entry);
        list_2.push(list_2_entry);
    }

    list_1.sort_unstable();
    list_2.sort_unstable();

    let mut cumulative_difference = 0;
    // Use zip to create an iterator that returns a tuple of the nth element from each list.
    for (line_1_entry, line_2_entry) in list_1.into_iter().zip(list_2) {
        cumulative_difference += line_1_entry.abs_diff(line_2_entry);
    }

    Solution::U32(cumulative_difference)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            Solution::U8(11)
        );
    }
}
