use crate::solver::{Solution, Solver};
use std::collections::VecDeque;

pub const SOLVER: Solver = Solver {
    year: 2017,
    day: 16,
    title: "Permutation Promenade",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let mut dancers = VecDeque::from([
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ]);

    dance(&mut dancers, input);

    Solution::String(dancers.iter().collect())
}

fn solve_2(input: &str) -> Solution {
    // The positions the dancers are in after each dance loops quickly. By identifying the loop
    // size, most dances can be skipped.
    let mut dancers = VecDeque::from([
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ]);
    let starting_positions = dancers.clone();

    // Do one initial dance so the while loop condition isn't initially unsatisfied.
    dance(&mut dancers, input);
    let mut dance_loop_size = 1;

    while dancers != starting_positions {
        dance(&mut dancers, input);
        dance_loop_size += 1;
    }

    let unique_dances = 1_000_000_000 % dance_loop_size;
    for _ in 0..unique_dances {
        dance(&mut dancers, input);
    }

    Solution::String(dancers.iter().collect())
}

fn dance(dancers: &mut VecDeque<char>, input: &str) {
    for dance_move in input.split(',') {
        let mut chars = dance_move.chars();

        // Get the first character and perform the corresponding dance move.
        match chars.next() {
            Some('s') => {
                // The first character has already been consumed, so collecting the chars iterator
                // now simply gets the spin size as a string which can be parsed.
                let spin_size = chars
                    .collect::<String>()
                    .parse()
                    .expect("Spin size should be valid integer");
                dancers.rotate_right(spin_size);
            }
            Some('x') => {
                // Collecting the chars iterator gets a string of the two position integers
                // separated by a forward slash, which can be split by that forward slash and then
                // parsed.
                let exchange_parameters = chars.collect::<String>();
                let mut exchange_parameters = exchange_parameters.split('/');
                let first_position = exchange_parameters
                    .next()
                    .expect("Exchange parameters should have first value")
                    .parse()
                    .expect("First exchange position should be valid integer");
                let second_position = exchange_parameters
                    .next()
                    .expect("Exchange parameters should have second value")
                    .parse()
                    .expect("Second exchange position should be valid integer");
                dancers.swap(first_position, second_position);
            }
            Some('p') => {
                // As the arguments for a partner move are themselves characters, the chars iterator
                // can simply be advanced to get the dancer names.
                let first_dancer = chars
                    .next()
                    .expect("Partner parameters should have first value");
                // Ignore the '/'
                chars.next();
                let second_dancer = chars
                    .next()
                    .expect("Partner parameters should have second value");
                let first_position = dancers
                    .iter()
                    .position(|&d| d == first_dancer)
                    .expect("Dancer should be present");
                let second_position = dancers
                    .iter()
                    .position(|&d| d == second_dancer)
                    .expect("Dancer should be present");
                dancers.swap(first_position, second_position);
            }
            _ => panic!("Dance move has invalid instruction"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        let mut dancers = VecDeque::from(['a', 'b', 'c', 'd', 'e']);
        dance(&mut dancers, "s1,x3/4,pe/b");
        assert_eq!(dancers.iter().collect::<String>(), "baedc");
    }

    #[test]
    fn example2_1() {
        let mut dancers = VecDeque::from(['a', 'b', 'c', 'd', 'e']);
        dance(&mut dancers, "s1,x3/4,pe/b");
        dance(&mut dancers, "s1,x3/4,pe/b");
        assert_eq!(dancers.iter().collect::<String>(), "ceadb");
    }
}
