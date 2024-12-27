use crate::solver::{Solution, AdventOfCode};
use std::cmp::Ordering;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2024,
    day: 5,
    title: "Print Queue",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let (ordering_rules, update_iter) = get_ordering_rules_and_update_iter(input);
    let mut correctly_ordered_middle_page_sum = 0;

    // Check each update to see if it is correctly-ordered. If so, add its middle page number to the
    // overall sum. This loop is labelled so that when an update is found to break an ordering rule,
    // it can immediately continue to the next update.
    'updates: for update in update_iter {
        let mut checked_pages = Vec::new();

        // Check all page numbers in this update. Map page numbers to u32s before iterating.
        for page in update
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

fn solve_2(input: &str) -> Solution {
    let (ordering_rules, update_iter) = get_ordering_rules_and_update_iter(input);
    let mut incorrectly_ordered_middle_page_sum = 0;

    // Check each update to see if it is incorrectly-ordered. If so, sort it according to ordering
    // rules, then add its middle page number to the overall sum.
    for update in update_iter {
        let mut checked_pages = Vec::new();
        let mut is_correctly_ordered = true;

        for page in update
            .split(',')
            .map(|page_str| page_str.parse().expect("All pages should be numbers"))
        {
            // Check if printing this page next breaks any ordering rules with already-checked
            // pages. This check can be skipped if this update is already known to be
            // incorrectly-ordered.
            if is_correctly_ordered {
                for checked_page in &checked_pages {
                    if ordering_rules.contains(&OrderingRule {
                        first_page: page,
                        second_page: *checked_page,
                    }) {
                        // If so, remember that it is incorrectly-ordered. The full list must still
                        // be built, as it needs to be sorted, so do not continue to the next update
                        // immediately.
                        is_correctly_ordered = false;
                    }
                }
            }

            checked_pages.push(page);
        }

        // If it is still correctly-sorted after checking all pages, then skip this update.
        if is_correctly_ordered {
            continue;
        }

        // All pages have been checked and at least one of them breaks an ordering rule. Sort the
        // pages so that they respect all ordering rules.
        checked_pages.sort_unstable_by(|page_a, page_b| {
            for ordering_rule in &ordering_rules {
                if *ordering_rule
                    == (OrderingRule {
                        first_page: *page_a,
                        second_page: *page_b,
                    })
                {
                    return Ordering::Less;
                } else if *ordering_rule
                    == (OrderingRule {
                        first_page: *page_b,
                        second_page: *page_a,
                    })
                {
                    return Ordering::Greater;
                }
            }
            panic!("An ordering rule wasn't found")
        });

        // Now that all pages of this update have been sorted into the correct order, get the middle
        // value and add it to the overall sum.
        incorrectly_ordered_middle_page_sum += checked_pages[(checked_pages.len() - 1) / 2];
    }

    Solution::U32(incorrectly_ordered_middle_page_sum)
}

#[derive(PartialEq)]
struct OrderingRule {
    first_page: u32,
    second_page: u32,
}

fn get_ordering_rules_and_update_iter(
    input: &str,
) -> (Vec<OrderingRule>, impl Iterator<Item = &str>) {
    let mut ordering_rules = Vec::new();
    // An iterator must be explicitly created so it can be used across two different for loops while
    // maintaining its position (the first for loop will break early when it encounters a blank
    // line).
    let mut line_iter = input.lines();

    // Get all the ordering rules. by_ref() must be used so the line_iter isn't moved and can
    // continue to be used after this for loop.
    for line in line_iter.by_ref() {
        // A blank line indicates where ordering rules stop.
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

    // After this point, line_iter only contains strings representing lists of pages in an update,
    // so it is referred to as update_iter in code that uses this return value.
    (ordering_rules, line_iter)
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

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
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
            Solution::U8(123)
        );
    }
}
