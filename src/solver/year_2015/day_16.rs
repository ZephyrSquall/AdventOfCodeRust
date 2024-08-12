use crate::solver::{Solution, Solver};

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 16,
    title: "Aunt Sue",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    let aunt_sues = get_aunt_sues(input);

    for (index, aunt_sue) in aunt_sues.iter().enumerate() {
        if is_real_aunt_sue(aunt_sue) {
            // It is assumed that exactly one Aunt Sue matches, so this function immediately returns
            // the first one that matches because there are no more matches to find.
            return Solution::USize(index + 1);
        }
    }

    panic!("No Aunt Sue matched");
}

// None represents a characteristic that isn't remembered.
#[derive(Default)]
struct AuntSue {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

const REAL_AUNT_SUE: AuntSue = AuntSue {
    children: Some(3),
    cats: Some(7),
    samoyeds: Some(2),
    pomeranians: Some(3),
    akitas: Some(0),
    vizslas: Some(0),
    goldfish: Some(5),
    trees: Some(3),
    cars: Some(2),
    perfumes: Some(1),
};

fn get_aunt_sues(input: &str) -> Vec<AuntSue> {
    let mut aunt_sues = Vec::with_capacity(500);

    for line in input.lines() {
        // Initialize all of Aunt Sue's characteristics to None, so characteristics with no matching
        // information remain None.
        let mut aunt_sue = AuntSue::default();
        let mut word_iter = line.split(' ');
        // Ignore the "Sue" and the following number (it's assumed all Aunt Sues are in order)
        word_iter.next();
        word_iter.next();

        while let Some(characteristic) = word_iter.next() {
            match characteristic {
                "children:" => aunt_sue.children = Some(parse_characteristic(word_iter.next())),
                "cats:" => aunt_sue.cats = Some(parse_characteristic(word_iter.next())),
                "samoyeds:" => aunt_sue.samoyeds = Some(parse_characteristic(word_iter.next())),
                "pomeranians:" => {
                    aunt_sue.pomeranians = Some(parse_characteristic(word_iter.next()));
                }
                "akitas:" => aunt_sue.akitas = Some(parse_characteristic(word_iter.next())),
                "vizslas:" => aunt_sue.vizslas = Some(parse_characteristic(word_iter.next())),
                "goldfish:" => aunt_sue.goldfish = Some(parse_characteristic(word_iter.next())),
                "trees:" => aunt_sue.trees = Some(parse_characteristic(word_iter.next())),
                "cars:" => aunt_sue.cars = Some(parse_characteristic(word_iter.next())),
                "perfumes:" => aunt_sue.perfumes = Some(parse_characteristic(word_iter.next())),
                _ => panic!("Not a valid characteristic"),
            }
        }

        aunt_sues.push(aunt_sue);
    }

    aunt_sues
}

fn parse_characteristic(value: Option<&str>) -> u32 {
    value
        .expect("Characteristic list should have even number of words")
        .trim_end_matches(',')
        .parse()
        .expect("Value should be a number")
}

fn is_real_aunt_sue(aunt_sue: &AuntSue) -> bool {
    (aunt_sue.children.is_none() || aunt_sue.children == REAL_AUNT_SUE.children)
        && (aunt_sue.cats.is_none() || aunt_sue.cats == REAL_AUNT_SUE.cats)
        && (aunt_sue.samoyeds.is_none() || aunt_sue.samoyeds == REAL_AUNT_SUE.samoyeds)
        && (aunt_sue.pomeranians.is_none() || aunt_sue.pomeranians == REAL_AUNT_SUE.pomeranians)
        && (aunt_sue.akitas.is_none() || aunt_sue.akitas == REAL_AUNT_SUE.akitas)
        && (aunt_sue.vizslas.is_none() || aunt_sue.vizslas == REAL_AUNT_SUE.vizslas)
        && (aunt_sue.goldfish.is_none() || aunt_sue.goldfish == REAL_AUNT_SUE.goldfish)
        && (aunt_sue.trees.is_none() || aunt_sue.trees == REAL_AUNT_SUE.trees)
        && (aunt_sue.cars.is_none() || aunt_sue.cars == REAL_AUNT_SUE.cars)
        && (aunt_sue.perfumes.is_none() || aunt_sue.perfumes == REAL_AUNT_SUE.perfumes)
}

// The puzzle description provides no examples for this puzzle.
