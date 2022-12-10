use std::fmt::Write;

use crate::{Solution, SolutionPair};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day10/input"));

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    solver(INPUT)
}

fn solver(data: &str) -> SolutionPair {
    let mut p1 = 0;
    let mut p2 = String::with_capacity(40 * 6);

    writeln!(&mut p2).unwrap();

    let mut x = 1;

    let mut deferred_amount = None;

    let interesting_cycle_start = 20;
    let interesting_cycle_recurrance = 40;

    let mut lines = data.lines();

    for cycle in 1.. {
        if ((cycle - interesting_cycle_start) % interesting_cycle_recurrance) == 0 {
            p1 += cycle * x;
        }

        println!("x {} cycle {}", x, cycle);

        p2 += if ((x - 1)..=(x + 1)).contains(&((cycle - 1) % 40)) {
            "#"
        } else {
            "."
        };

        if cycle % 40 == 0 {
            writeln!(&mut p2).unwrap();
        }

        if let Some(amount) = deferred_amount {
            x += amount;
            deferred_amount = None;
        } else {
            let line = lines.next();

            if line.is_none() {
                break;
            }

            let line = line.unwrap();

            if !line.starts_with("noop") {
                let (instruction, amount) = line.split_once(' ').unwrap();
                assert_eq!(instruction, "addx");

                let amount = amount.parse::<i32>().unwrap();

                deferred_amount = Some(amount);
            }
        }
    }

    (Solution::I32(p1), Solution::Str(p2))
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    use super::solver;

    static EXAMPLE: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    #[test]
    fn it_works() {
        let solution = solver(EXAMPLE);

        if let (Solution::I32(p1), Solution::Str(p2)) = solution {
            assert_eq!(p1, 13140);
            assert_eq!(
                p2,
                r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
."#
            );
        } else {
            panic!();
        }
    }
}
