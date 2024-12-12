use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 12,
    title: "Garden Groups",
    part_solvers: &[solve_1],
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

    // Read the input.
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
    let mut farm_visited = vec![vec![false; x_len]; y_len];

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
}
