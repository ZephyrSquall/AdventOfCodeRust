use crate::solver::{AdventOfCode, Solution};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2023,
    day: 5,
    title: "If You Give A Seed A Fertilizer",
    part_solvers: &[solve_1],
};

struct Map {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32,
}
impl Map {
    fn new(map_line: &str) -> Map {
        let mut map_iter = map_line.split(' ').map(|map_value_str| {
            map_value_str
                .parse()
                .expect("Map value should be valid number")
        });
        let destination_range_start = map_iter
            .next()
            .expect("Map iterator should have first value");
        let source_range_start = map_iter
            .next()
            .expect("Map iterator should have second value");
        let range_length = map_iter
            .next()
            .expect("Map iterator should have third value");

        Map {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }

    fn map(&self, id: u32) -> Option<u32> {
        (id >= self.source_range_start && id < self.source_range_start + self.range_length).then(
            || {
                let offset = id - self.source_range_start;
                self.destination_range_start + offset
            },
        )
    }
}

fn map_single_layer(map_layer: &[Map], id: u32) -> u32 {
    for map in map_layer {
        if let Some(mapped_id) = map.map(id) {
            return mapped_id;
        }
    }

    // If none of the maps applied, the id maps to itself.
    id
}

fn map_all_layers(map_layers: &[Vec<Map>], mut id: u32) -> u32 {
    for map_layer in map_layers {
        id = map_single_layer(map_layer, id);
    }
    id
}

fn solve_1(input: &str) -> Solution {
    let mut line_block_iter = input.split("\n\n");

    let seeds_line = line_block_iter
        .next()
        .expect("line_iter should have first line");
    let seeds = seeds_line
        .strip_prefix("seeds: ")
        .expect("All lines should start with \"seeds: \"")
        .split(' ')
        .map(|seed_str| seed_str.parse().expect("seed_str should be a valid number"))
        .collect::<Vec<u32>>();

    let map_layers = line_block_iter
        .map(
            // .skip(1) to skip the starting line that lists the name of the map.
            |map_block| map_block.lines().skip(1).map(Map::new).collect::<Vec<_>>(),
        )
        .collect::<Vec<_>>();

    let lowest_location = seeds
        .into_iter()
        .map(|seed| map_all_layers(&map_layers, seed))
        .min()
        .expect("There should be at least one seed");

    Solution::U32(lowest_location)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            Solution::U8(35)
        );
    }
}
