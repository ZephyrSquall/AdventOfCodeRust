use crate::solver::{Solution, Solver};

// Temporarily allow dead code, as this solver is disabled for now for taking several minutes to
// run. It will be re-enabled once a more optimized solution is found.
#[allow(dead_code)]
pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 18,
    title: "RAM Run",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    solve_1_with_grid(input, 70, 70, 1024)
}

fn solve_2(input: &str) -> Solution {
    solve_2_with_grid(input, 70, 70)
}

fn solve_1_with_grid(input: &str, width: usize, height: usize, fallen_bytes: usize) -> Solution {
    // true represents a safe byte, false represents a corrupted byte. Note that valid x values for
    // the bytes includes the width and valid y values for the bytes include the height, so the
    // lengths are actually width + 1 and height + 1.
    let mut bytes = vec![vec![true; width + 1]; height + 1];

    // For the indicated number of fallen bytes, corrupt the square it lands on.
    for line in input.lines().take(fallen_bytes) {
        let mut char_iter = line.split(',');
        let x = char_iter
            .next()
            .expect("Line should have first value")
            .parse::<usize>()
            .expect("First value should be a number");
        let y = char_iter
            .next()
            .expect("Line should have second value")
            .parse::<usize>()
            .expect("Second value should be a number");
        bytes[y][x] = false;
    }

    let shortest_path = a_star(&bytes).expect("A valid path should have been found");
    Solution::USize(shortest_path)
}

fn solve_2_with_grid(input: &str, width: usize, height: usize) -> Solution {
    let mut bytes = vec![vec![true; width + 1]; height + 1];

    // Corrupt squares one at a time. After each corruption, check if the A* algorithm can find a
    // path to the end. If not, return the coordinates of the byte that just fell.
    for line in input.lines() {
        let mut char_iter = line.split(',');
        let x = char_iter
            .next()
            .expect("Line should have first value")
            .parse::<usize>()
            .expect("First value should be a number");
        let y = char_iter
            .next()
            .expect("Line should have second value")
            .parse::<usize>()
            .expect("Second value should be a number");
        bytes[y][x] = false;

        if a_star(&bytes).is_none() {
            return Solution::Str(line);
        }
    }

    panic!("The path to the end should have been blocked");
}

fn a_star(bytes: &[Vec<bool>]) -> Option<usize> {
    #[derive(PartialEq)]
    struct Position {
        x: usize,
        y: usize,
    }

    let width = bytes[0].len() - 1;
    let height = bytes.len() - 1;

    // Use the A* algorithm to find the shortest path from (0, 0) to (width, height).
    let mut open_set = Vec::with_capacity(width * height);
    open_set.push(Position { x: 0, y: 0 });
    let mut byte_g = vec![vec![usize::MAX; width + 1]; height + 1];
    byte_g[0][0] = 0;
    let mut byte_f = vec![vec![usize::MAX; width + 1]; height + 1];
    // width + height is the distance from (0, 0), the start point, to (width, height), the end
    // point.
    byte_f[0][0] = width + height;

    // This solution would run faster if open_set was a priority queue, but such optimizations will
    // be made after seeing Part 2. For simplicity, just store the open set as a vector that is
    // sorted according to byte_f before each pop for now.
    while let Some(byte) = open_set.pop() {
        if byte.x == width && byte.y == height {
            return Some(byte_g[height][width]);
        }

        let mut neighbours = Vec::with_capacity(4);
        if byte.y > 0 && bytes[byte.y - 1][byte.x] {
            neighbours.push(Position {
                x: byte.x,
                y: byte.y - 1,
            });
        }
        if byte.x < width && bytes[byte.y][byte.x + 1] {
            neighbours.push(Position {
                x: byte.x + 1,
                y: byte.y,
            });
        }
        if byte.y < height && bytes[byte.y + 1][byte.x] {
            neighbours.push(Position {
                x: byte.x,
                y: byte.y + 1,
            });
        }
        if byte.x > 0 && bytes[byte.y][byte.x - 1] {
            neighbours.push(Position {
                x: byte.x - 1,
                y: byte.y,
            });
        }

        for neighbour in neighbours {
            let neighbour_tentative_g = byte_g[byte.y][byte.x] + 1;
            if neighbour_tentative_g < byte_g[neighbour.y][neighbour.x] {
                byte_g[neighbour.y][neighbour.x] = neighbour_tentative_g;
                // The h function is the distance to the bottom-left corner, which is at (width,
                // height). As this is always to the right and beneath any other point, this
                // distance can be calculated by width + height - x - y
                byte_f[neighbour.y][neighbour.x] =
                    neighbour_tentative_g + width + height - neighbour.x - neighbour.y;
                if !open_set.contains(&neighbour) {
                    open_set.push(neighbour);
                }
            }
        }

        // Sort the elements in open set by the byte_f score at their positions. Reverse the
        // comparison so the vector is sorted biggest to smallest, so the byte with the smallest
        // byte_f score is popped first.
        open_set.sort_unstable_by(|a, b| byte_f[a.y][a.x].cmp(&byte_f[b.y][b.x]).reverse());
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1_with_grid(
                "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
                6,
                6,
                12
            ),
            Solution::U8(22)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2_with_grid(
                "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
                6,
                6,
            ),
            Solution::Str("6,1")
        );
    }
}
