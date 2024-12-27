use crate::solver::{Solution, AdventOfCode};
use rustc_hash::FxHashMap;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2017,
    day: 13,
    title: "Packet Scanners",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let (mut firewall, greatest_depth) = get_firewall(input);
    let mut total_severity = 0;

    for depth in 0..=greatest_depth {
        if advance_picosecond(&mut firewall).contains(&depth) {
            total_severity += u32::from(depth) * u32::from(firewall[&depth].range);
        }
    }

    Solution::U32(total_severity)
}

fn solve_2(input: &str) -> Solution {
    struct Packet {
        delay: u32,
        depth: u8,
    }

    let (mut firewall, greatest_depth) = get_firewall(input);
    let mut packets = Vec::new();
    let mut delay: u32 = 0;

    // Send a packet into a firewall every picosecond and track all uncaught packets. When a packet
    // is caught, remove it. The first packet to reach the end will have the smallest delay.
    loop {
        packets.push(Packet { delay, depth: 0 });

        let caught_depths = advance_picosecond(&mut firewall);

        // Remove caught packets.
        packets.retain(|packet| !caught_depths.contains(&packet.depth));

        for packet in &mut packets {
            if packet.depth == greatest_depth {
                return Solution::U32(packet.delay);
            }

            packet.depth += 1;
        }

        delay += 1;
    }
}

struct Layer {
    range: u8,
    position: u8,
    is_moving_down: bool,
}

fn get_firewall(input: &str) -> (FxHashMap<u8, Layer>, u8) {
    let mut firewall = FxHashMap::default();
    let mut greatest_depth = 0;

    for line in input.lines() {
        let mut iter = line.split(": ");

        let depth = iter
            .next()
            .expect("Line shouldn't be empty")
            .parse()
            .expect("Error parsing number");
        let range = iter
            .next()
            .expect("Line shouldn't be empty")
            .parse()
            .expect("Error parsing number");

        let layer = Layer {
            range,
            position: 0,
            is_moving_down: true,
        };
        firewall.insert(depth, layer);

        if depth > greatest_depth {
            greatest_depth = depth;
        }
    }

    (firewall, greatest_depth)
}

// Advances all scanners by one step. Returns a vector of all depths at which a packet would be
// caught.
fn advance_picosecond(firewall: &mut FxHashMap<u8, Layer>) -> Vec<u8> {
    let mut caught_depths = Vec::new();

    for (depth, layer) in firewall {
        // A packet is only caught if the scanner is at the top before moving, so check if caught
        // only before updating the scanner position.
        if layer.position == 0 {
            caught_depths.push(*depth);
        }

        if layer.is_moving_down {
            if layer.position + 1 == layer.range {
                layer.is_moving_down = false;
                layer.position -= 1;
            } else {
                layer.position += 1;
            }
        } else if layer.position == 0 {
            layer.is_moving_down = true;
            layer.position += 1;
        } else {
            layer.position -= 1;
        }
    }

    caught_depths
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
0: 3
1: 2
4: 4
6: 4"
            ),
            Solution::U8(24)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
0: 3
1: 2
4: 4
6: 4"
            ),
            Solution::U8(10)
        );
    }
}
