use core::panic;
use std::collections::HashSet;

use itertools::Itertools;

use crate::{Solution, SolutionPair};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day03/input"));

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    solver(INPUT)
}

fn cost(found: char) -> u32 {
    match found {
        'a'..='z' => 1 + (found as u32) - ('a' as u32),
        'A'..='Z' => 27 + (found as u32) - ('A' as u32),
        _ => panic!(),
    }
}

fn solver(data: &str) -> SolutionPair {
    let mut p1_costs = 0;

    for line in data.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let mid = line.len() / 2;

        let (bot, top) = line.split_at(mid);

        let mut found = None;

        for b in bot.chars() {
            for t in top.chars() {
                if b == t {
                    found = Some(b);
                    break;
                }
            }

            if found.is_some() {
                break;
            }
        }

        assert!(found.is_some());

        let found = found.unwrap();

        p1_costs += cost(found);
    }

    let mut p2_costs = 0;

    for chunk in &data.lines().chunks(3) {
        let mut sets = Vec::new();

        for line in chunk {
            let mut set = HashSet::new();

            for item in line.chars() {
                set.insert(item);
            }

            sets.push(set);
        }

        let mut set = sets.pop().unwrap();

        for other in &sets {
            set = set.intersection(other).copied().collect();
        }

        assert_eq!(set.len(), 1);

        let item = *set.iter().next().unwrap();

        p2_costs += cost(item);
    }

    (Solution::U32(p1_costs), Solution::U32(p2_costs))
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    use super::solver;

    static EXAMPLE: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

    #[test]
    fn it_works() {
        let solution = solver(EXAMPLE);

        if let (Solution::U32(p1), Solution::U32(p2)) = solution {
            assert_eq!(p1, 157);
            assert_eq!(p2, 70);
        } else {
            panic!();
        }
    }
}
