use super::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    day: 5,
    title: "A Maze of Twisty Trampolines, All Alike",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    solve(input, false)
}

fn solve_2(input: &str) -> Solution {
    solve(input, true)
}

fn solve(input: &str, strange_jumps: bool) -> Solution {
    let mut jumps = Vec::new();
    for jump in input.lines() {
        let jump = jump.parse::<isize>().expect("Error parsing number");
        jumps.push(jump);
    }

    let mut step = 0;
    let mut position: isize = 0;

    loop {
        match jumps.get_mut(position.unsigned_abs()) {
            Some(jump) => {
                step += 1;
                position += *jump;

                if strange_jumps && *jump >= 3 {
                    *jump -= 1;
                } else {
                    *jump += 1;
                }
            }
            None => return Solution::U32(step),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
0
3
0
1
-3",
            ),
            Solution::U8(5)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
0
3
0
1
-3",
            ),
            Solution::U8(10)
        );
    }
}
