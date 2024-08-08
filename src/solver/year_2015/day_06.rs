use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 6,
    title: "Probably a Fire Hazard",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    // One million bools is too large to fit on the stack (testing shows it causes stack overflows),
    // so use Box to allocate on the heap instead.
    let mut lights = vec![[false; 1000]; 1000].into_boxed_slice();

    for line in input.lines() {
        let mut word_iter = line.split(' ');
        let end_coordinate = word_iter
            .next_back()
            .expect("Line should have a first word");
        // Ignore the "through"
        word_iter.next_back();
        let start_coordinate = word_iter
            .next_back()
            .expect("Line should have a third word");
        let instruction = word_iter
            .next_back()
            .expect("Line should have a fourth word");

        let (start_x, start_y) = get_coordinates(start_coordinate);
        let (end_x, end_y) = get_coordinates(end_coordinate);

        match instruction {
            "on" => {
                for y in start_y..=end_y {
                    for x in start_x..=end_x {
                        lights[y][x] = true;
                    }
                }
            }
            "off" => {
                for y in start_y..=end_y {
                    for x in start_x..=end_x {
                        lights[y][x] = false;
                    }
                }
            }
            "toggle" => {
                for y in start_y..=end_y {
                    for x in start_x..=end_x {
                        lights[y][x] = !lights[y][x];
                    }
                }
            }
            _ => panic!("Invalid instruction"),
        }
    }

    let mut count = 0;
    for light_row in lights.iter() {
        for light in light_row {
            if *light {
                count += 1;
            }
        }
    }

    Solution::U32(count)
}

fn get_coordinates(coordinate: &str) -> (usize, usize) {
    let mut coordinate_iter = coordinate.split(',');
    let x = coordinate_iter
        .next()
        .expect("Coordinate should have x value")
        .parse()
        .expect("Coordinate should be a valid number");
    let y = coordinate_iter
        .next()
        .expect("Coordinate should have x value")
        .parse()
        .expect("Coordinate should be a valid number");
    (x, y)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1("turn on 0,0 through 999,999"),
            Solution::U32(1_000_000)
        );
    }
    #[test]
    fn example1_2() {
        assert_eq!(solve_1("toggle 0,0 through 999,0"), Solution::U16(1000));
    }
    #[test]
    fn example1_3() {
        assert_eq!(solve_1("turn off 499,499 through 500,500"), Solution::U8(0));
    }
}
