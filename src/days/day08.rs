use itertools::izip;

use crate::{Solution, SolutionPair};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day08/input"));

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    solver(INPUT)
}

fn solver(data: &str) -> SolutionPair {
    let mut trees = Vec::new();

    let mut width = None;
    let mut height = 0;

    for line in data.lines() {
        height += 1;

        if let Some(width) = width {
            assert_eq!(width, line.len());
        } else {
            width = Some(line.len());
        }

        for char in line.chars() {
            trees.push(char.to_digit(10).unwrap() as i8);
        }
    }

    let width = width.unwrap();

    let p1 = part1(&trees, width, height);
    let p2 = part2(&trees, width, height);

    (Solution::U32(p1), Solution::U32(p2))
}

fn part1(trees: &[i8], width: usize, height: usize) -> u32 {
    let mut visible = Vec::new();
    visible.resize(trees.len(), false);

    // cast from top
    for w in 0..width {
        let mut max = -1;

        for h in 0..height {
            let i = h * width + w;

            if trees[i] > max {
                visible[i] = true;
                max = trees[i];
            }
        }
    }

    // cast from left
    for h in 0..height {
        let mut max = -1;

        for w in 0..width {
            let i = h * width + w;

            if trees[i] > max {
                visible[i] = true;
                max = trees[i];
            }
        }
    }

    // cast from bottom
    for w in 0..width {
        let mut max = -1;

        for h in (0..height).rev() {
            let i = h * width + w;

            if trees[i] > max {
                visible[i] = true;
                max = trees[i];
            }
        }
    }

    // cast from right
    for h in 0..height {
        let mut max = -1;

        for w in (0..width).rev() {
            let i = h * width + w;

            if trees[i] > max {
                visible[i] = true;
                max = trees[i];
            }
        }
    }

    visible.iter().filter(|item| **item).count() as u32
}

fn part2(trees: &[i8], width: usize, height: usize) -> u32 {
    let mut top_score = Vec::new();
    top_score.resize(trees.len(), 0);

    let mut left_score = top_score.clone();
    let mut right_score = top_score.clone();
    let mut bottom_score = top_score.clone();

    for h in 0..height {
        for w in 0..width {
            let i = h * width + w;

            // top
            {
                let mut next = h;

                loop {
                    let (o, overflow) = next.overflowing_sub(1);
                    next = o;

                    if overflow {
                        break;
                    }

                    let next = next * width + w;

                    top_score[i] += 1;

                    if trees[i] <= trees[next] {
                        break;
                    }
                }
            }

            // left
            {
                let mut next = w;

                loop {
                    let (o, overflow) = next.overflowing_sub(1);
                    next = o;

                    if overflow {
                        break;
                    }

                    let next = h * width + next;

                    left_score[i] += 1;

                    if trees[i] <= trees[next] {
                        break;
                    }
                }
            }

            // bottom
            {
                let mut next = h;

                loop {
                    let (o, overflow) = (next + 1, (next == (height - 1)));
                    next = o;

                    if overflow {
                        break;
                    }

                    let next = next * width + w;

                    bottom_score[i] += 1;

                    if trees[i] <= trees[next] {
                        break;
                    }
                }
            }

            // right
            {
                let mut next = w;

                loop {
                    let (o, overflow) = (next + 1, next == (width - 1));
                    next = o;

                    if overflow {
                        break;
                    }

                    let next = h * width + next;

                    right_score[i] += 1;

                    if trees[i] <= trees[next] {
                        break;
                    }
                }
            }
        }
    }

    for i in 0..width {
        top_score[i] = 0;
        let len = bottom_score.len();
        bottom_score[len - 1 - i] = 0;
    }

    for i in 0..height {
        left_score[i * width] = 0;
        let len = right_score.len();
        right_score[len - 1 - (i * width)] = 0;
    }

    izip!(top_score, left_score, bottom_score, right_score)
        .map(|(t, l, b, r)| t * l * b * r)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    use super::solver;

    static EXAMPLE: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn it_works() {
        let solution = solver(EXAMPLE);

        if let (Solution::U32(p1), Solution::U32(p2)) = solution {
            assert_eq!(p1, 21);
            assert_eq!(p2, 8);
        } else {
            panic!();
        }
    }
}
