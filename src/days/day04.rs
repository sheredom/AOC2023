use std::collections::{HashMap, HashSet};

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve1(input: &str) -> Solution {
    let mut sol = 0;

    for line in input.lines() {
        let mut winners = HashSet::new();

        let (_, line) = line.split_once(':').unwrap();
        let (winning_numbers, my_numbers) = line.split_once('|').unwrap();

        for winning_number in winning_numbers.trim().split(' ') {
            if winning_number.is_empty() {
                continue;
            }

            let winning_number = winning_number.parse::<u8>().unwrap();
            winners.insert(winning_number);
        }

        let mut number_found = 0;

        for my_number in my_numbers.trim().split(' ') {
            if my_number.is_empty() {
                continue;
            }

            let my_number = my_number.parse::<u8>().unwrap();
            if winners.contains(&my_number) {
                number_found += 1;
            }
        }

        // 0 -> 0, 1 -> 1, 2 -> 2, 3 -> 4, 4 -> 8, etc.
        let points = if 0 == number_found {
            0
        } else {
            2_u32.pow(number_found - 1)
        };

        sol += points;
    }

    Solution::U32(sol)
}

pub fn solve2(input: &str) -> Solution {
    let mut scratch_card_hits = HashMap::new();

    for (scratch_card_id, line) in input.lines().enumerate() {
        let mut winners = HashSet::new();

        let (_, line) = line.split_once(':').unwrap();
        let (winning_numbers, my_numbers) = line.split_once('|').unwrap();

        for winning_number in winning_numbers.trim().split(' ') {
            if winning_number.is_empty() {
                continue;
            }

            let winning_number = winning_number.parse::<u8>().unwrap();
            winners.insert(winning_number);
        }

        let mut number_found = 0;

        for my_number in my_numbers.trim().split(' ') {
            if my_number.is_empty() {
                continue;
            }

            let my_number = my_number.parse::<u8>().unwrap();

            if winners.contains(&my_number) {
                number_found += 1;
            }
        }

        scratch_card_hits.entry(scratch_card_id).or_insert(0);

        *scratch_card_hits.get_mut(&scratch_card_id).unwrap() += 1;

        if 0 == number_found {
            continue;
        }

        for index in 1..=number_found {
            let id = scratch_card_id + index;

            scratch_card_hits.entry(id).or_insert(0);

            *scratch_card_hits.get_mut(&id).unwrap() +=
                *scratch_card_hits.get(&scratch_card_id).unwrap();
        }
    }

    let sol = scratch_card_hits.iter().fold(0, |acc, (_, v)| acc + *v);

    Solution::U32(sol)
}

pub fn solve() -> SolutionPair {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day04"));

    (solve1(input), solve2(input))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(super::solve1(input), Solution::U32(13));
        assert_eq!(super::solve2(input), Solution::U32(30));
    }
}
