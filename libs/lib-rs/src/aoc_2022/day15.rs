use std::collections::HashSet;

use crate::common::parse::{parse_signed_numbers, to_non_empty_lines};

pub fn run_first(input: &str) -> String {
    let mut zone = parse_zone(input);
    zone.count_invalid(2_000_000).to_string()
}

pub fn run_second(input: &str) -> String {
    let zone = parse_zone(input);
    zone.tuning_freq(4_000_000).to_string()
}

#[derive(Hash, Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Zone {
    sensors: Vec<(i64, i64, i64)>,
    grid: HashSet<(i64, i64)>,
}

impl Zone {
    fn count_invalid(&mut self, target_y: i64) -> i64 {
        let mut invalid = 0;

        for (x, y, distance) in &self.sensors {
            for dx in x - distance..x + distance {
                let dx_distance = (x - dx).abs() + (target_y - y).abs();
                let target = (dx, target_y);
                if dx_distance <= *distance && self.grid.get(&target).is_none() {
                    self.grid.insert(target);
                    invalid += 1;
                }
            }
        }

        invalid
    }

    fn tuning_freq(&self, limit: i64) -> i64 {
        let range = 0..limit;

        for (sx, sy, sd) in &self.sensors {
            for j in 0..*sd {
                let k = sd - j + 1;
                let deltas = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
                for (xd, yd) in deltas {
                    let curr_x = sx + j * xd;
                    let curr_y = sy + k * yd;

                    if !(range.contains(&curr_x) && range.contains(&curr_y)) {
                        continue;
                    }
                    if self
                        .sensors
                        .iter()
                        .any(|s| is_in_range(s, &(curr_x, curr_y)))
                    {
                        continue;
                    }
                    return 4_000_000 * curr_x + curr_y;
                }
            }
        }

        panic!("Did not found")
    }
}

fn is_in_range(s: &(i64, i64, i64), p: &(i64, i64)) -> bool {
    let d = md(&(s.0, s.1), p);
    d <= s.2
}

fn md(a: &(i64, i64), b: &(i64, i64)) -> i64 {
    let (ax, ay) = a;
    let (bx, by) = b;
    (bx - ax).abs() + (by - ay).abs()
}

fn parse_zone(input: &str) -> Zone {
    let lines = to_non_empty_lines(input);
    let mut sensors: Vec<(i64, i64, i64)> = vec![];
    let mut grid: HashSet<(i64, i64)> = HashSet::new();

    lines.iter().for_each(|line| {
        let nums = parse_signed_numbers(line);
        let s = (nums[0], nums[1]);
        let b = (nums[2], nums[3]);
        let distance = md(&s, &b);
        sensors.push((s.0, s.1, distance));
        grid.insert(s);
    });

    Zone { sensors, grid }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 15;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "5040644";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "11016575214126";
        assert_eq!(result, expected.to_string());
    }
}
