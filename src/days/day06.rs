use itertools::Itertools;

use crate::{Solution, SolutionPair};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day06/input"));

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    solver(INPUT)
}

fn solver(data: &str) -> SolutionPair {
    (solution::<4>(data), solution::<14>(data))
}

fn solution<const COUNT: usize>(data: &str) -> Solution {
    let mut chars = data.chars();

    let mut cache: [char; COUNT] = ['\0'; COUNT];

    for i in 0..(COUNT - 1) {
        cache[i] = chars.next().unwrap();
    }

    let mut insert_pos = COUNT - 1;

    let mut part1 = None;

    for (index, char) in chars.enumerate() {
        cache[insert_pos] = char;

        insert_pos += 1;
        insert_pos %= COUNT;

        // now check if the cache has anything in it that is a duplicate
        let mut duplicate = false;

        for (i, c0) in cache.iter().enumerate() {
            for c1 in cache.iter().skip(i + 1) {
                if c0 == c1 {
                    duplicate = true;
                    break;
                }
            }

            if duplicate {
                break;
            }
        }

        if duplicate {
            continue;
        }

        part1 = Some(COUNT + index);
        break;
    }

    Solution::U32(part1.unwrap() as u32)
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    use super::solver;

    static EXAMPLES: [(&str, u32); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    #[test]
    fn it_works() {
        for (example, r1) in EXAMPLES {
            let solution = solver(example);

            if let (Solution::U32(p1), Solution::U32(_)) = solution {
                assert_eq!(p1, r1);
                //assert_eq!(p2, "MCD");
            } else {
                panic!();
            }
        }
    }
}
