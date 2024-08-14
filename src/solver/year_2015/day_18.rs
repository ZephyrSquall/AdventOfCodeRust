use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 18,
    title: "Like a GIF For Your Yard",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    solve(input, 100)
}

fn solve(input: &str, steps: u32) -> Solution {
    let mut lights = get_lights(input);

    for _ in 0..steps {
        step(&mut lights);
    }

    let mut count = 0;
    for light_row in lights {
        for light in light_row {
            if light {
                count += 1;
            }
        }
    }

    Solution::U32(count)
}

fn get_lights(input: &str) -> Vec<Vec<bool>> {
    let mut lights = Vec::with_capacity(100);

    for line in input.lines() {
        let mut light_row = Vec::with_capacity(100);
        for light in line.chars() {
            if light == '#' {
                light_row.push(true);
            } else {
                light_row.push(false);
            }
        }
        lights.push(light_row);
    }

    lights
}

fn step(lights: &mut [Vec<bool>]) {
    let x_len = lights[0].len();
    let y_len = lights.len();
    let mut adjacent_light_counts = vec![vec!(0; x_len); y_len];

    // Count how many neighbours are lit for each light.
    for y in 0..y_len {
        for x in 0..x_len {
            let adjacent_light_count = &mut adjacent_light_counts[y][x];

            // Do not check an index if it's less than 0 or greater than len - 1, as this is outside
            // the bounds of the array.
            if y > 0 {
                if x > 0 && lights[y - 1][x - 1] {
                    *adjacent_light_count += 1;
                }
                if lights[y - 1][x] {
                    *adjacent_light_count += 1;
                }
                if x < x_len - 1 && lights[y - 1][x + 1] {
                    *adjacent_light_count += 1;
                }
            }

            if x > 0 && lights[y][x - 1] {
                *adjacent_light_count += 1;
            }
            // Do not check lights[y][x], which is the light itself instead of an adjacent light.
            if x < x_len - 1 && lights[y][x + 1] {
                *adjacent_light_count += 1;
            }

            if y < y_len - 1 {
                if x > 0 && lights[y + 1][x - 1] {
                    *adjacent_light_count += 1;
                }
                if lights[y + 1][x] {
                    *adjacent_light_count += 1;
                }
                if x < x_len - 1 && lights[y + 1][x + 1] {
                    *adjacent_light_count += 1;
                }
            }
        }
    }

    // Turn lights on or off according to their current state and number of lit neighbors.
    for y in 0..y_len {
        for x in 0..x_len {
            let light = &mut lights[y][x];
            let adjacent_light_count = adjacent_light_counts[y][x];

            if *light {
                if adjacent_light_count != 2 && adjacent_light_count != 3 {
                    *light = false;
                }
            } else {
                if adjacent_light_count == 3 {
                    *light = true;
                }
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
.#.#.#
...##.
#....#
..#...
#.#..#
####..",
                4
            ),
            Solution::U8(4)
        );
    }
}
