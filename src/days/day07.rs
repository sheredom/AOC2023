use itertools::Itertools;
use std::cmp::Ordering;
use strum::EnumCount;
use strum_macros::EnumCount;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, EnumCount, PartialEq, PartialOrd, Ord, Eq)]
enum Card1 {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl From<char> for Card1 {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => unreachable!(),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Hand1 {
    bid: u32,
    hand: [Card1; 5],
}

impl Hand1 {
    pub fn new(bid: u32, hand: &str) -> Hand1 {
        let mut hand_array = [Card1::Ace; 5];

        for (index, c) in hand.chars().enumerate() {
            hand_array[index] = Card1::from(c);
        }

        Hand1 {
            bid,
            hand: hand_array,
        }
    }

    fn rank(&self) -> usize {
        let mut hits = [0; Card1::COUNT];

        for c in self.hand {
            hits[c as usize] += 1;
        }

        hits.sort();

        let cur = hits[Card1::COUNT - 1];
        let nxt = hits[Card1::COUNT - 2];

        if 5 == cur {
            // Five of a kind.
            assert_eq!(0, nxt);
            0
        } else if 4 == cur {
            // Four of a kind.
            assert_eq!(1, nxt);
            1
        } else if 3 == cur && 2 == nxt {
            // Full house.
            2
        } else if 3 == cur {
            // Three of a kind.
            assert_eq!(1, nxt);
            3
        } else if 2 == cur && 2 == nxt {
            // Two pair.
            4
        } else if 2 == cur {
            // One pair.
            assert_eq!(1, nxt);
            5
        } else {
            assert_eq!(1, cur);
            assert_eq!(1, nxt);
            6
        }
    }
}

impl Ord for Hand1 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Hand1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let my_rank = self.rank();
        let other_rank = other.rank();

        match my_rank.cmp(&other_rank) {
            Ordering::Greater => Some(Ordering::Less),
            Ordering::Less => Some(Ordering::Greater),
            Ordering::Equal => {
                for (m, o) in self.hand.iter().zip_eq(other.hand.iter()) {
                    match m.cmp(o) {
                        Ordering::Greater => return Some(std::cmp::Ordering::Less),
                        Ordering::Less => return Some(std::cmp::Ordering::Greater),
                        Ordering::Equal => (),
                    }
                }

                unreachable!()
            }
        }
    }
}

pub fn solve1(input: &str) -> Solution {
    let mut hands = Vec::new();

    for line in input.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        assert_eq!(5, hand.len());
        let bid = bid.parse::<u32>().unwrap();

        hands.push(Hand1::new(bid, hand));
    }

    hands.sort();

    let mut sol = 0;

    for (index, hand) in hands.iter().enumerate() {
        sol += ((index as u32) + 1) * hand.bid;
    }

    Solution::U32(sol)
}

#[derive(Clone, Copy, Debug, EnumCount, PartialEq, PartialOrd, Ord, Eq)]
enum Card2 {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl From<char> for Card2 {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            'J' => Self::Joker,
            _ => unreachable!(),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Hand2 {
    bid: u32,
    hand: [Card2; 5],
}

impl Hand2 {
    pub fn new(bid: u32, hand: &str) -> Hand2 {
        let mut hand_array = [Card2::Ace; 5];

        for (index, c) in hand.chars().enumerate() {
            hand_array[index] = Card2::from(c);
        }

        Hand2 {
            bid,
            hand: hand_array,
        }
    }

    fn rank(&self) -> usize {
        let mut hits = [0; Card2::COUNT];

        let mut jokers = 0;

        for c in self.hand {
            if Card2::Joker == c {
                jokers += 1;
            } else {
                hits[c as usize] += 1;
            }
        }

        let jokers = jokers;

        hits.sort();

        let cur = jokers + hits[Card1::COUNT - 1];
        let nxt = hits[Card1::COUNT - 2];

        if 5 == cur {
            // Five of a kind.
            assert_eq!(0, nxt);
            0
        } else if 4 == cur {
            // Four of a kind.
            assert_eq!(1, nxt);
            1
        } else if 3 == cur && 2 == nxt {
            // Full house.
            2
        } else if 3 == cur {
            // Three of a kind.
            assert_eq!(1, nxt);
            3
        } else if 2 == cur && 2 == nxt {
            // Two pair.
            4
        } else if 2 == cur {
            // One pair.
            assert_eq!(1, nxt);
            5
        } else {
            assert_eq!(1, cur);
            assert_eq!(1, nxt);
            6
        }
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let my_rank = self.rank();
        let other_rank = other.rank();

        match my_rank.cmp(&other_rank) {
            Ordering::Greater => Some(Ordering::Less),
            Ordering::Less => Some(Ordering::Greater),
            Ordering::Equal => {
                for (m, o) in self.hand.iter().zip_eq(other.hand.iter()) {
                    match m.cmp(o) {
                        Ordering::Greater => return Some(std::cmp::Ordering::Less),
                        Ordering::Less => return Some(std::cmp::Ordering::Greater),
                        Ordering::Equal => (),
                    }
                }

                unreachable!()
            }
        }
    }
}

pub fn solve2(input: &str) -> Solution {
    let mut hands = Vec::new();

    for line in input.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        assert_eq!(5, hand.len());
        let bid = bid.parse::<u32>().unwrap();

        hands.push(Hand2::new(bid, hand));
    }

    hands.sort();

    let mut sol = 0;

    for (index, hand) in hands.iter().enumerate() {
        sol += ((index as u32) + 1) * hand.bid;
    }

    Solution::U32(sol)
}

pub fn solve() -> SolutionPair {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day07"));

    (solve1(input), solve2(input))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(super::solve1(input), Solution::U32(6440));
        assert_eq!(super::solve2(input), Solution::U32(5905));
    }
}
