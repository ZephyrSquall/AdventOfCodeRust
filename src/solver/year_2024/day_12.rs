use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 12,
    title: "Garden Groups",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    // Find the perimeter and area of the region containing the farm_plant at (x, y). This also
    // mutates farm_visited to set every square in this region to true, so it isn't visited twice.
    fn find_region_perimeter_and_area(
        farm_plant: char,
        x: usize,
        y: usize,
        farm: &Vec<Vec<char>>,
        farm_visited: &mut Vec<Vec<bool>>,
        x_len: usize,
        y_len: usize,
    ) -> (u32, u32) {
        farm_visited[y][x] = true;

        let mut perimeter = 0;
        // Area starts at 1 because it always counts its own square.
        let mut area = 1;

        // If y == 0, this farm plant borders the top of the map. If farm[y-1][x] != farm_plant, the
        // plant in the next square belongs to a different region so there must be a border between
        // them. In either case, this increases the perimeter of this region.
        if y == 0 || farm[y - 1][x] != farm_plant {
            perimeter += 1;
        // !(a || b) is equivalent to !a && !b. Hence this else only triggers if y != 0 &&
        // farm[y-1][x] == farm_plant, which indicates that this square is part of the same region.
        // Add one additional clause to check if this square has been checked before, and if not,
        // recursively check it.
        } else if !farm_visited[y - 1][x] {
            farm_visited[y - 1][x] = true;
            let (additional_perimeter, additional_area) = find_region_perimeter_and_area(
                farm_plant,
                x,
                y - 1,
                farm,
                farm_visited,
                x_len,
                y_len,
            );
            perimeter += additional_perimeter;
            area += additional_area;
        }

        if x == 0 || farm[y][x - 1] != farm_plant {
            perimeter += 1;
        } else if !farm_visited[y][x - 1] {
            farm_visited[y][x - 1] = true;
            let (additional_perimeter, additional_area) = find_region_perimeter_and_area(
                farm_plant,
                x - 1,
                y,
                farm,
                farm_visited,
                x_len,
                y_len,
            );
            perimeter += additional_perimeter;
            area += additional_area;
        }

        if y + 1 == y_len || farm[y + 1][x] != farm_plant {
            perimeter += 1;
        } else if !farm_visited[y + 1][x] {
            farm_visited[y + 1][x] = true;
            let (additional_perimeter, additional_area) = find_region_perimeter_and_area(
                farm_plant,
                x,
                y + 1,
                farm,
                farm_visited,
                x_len,
                y_len,
            );
            perimeter += additional_perimeter;
            area += additional_area;
        }

        if x + 1 == x_len || farm[y][x + 1] != farm_plant {
            perimeter += 1;
        } else if !farm_visited[y][x + 1] {
            farm_visited[y][x + 1] = true;
            let (additional_perimeter, additional_area) = find_region_perimeter_and_area(
                farm_plant,
                x + 1,
                y,
                farm,
                farm_visited,
                x_len,
                y_len,
            );
            perimeter += additional_perimeter;
            area += additional_area;
        }

        (perimeter, area)
    }

    let (farm, mut farm_visited, x_len, y_len) = get_farm_and_farm_visited_and_lens(input);

    // Iterate over every character in the farm. For each one, check if it has been visited before.
    // If not, find the perimeter and area for this region, multiply them together to find the
    // fencing price, and add it to the total.
    let mut total_fencing_price = 0;
    for (y, farm_line) in farm.iter().enumerate() {
        for (x, farm_plant) in farm_line.iter().enumerate() {
            if !farm_visited[y][x] {
                let (perimeter, area) = find_region_perimeter_and_area(
                    *farm_plant,
                    x,
                    y,
                    &farm,
                    &mut farm_visited,
                    x_len,
                    y_len,
                );
                total_fencing_price += perimeter * area;
            }
        }
    }
    Solution::U32(total_fencing_price)
}

fn solve_2(input: &str) -> Solution {
    enum Direction {
        North,
        East,
        West,
        South,
    }

    // IgnoreDirection is used to keep track of sides that are already counted. When a square
    // containing a new side is found, other squares along that side are explored until that side
    // ends, and all squares along that were explored have their corresponding ignore direction set
    // to true.
    //
    // Clippy suggests refactoring a struct with many bools into either enums or a struct with
    // enums, but this feels inappropriate here as whether a side should be ignored is true or
    // false.
    #[allow(clippy::struct_excessive_bools)]
    #[derive(Clone)]
    struct IgnoreDirection {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
    }
    impl IgnoreDirection {
        fn ignore_in_direction(&mut self, direction: &Direction) {
            match direction {
                Direction::North => self.north = true,
                Direction::East => self.east = true,
                Direction::South => self.south = true,
                Direction::West => self.west = true,
            }
        }
    }

    // Find the sides and area of the region containing the farm_plant at (x, y), mutating
    // farm_visited and farm_ignore_directions as needed.
    //
    // While it is unfortunate that this function has so many arguments, simplifying it in a logical
    // way doesn't seem practical. In particular, it's impossible to group all global farm-related
    // arguments (farm, farm_visited, farm_ignore_directions, x_len, and y_len) into a single
    // struct. farm needs to be immutably borrowed in two places at once, while farm_visited and
    // farm_ignore_direction must be mutably borrowed. If they were combined into a single struct,
    // that struct would therefore need to be mutably borrowed while it is immutably borrowed, which
    // isn't allowed by Rust. As such, I chose to leave it as-is, since any other way of grouping
    // arguments wouldn't be very logical and feel like an arbitrary choice just to follow Clippy's
    // recommendation.
    #[allow(clippy::too_many_arguments)]
    fn find_region_sides_and_area(
        farm_plant: char,
        x: usize,
        y: usize,
        farm: &Vec<Vec<char>>,
        farm_visited: &mut Vec<Vec<bool>>,
        farm_ignore_directions: &mut Vec<Vec<IgnoreDirection>>,
        x_len: usize,
        y_len: usize,
    ) -> (u32, u32) {
        farm_visited[y][x] = true;
        let mut sides = 0;
        let mut area = 1;

        // Check north
        if y == 0 || farm[y - 1][x] != farm_plant {
            // Check if this is the first time this side has been encountered. If so, count it, then
            // set every square bordering it to ignore this side so it isn't counted again.
            if !farm_ignore_directions[y][x].north {
                sides += 1;
                farm_ignore_directions[y][x].north = true;
                ignore_side(
                    farm_plant,
                    x,
                    y,
                    farm,
                    farm_ignore_directions,
                    x_len,
                    y_len,
                    &Direction::West,
                    &Direction::North,
                );
                ignore_side(
                    farm_plant,
                    x,
                    y,
                    farm,
                    farm_ignore_directions,
                    x_len,
                    y_len,
                    &Direction::East,
                    &Direction::North,
                );
            }
        } else if !farm_visited[y - 1][x] {
            farm_visited[y - 1][x] = true;
            let (additional_sides, additional_area) = find_region_sides_and_area(
                farm_plant,
                x,
                y - 1,
                farm,
                farm_visited,
                farm_ignore_directions,
                x_len,
                y_len,
            );
            sides += additional_sides;
            area += additional_area;
        }

        // Check east
        if x + 1 == x_len || farm[y][x + 1] != farm_plant {
            if !farm_ignore_directions[y][x].east {
                sides += 1;
                farm_ignore_directions[y][x].east = true;
                ignore_side(
                    farm_plant,
                    x,
                    y,
                    farm,
                    farm_ignore_directions,
                    x_len,
                    y_len,
                    &Direction::North,
                    &Direction::East,
                );
                ignore_side(
                    farm_plant,
                    x,
                    y,
                    farm,
                    farm_ignore_directions,
                    x_len,
                    y_len,
                    &Direction::South,
                    &Direction::East,
                );
            }
        } else if !farm_visited[y][x + 1] {
            farm_visited[y][x + 1] = true;
            let (additional_sides, additional_area) = find_region_sides_and_area(
                farm_plant,
                x + 1,
                y,
                farm,
                farm_visited,
                farm_ignore_directions,
                x_len,
                y_len,
            );
            sides += additional_sides;
            area += additional_area;
        }

        // Check south
        if y + 1 == y_len || farm[y + 1][x] != farm_plant {
            if !farm_ignore_directions[y][x].south {
                sides += 1;
                farm_ignore_directions[y][x].south = true;
                ignore_side(
                    farm_plant,
                    x,
                    y,
                    farm,
                    farm_ignore_directions,
                    x_len,
                    y_len,
                    &Direction::West,
                    &Direction::South,
                );
                ignore_side(
                    farm_plant,
                    x,
                    y,
                    farm,
                    farm_ignore_directions,
                    x_len,
                    y_len,
                    &Direction::East,
                    &Direction::South,
                );
            }
        } else if !farm_visited[y + 1][x] {
            farm_visited[y + 1][x] = true;
            let (additional_sides, additional_area) = find_region_sides_and_area(
                farm_plant,
                x,
                y + 1,
                farm,
                farm_visited,
                farm_ignore_directions,
                x_len,
                y_len,
            );
            sides += additional_sides;
            area += additional_area;
        }

        // Check east
        if x == 0 || farm[y][x - 1] != farm_plant {
            if !farm_ignore_directions[y][x].west {
                sides += 1;
                farm_ignore_directions[y][x].west = true;
                ignore_side(
                    farm_plant,
                    x,
                    y,
                    farm,
                    farm_ignore_directions,
                    x_len,
                    y_len,
                    &Direction::North,
                    &Direction::West,
                );
                ignore_side(
                    farm_plant,
                    x,
                    y,
                    farm,
                    farm_ignore_directions,
                    x_len,
                    y_len,
                    &Direction::South,
                    &Direction::West,
                );
            }
        } else if !farm_visited[y][x - 1] {
            farm_visited[y][x - 1] = true;
            let (additional_sides, additional_area) = find_region_sides_and_area(
                farm_plant,
                x - 1,
                y,
                farm,
                farm_visited,
                farm_ignore_directions,
                x_len,
                y_len,
            );
            sides += additional_sides;
            area += additional_area;
        }

        (sides, area)
    }

    // This function traverses a side until it ends, telling every square it encounters to ignore
    // that side so it doesn't get double-counted. It needs a travel direction and an ignore
    // direction, which should be orthogonal to each other. The ignore direction tells it which side
    // of the square needs to be ignored, and the travel direction tells it which way to traverse
    // along that side. This function should be called twice when it is needed, with travel
    // directions in opposite directions, to ensure the entire side is traversed and marked to be
    // ignored.
    #[allow(clippy::too_many_arguments)]
    fn ignore_side(
        farm_plant: char,
        x: usize,
        y: usize,
        farm: &Vec<Vec<char>>,
        farm_ignore_directions: &mut Vec<Vec<IgnoreDirection>>,
        x_len: usize,
        y_len: usize,
        direction_to_travel: &Direction,
        direction_to_ignore: &Direction,
    ) {
        let next_x;
        let next_y;
        // Check that the region continues in the travel direction.
        match direction_to_travel {
            Direction::North => {
                if y == 0 || farm[y - 1][x] != farm_plant {
                    return;
                }
                next_x = x;
                next_y = y - 1;
            }
            Direction::East => {
                if x + 1 == x_len || farm[y][x + 1] != farm_plant {
                    return;
                }
                next_x = x + 1;
                next_y = y;
            }
            Direction::South => {
                if y + 1 == y_len || farm[y + 1][x] != farm_plant {
                    return;
                }
                next_x = x;
                next_y = y + 1;
            }
            Direction::West => {
                if x == 0 || farm[y][x - 1] != farm_plant {
                    return;
                }
                next_x = x - 1;
                next_y = y;
            }
        }

        // Check that there is still a border between regions or the edge of the map in the ignore
        // direction.
        match direction_to_ignore {
            Direction::North => {
                if next_y == 0 || farm[next_y - 1][next_x] != farm_plant {
                    farm_ignore_directions[next_y][next_x].ignore_in_direction(direction_to_ignore);
                    ignore_side(
                        farm_plant,
                        next_x,
                        next_y,
                        farm,
                        farm_ignore_directions,
                        x_len,
                        y_len,
                        direction_to_travel,
                        direction_to_ignore,
                    );
                }
            }
            Direction::East => {
                if next_x + 1 == x_len || farm[next_y][next_x + 1] != farm_plant {
                    farm_ignore_directions[next_y][next_x].ignore_in_direction(direction_to_ignore);
                    ignore_side(
                        farm_plant,
                        next_x,
                        next_y,
                        farm,
                        farm_ignore_directions,
                        x_len,
                        y_len,
                        direction_to_travel,
                        direction_to_ignore,
                    );
                }
            }
            Direction::South => {
                if next_y + 1 == y_len || farm[next_y + 1][next_x] != farm_plant {
                    farm_ignore_directions[next_y][next_x].ignore_in_direction(direction_to_ignore);
                    ignore_side(
                        farm_plant,
                        next_x,
                        next_y,
                        farm,
                        farm_ignore_directions,
                        x_len,
                        y_len,
                        direction_to_travel,
                        direction_to_ignore,
                    );
                }
            }
            Direction::West => {
                if next_x == 0 || farm[next_y][next_x - 1] != farm_plant {
                    farm_ignore_directions[next_y][next_x].ignore_in_direction(direction_to_ignore);
                    ignore_side(
                        farm_plant,
                        next_x,
                        next_y,
                        farm,
                        farm_ignore_directions,
                        x_len,
                        y_len,
                        direction_to_travel,
                        direction_to_ignore,
                    );
                }
            }
        }
    }

    // Get the farm and initialize the ignored directions to false as no sides have been found yet.
    let (farm, mut farm_visited, x_len, y_len) = get_farm_and_farm_visited_and_lens(input);
    let mut farm_ignore_directions = vec![
        vec![
            IgnoreDirection {
                north: false,
                east: false,
                south: false,
                west: false
            };
            x_len
        ];
        y_len
    ];

    // Iterate over every character in the farm. For each one, check if it has been visited before.
    // If not, find the perimeter and area for this region, multiply them together to find the
    // fencing price, and add it to the total.
    let mut total_fencing_price = 0;
    for (y, farm_line) in farm.iter().enumerate() {
        for (x, farm_plant) in farm_line.iter().enumerate() {
            if !farm_visited[y][x] {
                let (sides, area) = find_region_sides_and_area(
                    *farm_plant,
                    x,
                    y,
                    &farm,
                    &mut farm_visited,
                    &mut farm_ignore_directions,
                    x_len,
                    y_len,
                );
                total_fencing_price += sides * area;
            }
        }
    }
    Solution::U32(total_fencing_price)
}

fn get_farm_and_farm_visited_and_lens(
    input: &str,
) -> (Vec<Vec<char>>, Vec<Vec<bool>>, usize, usize) {
    let mut farm = Vec::new();
    for line in input.lines() {
        let mut farm_line = Vec::new();
        for character in line.chars() {
            farm_line.push(character);
        }
        farm.push(farm_line);
    }

    // Initialize a 2D array of bool the size of the grid, set to false to reflect that none of the
    // grid has been searched yet.
    let x_len = farm[0].len();
    let y_len = farm.len();
    let farm_visited = vec![vec![false; x_len]; y_len];

    (farm, farm_visited, x_len, y_len)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
AAAA
BBCD
BBCC
EEEC"
            ),
            Solution::U8(140)
        );
    }
    #[test]
    fn example1_2() {
        assert_eq!(
            solve_1(
                "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
            ),
            Solution::U16(772)
        );
    }
    #[test]
    fn example1_3() {
        assert_eq!(
            solve_1(
                "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            ),
            Solution::U16(1930)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
AAAA
BBCD
BBCC
EEEC"
            ),
            Solution::U8(80)
        );
    }
    #[test]
    fn example2_2() {
        assert_eq!(
            solve_2(
                "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
            ),
            Solution::U16(436)
        );
    }
    #[test]
    fn example2_3() {
        assert_eq!(
            solve_2(
                "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"
            ),
            Solution::U8(236)
        );
    }
    #[test]
    fn example2_4() {
        assert_eq!(
            solve_2(
                "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"
            ),
            Solution::U16(368)
        );
    }
    #[test]
    fn example2_5() {
        assert_eq!(
            solve_2(
                "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            ),
            Solution::U16(1206)
        );
    }
}
