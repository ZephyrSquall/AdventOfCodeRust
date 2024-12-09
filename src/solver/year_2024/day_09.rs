use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 9,
    title: "Disk Fragmenter",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    // Each element of the disk vector represents a block on the disk. Some(id) represents a file
    // with that id occupying that block, and None represents a block of free space.
    let mut disk = Vec::new();
    let mut id = 0;
    let mut is_file = true;

    // Read the disk from the puzzle input.
    for length in input.chars().map(|character| {
        character
            .to_digit(10)
            .expect("Input should only contain digits")
    }) {
        if is_file {
            for _ in 0..length {
                disk.push(Some(id));
            }
            // Only increment the file id if a file was actually added to the disk, to make sure no
            // file ids are skipped.
            id += 1;
        } else {
            for _ in 0..length {
                disk.push(None);
            }
        }

        is_file = !is_file;
    }

    // Remove free space by bringing in file blocks from the end of the disk one at a time.
    let mut index = 0;
    while index < disk.len() {
        if disk[index].is_none() {
            // Rust's swap_remove method is perfect for this scenario. It removes elements from a
            // vector efficiently when the order of the vector's elements isn't important by
            // swapping the element to be removed by the last element, to avoid needing to shift all
            // elements down by one index. In this case however, we do care about the order, but we
            // want to remove free space by bringing in the file in the block at the very end of the
            // into the free space, which is what swap_remove already does.
            disk.swap_remove(index);
        } else {
            // A None could have been swapped for another None at the end of the disk, so don't move
            // on to the next index until we're sure it's no longer a None (got a Some in the if
            // statement).
            index += 1;
        }
    }

    // Calculate and return the checksum of the file-compacted disk.
    let mut checksum = 0;
    for (position, id) in disk
        .iter()
        .map(|id_option| id_option.expect("All Nones have been removed from the disk"))
        .enumerate()
    {
        checksum += position * id;
    }

    Solution::USize(checksum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_1("2333133121414131402"), Solution::U16(1928));
    }
}
