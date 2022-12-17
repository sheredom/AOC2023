use regex::Regex;

use crate::{Solution, SolutionPair};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day15/input"));

///////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Debug)]
enum Data {
    Unknown,
    Empty,
    Sensor,
    Beacon,
}

struct Sensor {
    sensor: (i32, i32),
    beacon: (i32, i32),
    radius: i32,
}

impl Sensor {
    pub fn new(sensor: (i32, i32), beacon: (i32, i32)) -> Self {
        let radius = Self::calculate_manhattan_distance(sensor, beacon);

        Self {
            sensor,
            beacon,
            radius,
        }
    }

    pub fn min_known_x(&self) -> i32 {
        self.sensor.0 - self.manhattan_distance()
    }

    pub fn max_known_x(&self) -> i32 {
        self.sensor.0 + self.manhattan_distance()
    }

    pub fn range_of_x_at_y(&self, y: i32) -> Option<(i32, i32)> {
        let x = self.sensor.0;

        let d = Self::calculate_manhattan_distance(self.sensor, (x, y));

        if d <= self.radius {
            let diff = self.radius - d;

            Some((x - diff, x + diff))
        } else {
            None
        }
    }

    fn calculate_manhattan_distance((x, y): (i32, i32), (a, b): (i32, i32)) -> i32 {
        (x.abs_diff(a) + y.abs_diff(b)) as i32
    }

    pub fn manhattan_distance(&self) -> i32 {
        self.radius
    }

    pub fn query(&self, (x, y): (i32, i32)) -> Data {
        if (x, y) == self.sensor {
            Data::Sensor
        } else if (x, y) == self.beacon {
            Data::Beacon
        } else if Self::calculate_manhattan_distance(self.sensor, (x, y))
            <= self.manhattan_distance()
        {
            Data::Empty
        } else {
            Data::Unknown
        }
    }
}

pub fn solve() -> SolutionPair {
    solver(INPUT, 2000000, 4000000)
}

fn solver(data: &str, p1_y: i32, p2_size: i32) -> SolutionPair {
    let mut sensors = Vec::new();

    let re = Regex::new(
        r"Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)",
    )
    .unwrap();

    for line in data.lines() {
        for capture in re.captures_iter(line) {
            let (sx, sy) = (&capture[1], &capture[2]);
            let (bx, by) = (&capture[3], &capture[4]);

            let sx = sx.parse::<i32>().unwrap();
            let sy = sy.parse::<i32>().unwrap();
            let bx = bx.parse::<i32>().unwrap();
            let by = by.parse::<i32>().unwrap();

            sensors.push(Sensor::new((sx, sy), (bx, by)));
        }
    }

    let min_x = sensors
        .iter()
        .map(|sensor| sensor.min_known_x())
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .map(|sensor| sensor.max_known_x())
        .max()
        .unwrap();

    let mut p1 = 0;

    for x in min_x..=max_x {
        let p = (x, p1_y);

        let data = sensors.iter().fold(Data::Unknown, |folder, sensor| {
            match folder {
                Data::Sensor | Data::Beacon => return folder,
                _ => (),
            }

            match sensor.query(p) {
                Data::Unknown => folder,
                data => data,
            }
        });

        if matches!(data, Data::Empty) {
            p1 += 1;
        }
    }

    for y in 0..=p2_size {
        let mut ranges = Vec::new();

        for sensor in &sensors {
            if let Some(range) = sensor.range_of_x_at_y(y) {
                if range.1 < 0 {
                    continue;
                }

                if range.0 > p2_size {
                    continue;
                }

                let actual_range = (range.0.max(0), range.1.min(p2_size));

                ranges.push(actual_range);
            }
        }

        ranges.sort_by(|a, b| a.0.cmp(&b.0));

        loop {
            let len = ranges.len();

            for i in 0..(len - 1) {
                let r1 = ranges[i + 1];
                let r0 = &mut ranges[i];

                if (r0.0..=r0.1).contains(&r1.0) {
                    r0.0 = r0.0.min(r1.0);
                    r0.1 = r0.1.max(r1.1);
                    ranges.remove(i + 1);
                    break;
                }
            }

            if len == ranges.len() {
                break;
            }
        }

        if ranges.len() != 1 {
            let x = ranges[0].1 + 1;

            let tuning_frequency = x as i64 * 4000000 + y as i64;

            return (Solution::U32(p1), Solution::I64(tuning_frequency));
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    use super::solver;

    static EXAMPLE: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

    #[test]
    fn it_works() {
        let solution = solver(EXAMPLE, 10, 20);

        if let (Solution::U32(p1), Solution::I64(p2)) = solution {
            assert_eq!(p1, 26);
            assert_eq!(p2, 56000011);
        } else {
            panic!();
        }
    }
}
