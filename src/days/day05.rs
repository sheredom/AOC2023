use iset::IntervalMap;
use itertools::*;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type IntTypeToUse = i64;

pub fn solve1(input: &str) -> Solution {
    let mut seeds = Vec::new();

    let mut maps = Vec::new();

    for line in input.lines() {
        if let Some(line) = line.strip_prefix("seeds: ") {
            for seed in line.split(' ') {
                let seed = seed.parse::<IntTypeToUse>().unwrap();
                seeds.push(seed);
            }
        } else if line.ends_with("map:") {
            // We're just assuming everything is in order with the intervals!
            maps.push(IntervalMap::new());
        } else if !line.is_empty() {
            let (dst, line) = line.split_once(' ').unwrap();
            let (src, len) = line.split_once(' ').unwrap();

            let dst = dst.parse::<IntTypeToUse>().unwrap();
            let src = src.parse::<IntTypeToUse>().unwrap();
            let len = len.parse::<IntTypeToUse>().unwrap();

            let map = maps.last_mut().unwrap();

            map.insert(src..(src + len), dst);
        }
    }

    let mut sol = IntTypeToUse::MAX;

    for seed in seeds {
        let mut id = seed;

        for map in &maps {
            let mut values = map.overlap(id);

            if let Some((range, dst)) = values.next() {
                // We need to get the correct offset into dest, which means taking the base source range away from our current id (which could have been in the middle of the original range!).
                let len = id - range.start;

                id = dst + len;
            }

            assert_eq!(0, values.count());

            // Otherwise id just maps to itself and we don't have to do anything.
        }

        sol = sol.min(id);
    }

    Solution::I64(sol)
}

pub fn solve2(input: &str) -> Solution {
    let mut ranges = Vec::new();

    let mut maps = Vec::new();

    for line in input.lines() {
        if let Some(line) = line.strip_prefix("seeds: ") {
            for (start, len) in line.split(' ').tuples() {
                let start = start.parse::<IntTypeToUse>().unwrap();
                let len = len.parse::<IntTypeToUse>().unwrap();
                ranges.push(start..(start + len));
            }
        } else if line.ends_with("map:") {
            // We're just assuming everything is in order with the intervals!
            maps.push(IntervalMap::new());
        } else if !line.is_empty() {
            let (dst, line) = line.split_once(' ').unwrap();
            let (src, len) = line.split_once(' ').unwrap();

            let dst = dst.parse::<IntTypeToUse>().unwrap();
            let src = src.parse::<IntTypeToUse>().unwrap();
            let len = len.parse::<IntTypeToUse>().unwrap();

            let map = maps.last_mut().unwrap();

            map.insert(src..(src + len), dst);
        }
    }

    for map in &mut maps {
        let mut ranges_missing = Vec::new();

        let mut start = 0;

        for (range, _) in map.iter(0..IntTypeToUse::MAX) {
            if start < range.start {
                // Remember the missing range.
                ranges_missing.push(start..range.start);
            }

            // And bump over to the end of the known recorded range.
            start = range.end;
        }

        ranges_missing.push(start..IntTypeToUse::MAX);

        for range in ranges_missing {
            let start = range.start;
            map.insert(range, start);
        }
    }

    for map in &maps {
        let ranges_copy = ranges;

        // Wipe out ranges so we can record the next loop iteration!
        ranges = Vec::new();

        for in_range in ranges_copy {
            for (range, to_dst) in map.iter(in_range.clone()) {
                let offset = to_dst - range.start;

                let clamped_range = range.start.max(in_range.start)..range.end.min(in_range.end);

                ranges.push((clamped_range.start + offset)..(clamped_range.end + offset));
            }
        }
    }

    let sol = ranges
        .iter()
        .fold(IntTypeToUse::MAX, |acc, x| acc.min(x.start));

    Solution::I64(sol)
}

pub fn solve() -> SolutionPair {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day05"));

    (solve1(input), solve2(input))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(super::solve1(input), Solution::I64(35));
        assert_eq!(super::solve2(input), Solution::I64(46));
    }
}
