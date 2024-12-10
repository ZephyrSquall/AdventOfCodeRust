use crate::solver::{Solution, Solver};
use rustc_hash::FxHashSet;

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 10,
    title: "Hoof It",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    // A recursive function that keeps searching positions directly adjacent to the current position
    // to check if their height is one greater. When a height of 9 is found, the position is
    // inserted into a hash set of found positions at that height (hash sets ensure no duplicates)
    fn find_hiking_trails(
        topographic_map: &[Vec<u32>],
        position: &Position,
        current_height: u32,
        found_nine_heights: &mut FxHashSet<Position>,
    ) {
        let mut next_positions = Vec::with_capacity(4);
        if position.x > 0 {
            next_positions.push(Position {
                x: position.x - 1,
                y: position.y,
            });
        }
        if position.y > 0 {
            next_positions.push(Position {
                x: position.x,
                y: position.y - 1,
            });
        }
        if position.x + 1 < topographic_map[0].len() {
            next_positions.push(Position {
                x: position.x + 1,
                y: position.y,
            });
        }
        if position.y + 1 < topographic_map.len() {
            next_positions.push(Position {
                x: position.x,
                y: position.y + 1,
            });
        }

        let next_height = current_height + 1;
        if next_height == 9 {
            for next_position in next_positions {
                if topographic_map[next_position.y][next_position.x] == 9 {
                    found_nine_heights.insert(next_position);
                }
            }
        } else {
            for next_position in next_positions {
                if topographic_map[next_position.y][next_position.x] == next_height {
                    find_hiking_trails(
                        topographic_map,
                        &next_position,
                        next_height,
                        found_nine_heights,
                    );
                }
            }
        }
    }

    // Search the topographic map for trailheads (heights of 0). When one is found, call
    // find_hiking_trails to get all 9-height positions reachable from it, and add the number of
    // them to the score.
    let topographic_map = get_topographic_map(input);
    let mut score = 0;
    for (y, topographic_map_line) in topographic_map.iter().enumerate() {
        for (x, height) in topographic_map_line.iter().enumerate() {
            if *height == 0 {
                let mut found_nine_heights = FxHashSet::default();
                find_hiking_trails(
                    &topographic_map,
                    &Position { x, y },
                    0,
                    &mut found_nine_heights,
                );
                score += found_nine_heights.len();
            }
        }
    }

    Solution::USize(score)
}

fn solve_2(input: &str) -> Solution {
    // A recursive function that keeps searching positions directly adjacent to the current position
    // to check if their height is one greater. When a height of 9 is found, the score is
    // incremented by 1.
    fn find_hiking_trails(
        topographic_map: &[Vec<u32>],
        position: &Position,
        current_height: u32,
    ) -> u32 {
        let mut next_positions = Vec::with_capacity(4);
        if position.x > 0 {
            next_positions.push(Position {
                x: position.x - 1,
                y: position.y,
            });
        }
        if position.y > 0 {
            next_positions.push(Position {
                x: position.x,
                y: position.y - 1,
            });
        }
        if position.x + 1 < topographic_map[0].len() {
            next_positions.push(Position {
                x: position.x + 1,
                y: position.y,
            });
        }
        if position.y + 1 < topographic_map.len() {
            next_positions.push(Position {
                x: position.x,
                y: position.y + 1,
            });
        }

        let mut score = 0;
        let next_height = current_height + 1;
        if next_height == 9 {
            for next_position in next_positions {
                if topographic_map[next_position.y][next_position.x] == 9 {
                    score += 1;
                }
            }
        } else {
            for next_position in next_positions {
                if topographic_map[next_position.y][next_position.x] == next_height {
                    score += find_hiking_trails(topographic_map, &next_position, next_height);
                }
            }
        }
        score
    }

    // Search the topographic map for trailheads (heights of 0). When one is found, call
    // find_hiking_trails to get a count of all distinct hiking trails from it, and add the number
    // of them to the score.
    let topographic_map = get_topographic_map(input);
    let mut score = 0;
    for (y, topographic_map_line) in topographic_map.iter().enumerate() {
        for (x, height) in topographic_map_line.iter().enumerate() {
            if *height == 0 {
                score += find_hiking_trails(&topographic_map, &Position { x, y }, 0);
            }
        }
    }

    Solution::U32(score)
}

#[derive(PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

fn get_topographic_map(input: &str) -> Vec<Vec<u32>> {
    let mut topographic_map = Vec::new();
    for line in input.lines() {
        let mut topographic_map_line = Vec::new();
        for digit in line.chars().map(|character| {
            character
                .to_digit(10)
                .expect("Every character should be a digit from 0 to 9")
        }) {
            topographic_map_line.push(digit);
        }
        topographic_map.push(topographic_map_line);
    }
    topographic_map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            ),
            Solution::U8(36)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            ),
            Solution::U8(81)
        );
    }
}
