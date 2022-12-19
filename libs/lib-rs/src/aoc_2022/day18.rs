use std::collections::{HashSet, VecDeque};

use crate::common::parse::{parse_numbers, to_non_empty_lines};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    fn move_by(&self, vector: &(i64, i64, i64)) -> Point3D {
        let (x, y, z) = vector;

        Point3D {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
        }
    }

    fn min(&self) -> i64 {
        i64::min(self.x, i64::min(self.y, self.z))
    }

    fn max(&self) -> i64 {
        i64::max(self.x, i64::max(self.y, self.z))
    }

    fn in_bounds(&self, min: i64, max: i64) -> bool {
        let range = min..=max;
        range.contains(&self.x) && range.contains(&self.y) && range.contains(&self.z)
    }

    fn surrounding(&self) -> Vec<Point3D> {
        let deltas: Vec<(i64, i64, i64)> = vec![
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ];

        deltas.iter().map(|d| self.move_by(d)).collect()
    }
}

pub fn run_first(input: &str) -> String {
    let points = parse_input(input);
    surface_area(&points).to_string()
}

fn surface_area(points: &HashSet<Point3D>) -> i64 {
    let deltas: Vec<(i64, i64, i64)> = vec![
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    points
        .iter()
        .map(|p| {
            deltas
                .iter()
                .filter(|&d| !points.contains(&p.move_by(d)))
                .count() as i64
        })
        .sum()
}

fn exterior_surface_area(points: &HashSet<Point3D>) -> i64 {
    let min = points.iter().map(|p| p.min()).min().unwrap() - 1;
    let max = points.iter().map(|p| p.max()).max().unwrap() + 1;

    let mut surface_size = 0_i64;
    let mut seen: HashSet<Point3D> = HashSet::new();
    let mut to_check: VecDeque<Point3D> = VecDeque::new();

    seen.insert(Point3D {
        x: min,
        y: min,
        z: min,
    });
    to_check.push_back(Point3D {
        x: min,
        y: min,
        z: min,
    });

    while !to_check.is_empty() {
        let curr = to_check.pop_front().unwrap();
        let surrounding: HashSet<_> = curr
            .surrounding()
            .into_iter()
            .filter(|p| p.in_bounds(min, max) && !seen.contains(p))
            .collect();

        let intersecting: HashSet<_> = surrounding
            .iter()
            .cloned()
            .filter(|p| points.contains(p))
            .collect();
        surface_size += intersecting.len() as i64;

        for point in surrounding.difference(&intersecting) {
            seen.insert(*point);
            to_check.push_back(*point);
        }
    }

    surface_size
}

pub fn run_second(input: &str) -> String {
    let points = parse_input(input);
    exterior_surface_area(&points).to_string()
}

fn parse_input(input: &str) -> HashSet<Point3D> {
    to_non_empty_lines(input)
        .iter()
        .map(|l| {
            let nums = parse_numbers(l);

            Point3D {
                x: nums[0],
                y: nums[1],
                z: nums[2],
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 18;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "3576";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "2066";
        assert_eq!(result, expected.to_string());
    }
}
