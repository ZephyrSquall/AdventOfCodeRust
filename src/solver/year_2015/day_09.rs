use rustc_hash::FxHashSet;

use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 9,
    title: "All in a Single Night",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    // Using a hash set for nodes ensures that it only contains each location once, even if the same
    // location is inserted multiple times.
    let mut nodes = FxHashSet::default();
    let mut edges = Vec::new();

    for line in input.lines() {
        let mut word_iter = line.split(' ');
        let location_1 = word_iter.next().expect("Line should have first word");
        // Ignore the "to"
        word_iter.next();
        let location_2 = word_iter.next().expect("Line should have third word");
        // Ignore the "="
        word_iter.next();
        let distance = word_iter
            .next()
            .expect("Line should have fifth word")
            .parse()
            .expect("Distance should be a number");

        nodes.insert(location_1);
        nodes.insert(location_2);
        edges.push(Edge {
            location_1,
            location_2,
            distance,
        });
    }

    let visited_nodes = FxHashSet::default();

    Solution::U32(find_shortest_path(&nodes, &edges, &visited_nodes, None))
}

struct Edge<'a, 'b> {
    location_1: &'a str,
    location_2: &'b str,
    distance: u32,
}

// Perform a depth-first search to find the shortest path.
fn find_shortest_path(
    nodes: &FxHashSet<&str>,
    edges: &Vec<Edge>,
    visited_nodes: &FxHashSet<&str>,
    last_node: Option<&str>,
) -> u32 {
    // If nodes and visited_nodes have the same length, then every node has been visited and there's
    // nowhere else to go, so return 0.
    if nodes.len() == visited_nodes.len() {
        return 0;
    }

    let mut smallest_total_distance = u32::MAX;

    // Loop over each node that has not yet been visited.
    for unvisited_node in nodes.difference(visited_nodes) {
        let distance = match last_node {
            // If this is not the first node, then find the edge that gives the distance between the
            // last node and this node.
            Some(last_node) => {
                let edge = edges
                    .iter()
                    .find(|edge| {
                        (edge.location_1 == last_node && edge.location_2 == *unvisited_node)
                            || (edge.location_1 == *unvisited_node && edge.location_2 == last_node)
                    })
                    .expect("Every pair of locations should have an edge connecting them");
                edge.distance
            }
            // If this is the first node, then the distance is zero because there was no prior
            // location so no traveling has occurred yet.
            None => 0,
        };

        // If the distance of the edge alone is already greater than the smallest total distance
        // found so far, then it's not possible for any path beyond this one to be a new shortest
        // path, so save time by immediately skipping to the next node.
        if distance > smallest_total_distance {
            continue;
        }

        // Recursively call this function and add the result to the distance from the last node to
        // this node to get the smallest total distance from all paths after this node. If it is
        // smaller than the smallest total distance found so far, update the smallest total
        // distance.
        let mut next_visited_nodes = visited_nodes.clone();
        next_visited_nodes.insert(unvisited_node);

        let total_distance =
            distance + find_shortest_path(nodes, edges, &next_visited_nodes, Some(unvisited_node));
        if total_distance < smallest_total_distance {
            smallest_total_distance = total_distance;
        }
    }

    smallest_total_distance
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"
            ),
            Solution::U16(605)
        );
    }
}
