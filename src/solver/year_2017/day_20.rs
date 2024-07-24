use crate::solver::{Solution, Solver};
use itertools::Itertools;

pub const SOLVER: Solver = Solver {
    year: 2017,
    day: 20,
    title: "Particle Swarm",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    // In the long term, position and velocity are irrelevant, as given enough time, eventually any
    // particle with a higher acceleration magnitude will end up with a higher velocity magnitude,
    // which in turn will eventually lead to a farther position from <0, 0, 0> (or any other finite
    // point). Hence in the long term, the particle that will stay closest to <0, 0, 0> is the
    // particle with the smallest acceleration magnitude. (Magnitude in this case is calculated
    // using Manhattan distance instead of the actual direct line distance through 3D space, i.e.
    // calculated by the sum of absolute values instead of the square root of the sum of squares.)

    // Helper method to take a line from the puzzle input and calculate the acceleration magnitude.
    fn get_acceleration_magnitude(line: &str) -> u32 {
        line.rsplit('<')
            .next()
            .expect("Line should have an acceleration vector")
            .trim_end_matches('>')
            .split(',')
            .fold(0, |acc, a| {
                acc + a
                    .parse::<i32>()
                    .expect("Acceleration vector components should be numbers")
                    .unsigned_abs()
            })
    }

    let mut min_acceleration_magnitude = u32::MAX;
    let mut min_particle_id = usize::MAX;

    for (particle_id, line) in input.lines().enumerate() {
        let acceleration_magnitude = get_acceleration_magnitude(line);
        if acceleration_magnitude < min_acceleration_magnitude {
            min_acceleration_magnitude = acceleration_magnitude;
            min_particle_id = particle_id;
        }
    }

    Solution::USize(min_particle_id)
}

fn solve_2(input: &str) -> Solution {
    // This vector refers to a Euclidean vector (a value with a magnitude and direction), not the
    // type of a collection in Rust named Vec. It's postfixed with "3d" to make this distinction
    // more explicit.
    #[derive(Clone, PartialEq)]
    struct Vector3d {
        x: i64,
        y: i64,
        z: i64,
    }
    struct Particle {
        position: Vector3d,
        velocity: Vector3d,
        acceleration: Vector3d,
    }
    impl Particle {
        fn new(line: &str) -> Particle {
            let mut line_iter = line.split(", ");
            let position = Particle::get_vector(line_iter.next());
            let velocity = Particle::get_vector(line_iter.next());
            let acceleration = Particle::get_vector(line_iter.next());

            Particle {
                position,
                velocity,
                acceleration,
            }
        }
        fn get_vector(line_split: Option<&str>) -> Vector3d {
            let mut component_iter = line_split
                .expect("Line should have three vectors")
                .rsplit('<')
                .next()
                .expect("Line should have an acceleration vector")
                .trim_end_matches('>')
                .split(',');
            let x = Particle::get_component(component_iter.next());
            let y = Particle::get_component(component_iter.next());
            let z = Particle::get_component(component_iter.next());

            Vector3d { x, y, z }
        }
        fn get_component(component_split: Option<&str>) -> i64 {
            component_split
                .expect("Vector should have three components")
                .parse()
                .expect("Component should be a number")
        }
    }

    let mut particles = Vec::new();
    for line in input.lines() {
        particles.push(Particle::new(line));
    }

    // I'm assuming that no further collisions occur after 100 ticks. I have run the simulation up
    // to 1 million ticks and found no further collisions occur after the 40th tick. To solve this
    // with absolute certainty would most likely require very complicated analysis around finding
    // the line that each particle follows in the long term, making a buffer around that line to
    // account for never perfectly following that line in a finite amount of time, and checking that
    // none of these buffers overlap. This feels like a level of complexity that is outside the
    // scope of an AoC puzzle.
    for _ in 0..100 {
        let mut collision_positions = Vec::with_capacity(particles.len());
        for (particle_0, particle_1) in particles.iter().tuple_combinations() {
            if particle_0.position == particle_1.position {
                // The position must be cloned instead of borrowed in case this particle is removed
                // before other particles that collided at this position are checked (a mutable
                // borrow of an immutable reference would occur in the retain statement if this were
                // not cloned).
                collision_positions.push(particle_0.position.clone());
            }
        }

        particles.retain(|particle| !collision_positions.contains(&particle.position));

        for particle in &mut particles {
            particle.velocity.x += particle.acceleration.x;
            particle.velocity.y += particle.acceleration.y;
            particle.velocity.z += particle.acceleration.z;
            particle.position.x += particle.velocity.x;
            particle.position.y += particle.velocity.y;
            particle.position.z += particle.velocity.z;
        }
    }

    Solution::USize(particles.len())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>"
            ),
            Solution::U8(0)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>"
            ),
            Solution::U8(1)
        );
    }
}
