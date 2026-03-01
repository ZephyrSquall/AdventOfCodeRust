use crate::solver::{AdventOfCode, Solution};
use std::collections::BTreeMap;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2025,
    day: 8,
    title: "Playground",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    solve(input, 1000)
}

fn solve(input: &str, pairs: usize) -> Solution {
    #[derive(Debug)]
    struct Position {
        x: i64,
        y: i64,
        z: i64,
    }
    impl Position {
        fn distance_square(&self, other: &Position) -> i64 {
            (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
        }
    }

    let positions = input
        .lines()
        .map(|line| {
            let mut num_str_iter = line.split(',');
            let x = num_str_iter
                .next()
                .expect("Line should have first number")
                .parse()
                .expect("First number should be valid");
            let y = num_str_iter
                .next()
                .expect("Line should have second number")
                .parse()
                .expect("Second number should be valid");
            let z = num_str_iter
                .next()
                .expect("Line should have third number")
                .parse()
                .expect("Third number should be valid");
            Position { x, y, z }
        })
        .collect::<Vec<_>>();

    // The Euclidian distance between two points is given by sqrt((x_1 - x_2)^2 + (y_1 - y_2)^2 +
    // (z_1 - z_2)^2). The sqrt operation is problematic because the result might not be a whole
    // number that can neatly fit into an integer and will therefore require a float, bringing in
    // floating point precision concerns. However, we don't care about the exact distance itself,
    // only the relative ordering of distances between each other. Taking the square root of two
    // different positive numbers won't change the relative ordering between those numbers.
    // Therefore, don't worry about taking the square root, simply use the square of the distances.

    // A BTreeMap automatically sorts its elements. To remember the junction box that the distance
    // square corresponds to, the map is keyed by the distance square and the value is a tuple of
    // the source and destination junction box indexes. The largest element can obtained by
    // `.last_key_value().expect("").0`
    let mut smallest_distance_squares = BTreeMap::new();
    let mut positions_iter = positions.iter().enumerate();

    while let Some((index, position)) = positions_iter.next() {
        let other_positions_iter = positions_iter.clone();

        for (other_index, other_position) in other_positions_iter {
            let distance_square = position.distance_square(other_position);

            if smallest_distance_squares.len() < pairs {
                smallest_distance_squares.insert(distance_square, (index, other_index));
            } else if distance_square < *smallest_distance_squares.last_key_value().expect("").0 {
                smallest_distance_squares.pop_last();
                smallest_distance_squares.insert(distance_square, (index, other_index));
            }
        }
    }

    // At this point, we essentially have a graph with nodes given by ids 0 to positions.len() - 1,
    // and edges given by the values in smallest_distance_squares. Use breadth-first-search on each
    // unvisited node to find all connected nodes from it, mark them as visited, and record the size
    // of the group.
    let mut circuit_sizes = Vec::with_capacity(positions.len());
    let mut is_visited = vec![false; positions.len()];
    let mut root_node = 0;

    loop {
        let mut circuit_size = 1;
        is_visited[root_node] = true;
        let mut queue = Vec::with_capacity(positions.len());
        // This isn't a queue as it's first-in-last-out, so this isn't technically breadth-first
        // search, but this doesn't matter as we simply want to visit all connected nodes once, no
        // matter the order.
        queue.push(root_node);

        while let Some(node) = queue.pop() {
            for edge in smallest_distance_squares.values() {
                if edge.0 == node && !is_visited[edge.1] {
                    is_visited[edge.1] = true;
                    queue.push(edge.1);
                    circuit_size += 1;
                } else if edge.1 == node && !is_visited[edge.0] {
                    is_visited[edge.0] = true;
                    queue.push(edge.0);
                    circuit_size += 1;
                }
            }
        }

        circuit_sizes.push(circuit_size);
        let next_root_node_offset = is_visited.iter().skip(root_node).position(|node| !node);
        if let Some(next_root_node_offset) = next_root_node_offset {
            root_node += next_root_node_offset;
        } else {
            break;
        }
    }

    // Sort by b.cmp(a) to get results in reversed order, so the three biggest circuits are at the
    // front.
    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));

    Solution::U32(circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve(
                "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
                10
            ),
            Solution::U8(40)
        );
    }
}
