use crate::solver::{AdventOfCode, Solution};
use std::cmp::Ordering;

pub const SOLVER: AdventOfCode = AdventOfCode {
    year: 2023,
    day: 7,
    title: "Camel Cards",
    part_solvers: &[solve_1],
};

// The variants in this enum are specifically ordered from weakest to strongest so that deriving Ord
// automatically assigns the correct ordering.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    // Assign each variant a specific index, so an array of 13 elements can be used to count
    // occurrences of each specific card in the hand.
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    T = 8,
    J = 9,
    Q = 10,
    K = 11,
    A = 12,
}
impl Card {
    fn new(card_char: char) -> Card {
        match card_char {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("Invalid card_char"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Hand {
    cards: [Card; 5],
    bid: u32,
}
impl Hand {
    fn new(line: &str) -> Hand {
        let mut hand_iter = line.split_ascii_whitespace();

        let mut card_iter = hand_iter
            .next()
            .expect("hand_iter should have hand portion of string")
            .chars()
            .map(Card::new);
        let cards = std::array::from_fn(|_| {
            card_iter
                .next()
                .expect("card_iter should have exactly five items")
        });

        let bid = hand_iter
            .next()
            .expect("hand_iter should have bid portion of string")
            .parse()
            .expect("Bid should be a valid number");

        Hand { cards, bid }
    }

    fn hand_type(&self) -> HandType {
        let mut card_counts: [usize; 13] = [0; 13];
        for card in &self.cards {
            // By adding 1 to the specific index associated with the card, card_counts will
            // ultimately be a count of how many copies of each card was found.
            card_counts[*card as usize] += 1;
        }

        let mut ones_found: u8 = 0;
        let mut twos_found: u8 = 0;
        let mut threes_found: u8 = 0;
        // There is no need to track fours or fives found; the existence of either immediately
        // narrows down the possible hand types to one option.
        for card_count in card_counts {
            if card_count == 5 {
                return HandType::FiveOfAKind;
            }
            if card_count == 4 {
                return HandType::FourOfAKind;
            }
            if card_count == 3 {
                if twos_found == 1 {
                    return HandType::FullHouse;
                }
                if ones_found >= 1 {
                    return HandType::ThreeOfAKind;
                }
                threes_found = 1;
            }
            if card_count == 2 {
                if threes_found == 1 {
                    return HandType::FullHouse;
                }
                if twos_found == 1 {
                    return HandType::TwoPair;
                }
                if ones_found >= 2 {
                    return HandType::OnePair;
                }
                twos_found = 1;
            }
            if card_count == 1 {
                if threes_found == 1 {
                    return HandType::ThreeOfAKind;
                }
                if ones_found >= 1 && twos_found == 1 {
                    return HandType::OnePair;
                }
                if ones_found >= 3 {
                    return HandType::HighCard;
                }

                ones_found += 1;
            }
        }

        panic!("Failed to find a hand type")
    }
}

// This manual implementation of Ord sorts Hand first by its type, then by the strength of each card
// in order. Arrays automatically compare themselves element-by-element, so just comparing the
// entire array is sufficient for the second step.
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type()
            .cmp(&other.hand_type())
            .then(self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

fn solve_1(input: &str) -> Solution {
    let mut hands = input.lines().map(Hand::new).collect::<Vec<_>>();

    // This works because of the custom Ord implementation on Hand.
    hands.sort_unstable();

    let mut winnings = 0;
    for (rank, hand) in (1..).zip(hands) {
        winnings += rank * hand.bid;
    }

    Solution::U32(winnings)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            Solution::U16(6440)
        );
    }
}
