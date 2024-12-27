use crate::solver::{Solution, AdventOfCode};

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2017,
    day: 17,
    title: "Spinlock",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let steps = input
        .parse::<usize>()
        .expect("Input should be single number");
    let mut circular_buffer: Vec<u16> = Vec::with_capacity(2018);
    circular_buffer.push(0);
    let mut index = 0;

    for value in 1..=2017 {
        // Vec::insert() inserts a new value at the given index, which means the value already there
        // is pushed after it. We actually want to insert the new value after the value that is
        // already there, so add 1 to the new index to get the index to point to the position after
        // the next value. This also means the index will be pointing to the new value after
        // insertion, so no further updates to the index are needed before starting the next loop.
        index = (index + steps + 1) % circular_buffer.len();
        circular_buffer.insert(index, value);
    }

    // After the final insertion, index is pointing to the 2017 value, so move it forwards once to
    // get the index of the value after 2017.
    index = (index + 1) % circular_buffer.len();

    Solution::U16(circular_buffer[index])
}

fn solve_2(input: &str) -> Solution {
    // Due to the way the circular buffer is built, 0 will always remain the first value (any value
    // that would be inserted before it is instead inserted at the very end of the buffer). This
    // allows building the circular buffer to be skipped, as only the length of the buffer and the
    // value immediately after 0 are required.
    //
    // Note that due to the way Rust's Vec::insert() method pushes the value at the current index
    // forward, the very initial push ends up moving 0 to the end of the array. From here on, if a
    // value were to be inserted immediately after the 0, the index would wrap around to the front
    // of the array before inserting, thus the 0 will always remain at the end of the array. This
    // quirk doesn't affect the relative order of any element (the Rust vector matches the circular
    // buffer as defined in the puzzle after the elements are rotated by 1), it just means the value
    // after 0 is actually at index 0 because the buffer wraps around.
    let steps = input.parse::<u32>().expect("Input should be single number");
    let mut index = 0;
    let mut value_after_zero = 0;

    for value in 1..=50_000_000 {
        index = (index + steps + 1) % value;
        if index == 0 {
            value_after_zero = value;
        }
    }

    Solution::U32(value_after_zero)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("3"), Solution::U16(638));
    }
}
