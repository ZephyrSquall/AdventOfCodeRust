use crate::solver::{Solution, Solver};
use itertools::Itertools;

pub const SOLVER: Solver = Solver {
    year: 2024,
    day: 9,
    title: "Disk Fragmenter",
    part_solvers: &[solve_1, solve_2],
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
        .map(|id_option| id_option.expect("All Nones should have been removed from the disk"))
        .enumerate()
    {
        checksum += position * id;
    }

    Solution::USize(checksum)
}

fn solve_2(input: &str) -> Solution {
    // File represents a contiguous sequence of blocks with the same purpose. It can be either a
    // file with a given id (in which case the id Option will be Some(id)), or free space (in which
    // case the id Option will be None).
    #[derive(Debug)]
    struct File {
        id: Option<usize>,
        length: usize,
    }

    // disk is structured differently in this part. Instead of each element representing a single
    // data block, each element now represents a contiguous span of blocks taken up by a File
    // struct.
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
            if length > 0 {
                disk.push(File {
                    id: Some(id),
                    length: length as usize,
                });
            }
            // Only increment the file id if a file was actually added to the disk, to make sure no
            // file ids are skipped.
            id += 1;
        } else {
            if length > 0 {
                disk.push(File {
                    id: None,
                    length: length as usize,
                });
            }
        }

        is_file = !is_file;
    }

    // Remove free space by bringing in entire files from the end of the disk to the earliest free
    // space large enough to contain it, one file at a time.

    // id is one above the maximum file id as it is always incremented after adding a file. Hence
    // the following for loop, which wants to iterate over all file ids, uses a .. range which
    // excludes the upper limit. Reverse the iterator over this range to iterate over files from
    // highest to lowest file id.
    for id in (0..id).rev() {
        // Find the file and its position. The file itself is needed to get its length.
        let (file_pos, file) = disk
            .iter()
            .find_position(|file| file.id == Some(id))
            .expect("Disk should have every file id");
        let file_len = file.length;

        // Can't used find_position to find the corresponding free space as it returns an immutable
        // reference to the found File (free space is also of type File). A mutable reference is
        // needed in case the free space's length needs to be reduced. So just get the position (if
        // it exists) of the first free space big enough and use this to mutably get the free space
        // later.
        if let Some(free_space_pos) = disk
            .iter()
            // use take() to only check free space in front of the file, not after it.
            .take(file_pos)
            .position(|free_space| free_space.id.is_none() && free_space.length >= file_len)
        {
            // Get the free space, shorten its length to be equal to the file length, and calculate
            // the length that was lost in this process.
            let free_space = &mut disk[free_space_pos];
            let free_space_len = free_space.length;
            free_space.length = file_len;
            let remainder_len = free_space_len - file_len;

            // Swap the free space with the file. As they have equal lengths now, no other files are
            // displaced by the swap.
            disk.swap(free_space_pos, file_pos);

            // If any length was lost when the free space's length was set to the file's length, add
            // additional empty space to account for the remaining free space that wasn't filled up
            // in the swap.
            if remainder_len > 0 {
                disk.insert(
                    free_space_pos + 1,
                    File {
                        id: None,
                        length: remainder_len,
                    },
                );
            }
            // Note that there is no need to combine consecutive free space files after the swap,
            // because free space can only be placed next to other free space where the file was
            // just swapped out from. All files are originally in descending order and checked in
            // descending order, so this region will never be checked again until the checksum is
            // calculated.
        }
    }

    // Calculate and return the checksum of the file-compacted disk.
    let mut checksum = 0;
    let mut block_position = 0;
    for file in disk {
        for _ in 0..file.length {
            // Nones (free spaces) still exist in the disk, so handle them by treating them as if
            // they were 0.
            checksum += block_position * file.id.unwrap_or(0);
            block_position += 1;
        }
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

    #[test]
    fn example2_1() {
        assert_eq!(solve_2("2333133121414131402"), Solution::U16(2858));
    }
}
