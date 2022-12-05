use itertools::Itertools;

use crate::{Solution, SolutionPair};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day05/input"));

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    solver(INPUT)
}

fn part1(data: &str) -> Solution {
    let mut stacks: Vec<Vec<char>> = Vec::new();

    for line in data.lines() {
        if line.is_empty() || line.starts_with(" 1") {
            // skip the numbered line cause it doesn't give us any info
            continue;
        }

        if line.starts_with("move") {
            let (lo, hi) = line.split_once("from").unwrap();
            let (mi, hi) = hi.split_once("to").unwrap();

            let amount = lo
                .strip_prefix("move ")
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();
            let from = mi.trim().parse::<usize>().unwrap() - 1;
            let to = hi.trim().parse::<usize>().unwrap() - 1;

            for _ in 0..amount {
                let container = stacks[from].pop().unwrap();
                stacks[to].push(container);
            }
        } else {
            let num_stacks = line.len() / 3;

            if stacks.is_empty() {
                for _ in 0..num_stacks {
                    stacks.push(Vec::new());
                }
            } else {
                assert_eq!(num_stacks, stacks.len());
            }

            for (stack, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
                match chunk.nth(1).unwrap() {
                    ' ' => (),
                    x => stacks[stack].insert(0, x),
                }
            }
        }
    }

    let mut result = String::new();

    for stack in &stacks {
        if let Some(container) = stack.last() {
            result.push(*container);
        } else {
            result.push(' ');
        }
    }

    Solution::Str(result)
}

fn part2(data: &str) -> Solution {
    let mut stacks: Vec<Vec<char>> = Vec::new();

    for line in data.lines() {
        if line.is_empty() || line.starts_with(" 1") {
            // skip the numbered line cause it doesn't give us any info
            continue;
        }

        if line.starts_with("move") {
            let (lo, hi) = line.split_once("from").unwrap();
            let (mi, hi) = hi.split_once("to").unwrap();

            let amount = lo
                .strip_prefix("move ")
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();
            let from = mi.trim().parse::<usize>().unwrap() - 1;
            let to = hi.trim().parse::<usize>().unwrap() - 1;

            for i in 0..amount {
                let len = stacks[from].len();

                let container_index = len - amount + i;

                let container = stacks[from][container_index];
                stacks[to].push(container);
            }

            for _ in 0..amount {
                stacks[from].pop();
            }
        } else {
            let num_stacks = line.len() / 3;

            if stacks.is_empty() {
                for _ in 0..num_stacks {
                    stacks.push(Vec::new());
                }
            } else {
                assert_eq!(num_stacks, stacks.len());
            }

            for (stack, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
                match chunk.nth(1).unwrap() {
                    ' ' => (),
                    x => stacks[stack].insert(0, x),
                }
            }
        }
    }

    let mut result = String::new();

    for stack in &stacks {
        if let Some(container) = stack.last() {
            result.push(*container);
        } else {
            result.push(' ');
        }
    }

    Solution::Str(result)
}

fn solver(data: &str) -> SolutionPair {
    (part1(data), part2(data))
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    use super::solver;

    static EXAMPLE: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn it_works() {
        let solution = solver(EXAMPLE);

        if let (Solution::Str(p1), Solution::Str(p2)) = solution {
            assert_eq!(p1, "CMZ");
            assert_eq!(p2, "MCD");
        } else {
            panic!();
        }
    }
}
