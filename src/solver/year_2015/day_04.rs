use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 4,
    title: "The Ideal Stocking Stuffer",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let mut index = 0;
    loop {
        let mut hash_input = input.to_string();
        hash_input.push_str(&index.to_string());
        let hash = md5(&hash_input);
        // Checking if the first 5 hexadecimal digits are 0 is equivalent to checking if the first
        // 20 binary digits are 0. This can be easily tested by right-shifting by 108 (128 - 20) to
        // truncate all but the upper 20 bits, and testing if the result is equal to 0.
        if hash >> 108 == 0 {
            break;
        }

        index += 1;
    }

    Solution::U32(index)
}

fn solve_2(input: &str) -> Solution {
    let mut index = 0;
    loop {
        let mut hash_input = input.to_string();
        hash_input.push_str(&index.to_string());
        let hash = md5(&hash_input);
        if hash >> 104 == 0 {
            break;
        }

        index += 1;
    }

    Solution::U32(index)
}

// This function is marked public so it can be reused for the Year 2016 Day 5 puzzle, which also
// requires calculating MD5 hashes.
//
// Note this isn't a full implementation of the MD5 algorithm, as it does not handle long input
// strings that must be broken down into more than one 512-bit blocks. Many single-character names
// are used to follow the names of each variable in the definition of the MD5 algorithm.
#[allow(clippy::many_single_char_names)]
pub fn md5(input: &str) -> u128 {
    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    const K: [u32; 64] = [
        0xd76a_a478,
        0xe8c7_b756,
        0x2420_70db,
        0xc1bd_ceee,
        0xf57c_0faf,
        0x4787_c62a,
        0xa830_4613,
        0xfd46_9501,
        0x6980_98d8,
        0x8b44_f7af,
        0xffff_5bb1,
        0x895c_d7be,
        0x6b90_1122,
        0xfd98_7193,
        0xa679_438e,
        0x49b4_0821,
        0xf61e_2562,
        0xc040_b340,
        0x265e_5a51,
        0xe9b6_c7aa,
        0xd62f_105d,
        0x0244_1453,
        0xd8a1_e681,
        0xe7d3_fbc8,
        0x21e1_cde6,
        0xc337_07d6,
        0xf4d5_0d87,
        0x455a_14ed,
        0xa9e3_e905,
        0xfcef_a3f8,
        0x676f_02d9,
        0x8d2a_4c8a,
        0xfffa_3942,
        0x8771_f681,
        0x6d9d_6122,
        0xfde5_380c,
        0xa4be_ea44,
        0x4bde_cfa9,
        0xf6bb_4b60,
        0xbebf_bc70,
        0x289b_7ec6,
        0xeaa1_27fa,
        0xd4ef_3085,
        0x0488_1d05,
        0xd9d4_d039,
        0xe6db_99e5,
        0x1fa2_7cf8,
        0xc4ac_5665,
        0xf429_2244,
        0x432a_ff97,
        0xab94_23a7,
        0xfc93_a039,
        0x655b_59c3,
        0x8f0c_cc92,
        0xffef_f47d,
        0x8584_5dd1,
        0x6fa8_7e4f,
        0xfe2c_e6e0,
        0xa301_4314,
        0x4e08_11a1,
        0xf753_7e82,
        0xbd3a_f235,
        0x2ad7_d2bb,
        0xeb86_d391,
    ];

    let mut bytes = input.as_bytes().to_owned();
    let input_size: u64 = (bytes.len() * 8) as u64;

    bytes.push(0b1000_0000);
    while bytes.len() < 56 {
        bytes.push(0b0000_0000);
    }

    let input_size_bytes = input_size.to_le_bytes();
    for input_size_byte in input_size_bytes {
        bytes.push(input_size_byte);
    }

    let mut words = Vec::with_capacity(16);
    for word_chunk in bytes.chunks_exact(4) {
        let word = u32::from_le_bytes(
            word_chunk
                .try_into()
                .expect("Each chunk should have 4 bytes"),
        );
        words.push(word);
    }

    let mut a0: u32 = 0x6745_2301;
    let mut b0: u32 = 0xefcd_ab89;
    let mut c0: u32 = 0x98ba_dcfe;
    let mut d0: u32 = 0x1032_5476;

    let mut a = a0;
    let mut b = b0;
    let mut c = c0;
    let mut d = d0;

    for i in 0..64 {
        let mut f: u32;
        let g: usize;

        if i < 16 {
            f = (b & c) | ((!b) & d);
            g = i;
        } else if i < 32 {
            f = (b & d) | (c & (!d));
            g = ((5 * i) + 1) % 16;
        } else if i < 48 {
            f = b ^ c ^ d;
            g = ((3 * i) + 5) % 16;
        } else {
            f = c ^ (b | (!d));
            g = (7 * i) % 16;
        }

        f = f.wrapping_add(a.wrapping_add(K[i].wrapping_add(words[g])));
        a = d;
        d = c;
        c = b;
        b = b.wrapping_add(f.rotate_left(S[i]));
    }

    a0 = a0.wrapping_add(a);
    b0 = b0.wrapping_add(b);
    c0 = c0.wrapping_add(c);
    d0 = d0.wrapping_add(d);

    let digest: u128 =
        (u128::from(d0) << 96) | (u128::from(c0) << 64) | (u128::from(b0) << 32) | u128::from(a0);

    digest.to_be()
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore = "Finding the correct hashes takes several seconds in debug mode"]
    #[test]
    fn example1_1() {
        assert_eq!(solve_1("abcdef"), Solution::U32(609_043));
    }
    #[ignore = "Finding the correct hashes takes several seconds in debug mode"]
    #[test]
    fn example1_2() {
        assert_eq!(solve_1("pqrstuv"), Solution::U32(1_048_970));
    }
}
