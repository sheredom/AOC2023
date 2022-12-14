use std::mem::swap;

use itertools::Itertools;

use crate::{Solution, SolutionPair};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day14/input"));

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Square {
    Empty,
    Rock,
    Sand,
}

pub fn solve() -> SolutionPair {
    solver(INPUT)
}

fn solver(data: &str) -> SolutionPair {
    const X_SIZE: usize = 1000;
    const Y_SIZE: usize = 200;
    let mut grid = [[Square::Empty; Y_SIZE]; X_SIZE];
    let mut max_y = 0;

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }

        for (start, end) in line.split("->").tuple_windows() {
            let (sx, sy) = start.split_once(',').unwrap();
            let (ex, ey) = end.split_once(',').unwrap();

            let mut sx = sx.trim().parse::<usize>().unwrap();
            let mut sy = sy.trim().parse::<usize>().unwrap();
            let mut ex = ex.trim().parse::<usize>().unwrap();
            let mut ey = ey.trim().parse::<usize>().unwrap();

            if sx > ex {
                swap(&mut sx, &mut ex);
            }

            if sy > ey {
                swap(&mut sy, &mut ey);
            }

            max_y = max_y.max(ey);

            for row in grid.iter_mut().take(ex + 1).skip(sx) {
                for element in row.iter_mut().take(ey + 1).skip(sy) {
                    *element = Square::Rock;
                }
            }
        }
    }

    let mut p1 = 0;

    loop {
        let mut sand = (500, 0);

        if grid[sand.0][sand.1] != Square::Empty {
            break;
        }

        grid[sand.0][sand.1] = Square::Sand;

        loop {
            if sand.1 == (Y_SIZE - 1) {
                // we've hit the abyss, meaning we're done!
                grid[sand.0][sand.1] = Square::Empty;
                break;
            }

            let next = if grid[sand.0][sand.1 + 1] == Square::Empty {
                // we go down!
                (sand.0, sand.1 + 1)
            } else if grid[sand.0 - 1][sand.1 + 1] == Square::Empty {
                // we go left!
                (sand.0 - 1, sand.1 + 1)
            } else if grid[sand.0 + 1][sand.1 + 1] == Square::Empty {
                // we go right!
                (sand.0 + 1, sand.1 + 1)
            } else {
                break;
            };

            // empty out our square as we're moving!
            grid[sand.0][sand.1] = Square::Empty;

            // update our location
            sand = next;

            // and make some sand
            grid[sand.0][sand.1] = Square::Sand;
        }

        if sand.1 == (Y_SIZE - 1) {
            for row in &grid {
                for element in row {
                    if *element == Square::Sand {
                        p1 += 1;
                    }
                }
            }

            // draw a floor at max_y + 2
            for row in &mut grid {
                row[max_y + 2] = Square::Rock;
            }
        }
    }

    let mut p2 = 0;

    for row in &grid {
        for element in row {
            if *element == Square::Sand {
                p2 += 1;
            }
        }
    }

    (Solution::U32(p1), Solution::U32(p2))
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    use super::solver;

    static EXAMPLE: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

    #[test]
    fn it_works() {
        let solution = solver(EXAMPLE);

        if let (Solution::U32(p1), Solution::U32(p2)) = solution {
            assert_eq!(p1, 24);
            assert_eq!(p2, 93);
        } else {
            panic!();
        }
    }
}
