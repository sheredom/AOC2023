use std::collections::HashMap;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve1(input: &str) -> Solution {
    let instructions = input.lines().next().unwrap();

    let mut map = HashMap::new();

    for line in input.lines().skip(1) {
        if line.is_empty() {
            continue;
        }

        let (node, line) = line.split_once('=').unwrap();
        let node = node.trim();

        let (left, right) = line.split_once(',').unwrap();
        let left = left.trim_matches(|x| matches!(x, '(' | ')' | ',' | ' '));
        let right = right.trim_matches(|x| matches!(x, '(' | ')' | ',' | ' '));

        assert_eq!(3, node.len());
        assert_eq!(3, left.len());
        assert_eq!(3, right.len());

        map.insert(node, (left, right));
    }

    let mut current = "AAA";
    let mut index = 0;
    let mut steps = 0;

    while "ZZZ" != current {
        let instruction = instructions.chars().nth(index).unwrap();

        match instruction {
            'L' => current = map[current].0,
            'R' => current = map[current].1,
            _ => unreachable!(),
        }

        index += 1;
        index %= instructions.len();

        steps += 1;
    }

    Solution::U32(steps)
}

pub fn solve2(input: &str) -> Solution {
    let instructions = input.lines().next().unwrap();

    let mut current = Vec::new();
    let mut map = HashMap::new();

    for line in input.lines().skip(1) {
        if line.is_empty() {
            continue;
        }

        let (node, line) = line.split_once('=').unwrap();
        let node = node.trim();

        if node.ends_with('A') {
            current.push(node);
        }

        let (left, right) = line.split_once(',').unwrap();
        let left = left.trim_matches(|x| matches!(x, '(' | ')' | ',' | ' '));
        let right = right.trim_matches(|x| matches!(x, '(' | ')' | ',' | ' '));

        assert_eq!(3, node.len());
        assert_eq!(3, left.len());
        assert_eq!(3, right.len());

        map.insert(node, (left, right));
    }

    let sol = current
        .iter()
        .map(|x| {
            let mut x = *x;

            let mut index = 0;
            let mut steps = 0;

            while !x.ends_with('Z') {
                let instruction = instructions.chars().nth(index).unwrap();

                match instruction {
                    'L' => x = map[x].0,
                    'R' => x = map[x].1,
                    _ => unreachable!(),
                }

                index += 1;
                index %= instructions.len();

                steps += 1;
            }

            steps as u64
        })
        .fold(1, num::integer::lcm);

    Solution::U64(sol)
}

pub fn solve() -> SolutionPair {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day08"));

    (solve1(input), solve2(input))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let input = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(super::solve1(input), Solution::U32(2));

        let input = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(super::solve1(input), Solution::U32(6));

        let input = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(super::solve2(input), Solution::U64(6));
    }
}
