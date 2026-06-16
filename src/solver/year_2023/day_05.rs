use crate::solver::{AdventOfCode, Solution};
use itertools::Itertools;
use std::collections::VecDeque;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2023,
    day: 5,
    title: "If You Give A Seed A Fertilizer",
    part_solvers: &[solve_1, solve_2],
};

struct IdRange {
    start: u64,
    length: u64,
}
impl IdRange {
    fn new<'a>(mut seed_iter_chunk: impl Iterator<Item = &'a str>) -> IdRange {
        let start = seed_iter_chunk
            .next()
            .expect("Seed iterator chunk should have first element")
            .parse()
            .expect("First element should be a valid number");
        let length = seed_iter_chunk
            .next()
            .expect("Seed iterator chunk should have second element")
            .parse()
            .expect("Second element should be a valid number");
        IdRange { start, length }
    }
}

struct Map {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
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

    fn map_seed(&self, id: u64) -> Option<u64> {
        (id >= self.source_range_start && id < self.source_range_start + self.range_length).then(
            || {
                let offset = id - self.source_range_start;
                self.destination_range_start + offset
            },
        )
    }

    // Takes an input_id_range_queue. For each input_id_range, if it is within this map's source
    // range, it gets mapped and added to the output_id_range_queue. Otherwise, it is added to a new
    // unmapped_id_range_queue, which is returned at the end of this method. If an input_id_range
    // passes over the boundary of this map's source range, it is split into two or three id_ranges,
    // with the portion within this map's source range getting mapped and added to the
    // output_id_range_queue, and the other portions added to the unmapped_id_range_queue.
    fn map_id_range_queue(
        &self,
        input_id_range_queue: VecDeque<IdRange>,
        output_id_range_queue: &mut VecDeque<IdRange>,
    ) -> VecDeque<IdRange> {
        let mut unmapped_id_range_queue = VecDeque::new();

        for mut input_id_range in input_id_range_queue {
            // If the input_id_range lies completely outside of this map's source range (either
            // below or above), add the entire id_range to the unmapped_id_range_queue
            if input_id_range.start + input_id_range.length < self.source_range_start
                || self.source_range_start + self.range_length < input_id_range.start
            {
                unmapped_id_range_queue.push_back(input_id_range);
                continue;
            }

            // We can now assume there must be overlap, as the loop wouldn't have made it this far
            // otherwise. So check if any part of the input_id_range is outside this map's source
            // range; if so, create a new unmapped_id_range representing the excess portion of the
            // input_id_range outside of this map's source range, then shrink the input_id_range to
            // the boundaries of this map's source range.
            if input_id_range.start < self.source_range_start {
                let excess_range_length = self.source_range_start - input_id_range.start;

                unmapped_id_range_queue.push_back(IdRange {
                    start: input_id_range.start,
                    length: excess_range_length,
                });

                input_id_range.start = self.source_range_start;
                input_id_range.length -= excess_range_length;
            }

            if self.source_range_start + self.range_length
                < input_id_range.start + input_id_range.length
            {
                let excess_range_length = (input_id_range.start + input_id_range.length)
                    - (self.source_range_start + self.range_length);

                unmapped_id_range_queue.push_back(IdRange {
                    start: self.source_range_start + self.range_length,
                    length: excess_range_length,
                });

                // input_id_range.start is unchanged
                input_id_range.length -= excess_range_length;
            }

            // What's left of the input_id_range now definitely lies entirely inside this map's
            // source range, so map it.
            let offset = input_id_range.start - self.source_range_start;
            output_id_range_queue.push_back(IdRange {
                start: self.destination_range_start + offset,
                length: input_id_range.length,
            });
        }

        unmapped_id_range_queue
    }
}

fn get_map_layers<'a>(line_block_iter: impl Iterator<Item = &'a str>) -> Vec<Vec<Map>> {
    line_block_iter
        .map(
            // .skip(1) to skip the starting line that lists the name of the map.
            |map_block| map_block.lines().skip(1).map(Map::new).collect::<Vec<_>>(),
        )
        .collect::<Vec<_>>()
}

fn solve_1(input: &str) -> Solution {
    fn map_single_layer(map_layer: &[Map], id: u64) -> u64 {
        for map in map_layer {
            if let Some(mapped_id) = map.map_seed(id) {
                return mapped_id;
            }
        }

        // If none of the maps applied, the id maps to itself.
        id
    }

    fn map_all_layers(map_layers: &[Vec<Map>], mut id: u64) -> u64 {
        for map_layer in map_layers {
            id = map_single_layer(map_layer, id);
        }
        id
    }

    let mut line_block_iter = input.split("\n\n");

    let seeds_line = line_block_iter
        .next()
        .expect("line_iter should have first line");
    let seeds = seeds_line
        .strip_prefix("seeds: ")
        .expect("All lines should start with \"seeds: \"")
        .split(' ')
        .map(|seed_str| seed_str.parse().expect("seed_str should be a valid number"))
        .collect::<Vec<_>>();

    let map_layers = get_map_layers(line_block_iter);

    let lowest_location = seeds
        .into_iter()
        .map(|seed| map_all_layers(&map_layers, seed))
        .min()
        .expect("There should be at least one location");

    Solution::U64(lowest_location)
}

fn solve_2(input: &str) -> Solution {
    fn map_single_layer(
        map_layer: &[Map],
        mut id_range_queue: VecDeque<IdRange>,
    ) -> VecDeque<IdRange> {
        let mut mapped_id_range_queue = VecDeque::new();
        for map in map_layer {
            id_range_queue = map.map_id_range_queue(id_range_queue, &mut mapped_id_range_queue);
        }

        // Anything left in the id_range_queue had no maps that applied to it, so they map to
        // themselves.
        mapped_id_range_queue.append(&mut id_range_queue);

        mapped_id_range_queue
    }

    fn map_all_layers(
        map_layers: &[Vec<Map>],
        mut id_range_queue: VecDeque<IdRange>,
    ) -> VecDeque<IdRange> {
        for map_layer in map_layers {
            id_range_queue = map_single_layer(map_layer, id_range_queue);
        }
        id_range_queue
    }

    let mut line_block_iter = input.split("\n\n");

    let seeds_line = line_block_iter
        .next()
        .expect("line_iter should have first line");
    let seed_range_queue = seeds_line
        .strip_prefix("seeds: ")
        .expect("All lines should start with \"seeds: \"")
        .split(' ')
        .chunks(2)
        .into_iter()
        .map(IdRange::new)
        .collect::<VecDeque<_>>();

    let map_layers = get_map_layers(line_block_iter);

    let location_range_queue = map_all_layers(&map_layers, seed_range_queue);

    let lowest_location = location_range_queue
        .into_iter()
        .map(|location_range| location_range.start)
        .min()
        .expect("There should be at least one location range");

    Solution::U64(lowest_location)
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

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
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
            Solution::U8(46)
        );
    }
}
