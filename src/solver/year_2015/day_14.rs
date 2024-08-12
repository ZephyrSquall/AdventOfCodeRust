use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 14,
    title: "Reindeer Olympics",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    solve(input, 2503, false)
}

fn solve_2(input: &str) -> Solution {
    solve(input, 2503, true)
}

fn solve(input: &str, race_time: u32, is_using_points: bool) -> Solution {
    let mut reindeers = get_reindeers(input);
    race(&mut reindeers, race_time);

    let winning_distance = reindeers
        .iter()
        .map(|reindeer| {
            if is_using_points {
                reindeer.points
            } else {
                reindeer.distance
            }
        })
        .max()
        .expect("Reindeers shouldn't be empty");
    Solution::U32(winning_distance)
}

// We don't care who the winning reindeer is, only the winner's distance/points, so reindeer names
// aren't required.
struct Reindeer {
    // speed, flight_time, and rest_time define the reindeer's attributes. They must be initialized
    // ahead of time, and don't change later.
    speed: u32,
    flight_time: u32,
    rest_time: u32,
    // distance, points, is_resting, and timer track the reindeer's state during the race. They are
    // automatically initialized to the same default values for all reindeer, and mutated during the
    // race.
    distance: u32,
    points: u32,
    is_resting: bool,
    timer: u32,
}
impl Reindeer {
    fn new(speed: u32, flight_time: u32, rest_time: u32) -> Self {
        Reindeer {
            speed,
            flight_time,
            rest_time,
            distance: 0,
            points: 0,
            is_resting: false,
            timer: 0,
        }
    }
}

fn get_reindeers(input: &str) -> Vec<Reindeer> {
    let mut reindeers = Vec::new();

    for line in input.lines() {
        let mut word_iter = line.split(' ');

        // Ignore the "seconds."
        word_iter.next_back();

        let rest_time = word_iter
            .next_back()
            .expect("Line should have second-last word")
            .parse()
            .expect("Rest time should be a number");

        // Ignore the "for", "rest", "must", "then", "but", and "seconds," (six words)
        word_iter.next_back();
        word_iter.next_back();
        word_iter.next_back();
        word_iter.next_back();
        word_iter.next_back();
        word_iter.next_back();

        let flight_time = word_iter
            .next_back()
            .expect("Line should have ninth-last word")
            .parse()
            .expect("Flight time should be a number");

        // Ignore the "for" and "km/s" (two words)
        word_iter.next_back();
        word_iter.next_back();

        let speed = word_iter
            .next_back()
            .expect("Line should have twelfth-last word")
            .parse()
            .expect("Flight time should be a number");

        reindeers.push(Reindeer::new(speed, flight_time, rest_time));
    }

    reindeers
}

// Run the race. Each reindeer's final distance and points are stored in their corresponding fields.
fn race(reindeers: &mut [Reindeer], race_time: u32) {
    for _ in 0..race_time {
        let mut max_distance = 0;
        for reindeer in reindeers.iter_mut() {
            reindeer.timer += 1;

            if reindeer.is_resting {
                if reindeer.timer == reindeer.rest_time {
                    reindeer.is_resting = false;
                    reindeer.timer = 0;
                }
            } else {
                reindeer.distance += reindeer.speed;
                if reindeer.timer == reindeer.flight_time {
                    reindeer.is_resting = true;
                    reindeer.timer = 0;
                }
            }

            if reindeer.distance > max_distance {
                max_distance = reindeer.distance;
            }
        }

        // After all reindeer have moved in the current time step, award points.
        for reindeer in reindeers.iter_mut() {
            if reindeer.distance == max_distance {
                reindeer.points += 1;
            }
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
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
                1000,
                false
            ),
            Solution::U16(1120)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve(
                "\
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
                1000,
                true
            ),
            Solution::U16(689)
        );
    }
}
