use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 5,
    title: "Print Queue",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    #[derive(PartialEq)]
    struct OrderingRule {
        first_page: u32,
        second_page: u32,
    }
    let mut ordering_rules = Vec::new();
    // An iterator must be explicitly created so it can be used across two different for loops while
    // maintaining its position (the first for loop will break early when it encounters a blank
    // line).
    let mut line_iter = input.lines();

    // Get all the ordering rules.
    for line in line_iter.by_ref() {
        if line.is_empty() {
            break;
        }

        let mut page_iter = line.split('|');
        let first_page = page_iter
            .next()
            .expect("Line should have first value")
            .parse()
            .expect("First value should be a number");
        let second_page = page_iter
            .next()
            .expect("Line should have second value")
            .parse()
            .expect("Second value should be a number");

        ordering_rules.push(OrderingRule {
            first_page,
            second_page,
        });
    }

    let mut correctly_ordered_middle_page_sum = 0;

    // Check each update to see if it is correctly-ordered. If so, add its middle page number to the
    // overall sum. This loop is labelled so that when an update is found to break an ordering rule,
    // it can immediately continue to the next update.
    'updates: for line in line_iter {
        let mut checked_pages = Vec::new();

        // Check all page numbers in this update. Map page numbers to u32s before iterating.
        for page in line
            .split(',')
            .map(|page_str| page_str.parse().expect("All pages should be numbers"))
        {
            // Check if printing this page next breaks any ordering rules with already-checked
            // pages.
            for checked_page in &checked_pages {
                if ordering_rules.contains(&OrderingRule {
                    first_page: page,
                    second_page: *checked_page,
                }) {
                    // If so, this update is not-correctly ordered and won't be adding its middle
                    // page number to the overall sum, so immediately skip to the next update.
                    continue 'updates;
                }
            }

            // This page doesn't conflict with any pages that came before it, so add it to the list
            // of checked pages and move to the next page.
            checked_pages.push(page);
        }

        // All pages have been checked and none of them break an ordering rule, so get the middle
        // page number and add it to the overall sum. It is assumed all updates have an odd number
        // of pages so the middle page is well-defined.
        correctly_ordered_middle_page_sum += checked_pages[(checked_pages.len() - 1) / 2];
    }

    Solution::U32(correctly_ordered_middle_page_sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            ),
            Solution::U8(143)
        );
    }
}
