use crate::solver::{Solution, Solver};
use rustc_hash::FxHashMap;

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 1,
    title: "Historian Hysteria",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();

    for line in input.lines() {
        let mut list_value_iter = line.split_whitespace();

        let list_1_value = list_value_iter
            .next()
            .expect("Line should have first list value")
            .parse::<u32>()
            .expect("First list value should be a number");
        let list_2_value = list_value_iter
            .next()
            .expect("Line should have second list value")
            .parse::<u32>()
            .expect("Second list value should be a number");

        list_1.push(list_1_value);
        list_2.push(list_2_value);
    }

    list_1.sort_unstable();
    list_2.sort_unstable();

    let mut cumulative_difference = 0;
    // Use zip to create an iterator that returns a tuple of the nth element from each list.
    for (list_1_value, list_2_value) in list_1.into_iter().zip(list_2) {
        cumulative_difference += list_1_value.abs_diff(list_2_value);
    }

    Solution::U32(cumulative_difference)
}

fn solve_2(input: &str) -> Solution {
    let mut list_1 = Vec::new();
    let mut list_2_counts = FxHashMap::default();

    for line in input.lines() {
        let mut list_value_iter = line.split_whitespace();

        let list_1_value = list_value_iter
            .next()
            .expect("Line should have first list value")
            .parse::<u32>()
            .expect("First list value should be a number");
        let list_2_value = list_value_iter
            .next()
            .expect("Line should have second list value")
            .parse::<u32>()
            .expect("Second list value should be a number");

        list_1.push(list_1_value);
        // Attempts to fetch the count for the given list value and add 1 to it. If the count for
        // the given list value doesn't exist, instead insert the value 1 to start the count.
        list_2_counts
            .entry(list_2_value)
            .and_modify(|value: &mut u32| *value += 1)
            .or_insert(1);
    }

    let mut similarity_score = 0;
    for list_1_value in list_1 {
        // Get the count if it exists, otherwise default to 0. Then multiply by the list value and
        // add to the similarity score.
        similarity_score += list_2_counts.get(&list_1_value).unwrap_or(&0) * list_1_value;
    }

    Solution::U32(similarity_score)
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

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            Solution::U8(31)
        );
    }
}
