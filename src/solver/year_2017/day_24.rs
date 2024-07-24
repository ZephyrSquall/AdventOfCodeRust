use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2017,
    day: 24,
    title: "Electromagnetic Moat",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    // A recursive function to calculate the strongest bridge by recursively searching all possible
    // bridges.
    fn find_strongest_bridge(components: &Vec<Component>, port_to_match: u16) -> u16 {
        // Helper function to take a component, all remaining components, and the next port size to
        // match on, and calculate the highest strength.
        fn get_strength(
            component: &Component,
            components: &[Component],
            port_to_match: u16,
        ) -> u16 {
            // The list of components must be cloned so that reducing the list here doesn't affect
            // the overall list or the list of other recursive branches.
            let mut reduced_components = components.to_owned();
            // Remove the component that was just attached to the bridge.
            reduced_components.swap_remove(
                reduced_components
                    .iter()
                    .position(|c| *c == *component)
                    .expect("Component should have come from components vector"),
            );

            // Get and return the strength by adding the numbers on both ports of the current
            // component, plus the maximum strength from all possible bridges from this point on.
            component.port0
                + component.port1
                + find_strongest_bridge(&reduced_components, port_to_match)
        }

        let mut max_strength = 0;

        // For each component, if it matches the current port, use the get_strength function to
        // recursively call itself and update the maximum strength.
        for component in components {
            if component.port0 == port_to_match {
                let strength = get_strength(component, components, component.port1);
                if strength > max_strength {
                    max_strength = strength;
                }
            // Using else if prevents ports with the same number on both sides from being searched
            // twice.
            } else if component.port1 == port_to_match {
                let strength = get_strength(component, components, component.port0);
                if strength > max_strength {
                    max_strength = strength;
                }
            }
        }

        max_strength
    }

    let components = get_components(input);

    Solution::U16(find_strongest_bridge(&components, 0))
}

fn solve_2(input: &str) -> Solution {
    // A recursive function to calculate the longest bridge, then the strongest bridge, by
    // recursively searching all possible bridges.
    fn find_longest_strongest_bridge(components: &Vec<Component>, port_to_match: u16) -> (u8, u16) {
        // Helper function to take a component, all remaining components, and the next port size to
        // match on, and calculate the highest length and strength.
        fn get_longest_strength(
            component: &Component,
            components: &[Component],
            port_to_match: u16,
        ) -> (u8, u16) {
            let mut reduced_components = components.to_owned();
            reduced_components.swap_remove(
                reduced_components
                    .iter()
                    .position(|c| *c == *component)
                    .expect("Component should have come from components vector"),
            );

            let (sub_bridge_length, sub_bridge_strength) =
                find_longest_strongest_bridge(&reduced_components, port_to_match);
            let length = sub_bridge_length + 1;
            let strength = sub_bridge_strength + component.port0 + component.port1;
            (length, strength)
        }

        let mut max_length = 0;
        let mut max_strength = 0;

        // Now get both the length and strength. Length takes priority, so if length is higher,
        // update strength too even if strength is lower.
        for component in components {
            if component.port0 == port_to_match {
                let (length, strength) =
                    get_longest_strength(component, components, component.port1);
                if length > max_length {
                    max_length = length;
                    max_strength = strength;
                } else if length == max_length && strength > max_strength {
                    max_strength = strength;
                }
            } else if component.port1 == port_to_match {
                let (length, strength) =
                    get_longest_strength(component, components, component.port0);
                if length > max_length {
                    max_length = length;
                    max_strength = strength;
                } else if length == max_length && strength > max_strength {
                    max_strength = strength;
                }
            }
        }

        (max_length, max_strength)
    }

    let components = get_components(input);

    let (_, max_strength) = find_longest_strongest_bridge(&components, 0);
    Solution::U16(max_strength)
}

#[derive(Clone, PartialEq)]
struct Component {
    port0: u16,
    port1: u16,
}

fn get_components(input: &str) -> Vec<Component> {
    let mut components = Vec::new();
    for line in input.lines() {
        let mut iter = line.split('/');
        let port0 = iter
            .next()
            .expect("Line should have first element")
            .parse()
            .expect("Element should be a number");
        let port1 = iter
            .next()
            .expect("Line should have second element")
            .parse()
            .expect("Element should be a number");
        components.push(Component { port0, port1 });
    }

    components
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10"
            ),
            Solution::U8(31)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10"
            ),
            Solution::U8(19)
        );
    }
}
