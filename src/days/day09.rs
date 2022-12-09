use std::collections::HashSet;

use crate::{Solution, SolutionPair};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day09/input"));

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    solver(INPUT)
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

struct Rope {
    parts: Vec<(i16, i16)>,
}

impl Rope {
    pub fn new(length: usize) -> Self {
        let mut parts = Vec::new();
        parts.resize(length, (0, 0));

        Self { parts }
    }

    pub fn move_head(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                self.parts[0].1 += 1;
            }
            Direction::Left => {
                self.parts[0].0 -= 1;
            }
            Direction::Down => {
                self.parts[0].1 -= 1;
            }
            Direction::Right => {
                self.parts[0].0 += 1;
            }
        }

        for i in 1..(self.parts.len()) {
            let last = self.parts[i - 1];
            let mut part = &mut self.parts[i];

            let (x, y) = (last.0 - part.0, last.1 - part.1);

            if x.abs() > 1 || y.abs() > 1 {
                part.0 += x.signum();
                part.1 += y.signum();
            }
        }
    }

    pub fn tail(&self) -> (i16, i16) {
        *self.parts.last().unwrap()
    }
}

fn solver(data: &str) -> SolutionPair {
    let mut p1_set = HashSet::new();
    let mut p2_set = HashSet::new();

    let mut p1 = Rope::new(2);
    let mut p2 = Rope::new(10);

    // we always visit (0, 0)!
    p1_set.insert(p1.tail());
    p2_set.insert(p2.tail());

    for line in data.lines() {
        let (direction, amount) = line.split_once(' ').unwrap();
        let amount = amount.parse::<i16>().unwrap();

        let direction = match direction {
            "U" => Direction::Up,
            "L" => Direction::Left,
            "D" => Direction::Down,
            "R" => Direction::Right,
            _ => panic!(),
        };

        for _ in 0..amount {
            p1.move_head(direction);
            p1_set.insert(p1.tail());

            p2.move_head(direction);
            p2_set.insert(p2.tail());
        }
    }

    (
        Solution::U32(p1_set.len() as u32),
        Solution::U32(p2_set.len() as u32),
    )
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    use super::solver;

    static EXAMPLES: [(&str, u32, u32); 2] = [
        (
            r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#,
            13,
            1,
        ),
        (
            r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#,
            88,
            36,
        ),
    ];

    #[test]
    fn it_works() {
        for (example, r1, r2) in EXAMPLES {
            let solution = solver(example);

            if let (Solution::U32(p1), Solution::U32(p2)) = solution {
                assert_eq!(p1, r1);
                assert_eq!(p2, r2);
            } else {
                panic!();
            }
        }
    }
}
