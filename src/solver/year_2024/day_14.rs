use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 14,
    title: "Restroom Redoubt",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    solve(input, 101, 103)
}

fn solve(input: &str, width: i32, height: i32) -> Solution {
    // A two-dimensional mathematical vector, not to be confused with Rust's Vec type.
    struct Vector {
        x: i32,
        y: i32,
    }
    impl Vector {
        fn wrap(&mut self, width: i32, height: i32) {
            // The % operator calculates the remainder, which is not desired here as we want
            // negative numbers to immediately wrap around to the maximum value. The rem_euclid
            // function calculates the modulus, which has the desired wrapping behaviour.
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

        // Return the quadrant the robot is in, or None if the robot is not in a quadrant (exactly
        // in the middle horizontally and/or vertically).
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

    // Get the robots.
    let mut robots = Vec::new();
    for line in input.lines() {
        robots.push(Robot::new(line));
    }

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve(
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
