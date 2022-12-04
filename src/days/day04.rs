use crate::{Solution, SolutionPair};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day04/input"));

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    solver(INPUT)
}

fn solver(data: &str) -> SolutionPair {
    let mut p1_overlaps = 0;
    let mut p2_overlaps = 0;

    for line in data.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let mut splits = line
            .split(|c| c == '-' || c == ',')
            .map(|s| s.parse::<u32>().unwrap());

        let p0 = splits.next().unwrap()..=splits.next().unwrap();
        let p1 = splits.next().unwrap()..=splits.next().unwrap();

        if (p0.contains(p1.start()) && p0.contains(p1.end()))
            || (p1.contains(p0.start()) && p1.contains(p0.end()))
        {
            p1_overlaps += 1;
        }

        if p0.contains(p1.start())
            || p0.contains(p1.end())
            || p1.contains(p0.start())
            || p1.contains(p0.end())
        {
            p2_overlaps += 1;
        }
    }

    (Solution::U32(p1_overlaps), Solution::U32(p2_overlaps))
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    use super::solver;

    static EXAMPLE: &str = r#"2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
"#;

    #[test]
    fn it_works() {
        let solution = solver(EXAMPLE);

        if let (Solution::U32(p1), Solution::U32(p2)) = solution {
            assert_eq!(p1, 2);
            assert_eq!(p2, 4);
        } else {
            panic!();
        }
    }
}
