use crate::solver::{Solution, Solver};
use itertools::Itertools;
use rustc_hash::FxHashSet;

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 13,
    title: "Knights of the Dinner Table",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let (attendees, relationships) = get_attendees_and_relationships(input);
    let mut greatest_total_happiness = i32::MIN;

    // Check every possible permutation of attendees.
    for arrangement in attendees.iter().permutations(attendees.len()) {
        // Sum up the happiness from every pair of adjacent attendees.
        let mut total_happiness = 0;
        for (attendee_1, attendee_2) in arrangement.iter().circular_tuple_windows() {
            total_happiness += get_happiness(&relationships, attendee_1, attendee_2);
        }

        if total_happiness > greatest_total_happiness {
            greatest_total_happiness = total_happiness;
        }
    }

    Solution::I32(greatest_total_happiness)
}

struct Relationship<'a, 'b> {
    attendee_1: &'a str,
    attendee_2: &'b str,
    happiness: i32,
}

// Get the sum of the total happiness that each of the two given attendees feel towards each other.
fn get_happiness(relationships: &[Relationship], attendee_1: &str, attendee_2: &str) -> i32 {
    let mut happiness = 0;
    // By tracking whether the first match is found, the search can be immediately terminated upon
    // finding the second match, improving performance.
    let mut found_first_match = false;

    for relationship in relationships {
        if relationship.attendee_1 == attendee_1 && relationship.attendee_2 == attendee_2 {
            happiness += relationship.happiness;
            if found_first_match {
                return happiness;
            }
            found_first_match = true;
        }
        if relationship.attendee_1 == attendee_2 && relationship.attendee_2 == attendee_1 {
            happiness += relationship.happiness;
            if found_first_match {
                return happiness;
            }
            found_first_match = true;
        }
    }

    panic!("Relationships vector should contain all possible relationships");
}

fn get_attendees_and_relationships(input: &str) -> (FxHashSet<&str>, Vec<Relationship>) {
    let mut attendees = FxHashSet::default();
    let mut relationships = Vec::new();

    for line in input.lines() {
        // Remove the last character from each line (an unnecessary '.')
        let mut char_iter = line.chars();
        char_iter.next_back();
        let line = char_iter.as_str();

        let mut word_iter = line.split(' ');
        let attendee_1 = word_iter.next().expect("Line should have first word");

        // Ignore the "would"
        word_iter.next();

        let is_loss = word_iter.next().expect("Line should have third word") == "lose";
        let mut happiness = word_iter
            .next()
            .expect("Line should have fourth word")
            .parse()
            .expect("Happiness should be a number");
        if is_loss {
            happiness *= -1;
        }

        // Ignore the "happiness", "units", "by", "sitting", "next", and "to" (six words) by reading
        // from the end instead.
        let attendee_2 = word_iter.next_back().expect("Line should have last word");

        attendees.insert(attendee_1);
        attendees.insert(attendee_2);
        relationships.push(Relationship {
            attendee_1,
            attendee_2,
            happiness,
        });
    }

    (attendees, relationships)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."
            ),
            Solution::U16(330)
        );
    }
}
