use crate::solver::{Solution, Solver};
use rustc_hash::FxHashSet;

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 14,
    title: "Restroom Redoubt",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    solve_1_for_width_and_height(input, 101, 103)
}

fn solve_1_for_width_and_height(input: &str, width: i32, height: i32) -> Solution {
    let mut robots = get_robots(input);

    // Simulate the robots for 100 seconds.
    for _ in 0..100 {
        for robot in &mut robots {
            robot.walk(width, height);
        }
    }

    // Assume the width and height are odd numbers (which is always the case since the only widths
    // and heights used are 101, 103, 11, and 7). Since indexes start at 0, the highest index is one
    // less than the width or the height, so subtract 1 to get the highest index, then divide it by
    // 2 to get the middle index.
    let middle_x = (width - 1) / 2;
    let middle_y = (height - 1) / 2;

    // Count the robots in each quadrant.
    let mut top_right_robots = 0;
    let mut bottom_right_robots = 0;
    let mut bottom_left_robots = 0;
    let mut top_left_robots = 0;
    for robot in &robots {
        match robot.get_quadrant(middle_x, middle_y) {
            Some(Quadrant::TopRight) => top_right_robots += 1,
            Some(Quadrant::BottomRight) => bottom_right_robots += 1,
            Some(Quadrant::BottomLeft) => bottom_left_robots += 1,
            Some(Quadrant::TopLeft) => top_left_robots += 1,
            None => {}
        }
    }

    // Calculate and return the safety factor.
    Solution::U32(top_right_robots * bottom_right_robots * bottom_left_robots * top_left_robots)
}

// "x" and "y" versions of variables are used often throughout this solver, and changing the names
// by more than this single letter will cause more confusion.
#[allow(clippy::similar_names)]
fn solve_2(input: &str) -> Solution {
    // As nothing is known about the target image except that it resembles a Christmas tree, making
    // an automated search for any specific patterns is impossible. Additionally, I have manually
    // looked at the first 2000 patterns and found nothing resembling a Christmas tree, so manually
    // inspecting all patterns until finding one with a Christmas tree is also impractical.
    //
    // However, if we assume the target image is distinctive enough to be recognizable at a glance,
    // and all other patterns place the robots in effectively random positions (which is reasonable
    // since many robots have velocity components greater than half the width or height of the
    // room), then the pattern with the Christmas tree must be significantly less "random" than all
    // other patterns. If the robots really were placed randomly throughout the room, with every
    // position having equal probability, then the distribution of the robots' positions' x and y
    // components should roughly follow a rectangular distribution.
    //
    // My technique is to find a "score" of how far away each pattern is from a perfectly
    // rectangular distribution of the x and y components of the robots' positions. The higher this
    // "score", the less "random" the pattern is. I assume the Christmas Tree pattern, being the
    // only "non-random" pattern, will have a significantly worse (higher) "score" than all other
    // patterns. I chose to use the "total diff", the sum of the differences of each of the actual
    // distributions from the perfect rectangular distributions, as my "score". Through
    // trial-and-error, I set a threshold for this "score" that should be between the Christmas Tree
    // pattern's score and the next-worse score. This solver checks all patterns in sequence until
    // it finds one whose "score" is above this threshold, and then returns the seconds it took to
    // reach that pattern.

    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;
    // This value is halfway between the total diff for the Christmas tree pattern (900) and the
    // greatest total diff among all non-Christmas tree patterns (640) for my puzzle input. By
    // choosing this value, I hope this makes this solver work for all puzzle inputs, but this is
    // impossible to verify. If this solver gives the wrong answer for anyone else's puzzle input,
    // it's likely the solver's logic is still sound but this value needs to be changed to reflect
    // the new puzzle input. The commented-out print_robot_formation function may help in finding
    // the ideal value for other puzzle inputs.
    const TOLERATED_DIFF: u32 = 770;

    // This function was used to print a formation of robots to the console and verify that a
    // Christmas tree pattern has been obtained. As printing out the Christmas tree pattern is not
    // necessary to execute the final version of this solution, it is commented out, but left here
    // for reference.
    /*
    fn print_robot_formation(robots: &Vec<Robot>, width: i32, height: i32) {
        let mut positions = FxHashSet::default();
        for robot in robots {
            positions.insert(robot.position.clone());
        }

        for y in 0..height {
            for x in 0..width {
                if positions.contains(&Vector { x, y }) {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
    */

    let mut robots = get_robots(input);
    let mut seconds = 0;

    // Positions will always be nonnegative after they wrap around, so there is no possibility of
    // losing the sign (this refers to the casts on lines 97 and 98).
    #[allow(clippy::cast_sign_loss)]
    loop {
        // Get every unique robot position.
        let mut positions = FxHashSet::default();
        for robot in &robots {
            positions.insert(robot.position.clone());
        }

        // Get the distribution of robot positions among both the x and y axes, as well as the count
        // of all robot positions.
        let mut x_counts = vec![0; WIDTH as usize];
        let mut y_counts = vec![0; HEIGHT as usize];
        let mut total_count = 0;
        for position in positions {
            x_counts[position.x as usize] += 1;
            y_counts[position.y as usize] += 1;
            total_count += 1;
        }

        // Calculate the expected counts of a perfectly rectangular distribution (this will be
        // slightly off due to integer rounding, but this is okay because being off by less than 1
        // won't have a massive impact on the score).
        let expected_x_count = total_count / WIDTH;
        let expected_y_count = total_count / HEIGHT;

        // Get the sum of all differences between the perfectly rectangular distributions and the
        // actual distributions, and then sum them into the final total_diff score.
        let x_diffs = x_counts.iter().fold(0, |acc, x_count: &i32| {
            acc + x_count.abs_diff(expected_x_count)
        });
        let y_diffs = y_counts.iter().fold(0, |acc, y_count: &i32| {
            acc + y_count.abs_diff(expected_y_count)
        });
        let total_diff = x_diffs + y_diffs;

        // If the total diff is above the threshold, the Christmas tree pattern was found, so return
        // the seconds (iterations) that has occurred so far.
        if total_diff > TOLERATED_DIFF {
            return Solution::U32(seconds);
        }

        // The total_diff was too low, so this isn't the Christmas tree pattern. Move onto the next
        // pattern.
        for robot in &mut robots {
            robot.walk(WIDTH, HEIGHT);
        }
        seconds += 1;
    }
}

// A two-dimensional mathematical vector, not to be confused with Rust's Vec type.
#[derive(PartialEq, Eq, Hash, Clone)]
struct Vector {
    x: i32,
    y: i32,
}
impl Vector {
    fn wrap(&mut self, width: i32, height: i32) {
        // The % operator calculates the remainder, which is not desired here as we want negative
        // numbers to immediately wrap around to the maximum value. The rem_euclid function
        // calculates the modulus, which has the desired wrapping behaviour.
        self.x = self.x.rem_euclid(width);
        self.y = self.y.rem_euclid(height);
    }
}

enum Quadrant {
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}

struct Robot {
    position: Vector,
    velocity: Vector,
}
impl Robot {
    fn new(line: &str) -> Robot {
        let trimmed_line = line.trim_start_matches("p=");
        let mut coordinates_iter = trimmed_line.split(" v=");

        let mut first_coordinate_iter = coordinates_iter
            .next()
            .expect("Line should have first coordinate")
            .split(',');
        let position_x = first_coordinate_iter
            .next()
            .expect("First coordinate should have first value")
            .parse()
            .expect("First value should be a number");
        let position_y = first_coordinate_iter
            .next()
            .expect("First coordinate should have second value")
            .parse()
            .expect("Second value should be a number");

        let mut second_coordinate_iter = coordinates_iter
            .next()
            .expect("Line should have second coordinate")
            .split(',');
        let velocity_x = second_coordinate_iter
            .next()
            .expect("Second coordinate should have first value")
            .parse()
            .expect("First value should be a number");
        let velocity_y = second_coordinate_iter
            .next()
            .expect("Second coordinate should have second value")
            .parse()
            .expect("Second value should be a number");

        Robot {
            position: Vector {
                x: position_x,
                y: position_y,
            },
            velocity: Vector {
                x: velocity_x,
                y: velocity_y,
            },
        }
    }

    // Move for one second, teleporting at the edges of the room.
    fn walk(&mut self, width: i32, height: i32) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.wrap(width, height);
    }

    // Return the quadrant the robot is in, or None if the robot is not in a quadrant (exactly in
    // the middle horizontally and/or vertically).
    fn get_quadrant(&self, middle_x: i32, middle_y: i32) -> Option<Quadrant> {
        if self.position.x < middle_x {
            if self.position.y < middle_y {
                return Some(Quadrant::TopLeft);
            }
            if self.position.y > middle_y {
                return Some(Quadrant::BottomLeft);
            }
        }

        if self.position.x > middle_x {
            if self.position.y < middle_y {
                return Some(Quadrant::TopRight);
            }
            if self.position.y > middle_y {
                return Some(Quadrant::BottomRight);
            }
        }

        None
    }
}

fn get_robots(input: &str) -> Vec<Robot> {
    let mut robots = Vec::new();
    for line in input.lines() {
        robots.push(Robot::new(line));
    }

    robots
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1_for_width_and_height(
                "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
                11,
                7
            ),
            Solution::U8(12)
        );
    }
}
