use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::common::parse::to_non_empty_lines;

pub fn run_first(input: &str) -> String {
    let lines = to_non_empty_lines(input);
    let matrix = parse_input(&lines);
    matrix.tl_br_risk().to_string()
}

pub fn run_second(input: &str) -> String {
    let lines = to_non_empty_lines(input);
    let matrix = parse_extended(&lines);
    matrix.tl_br_risk().to_string()
}

#[derive(Debug)]
struct RiskMatrix {
    risk_values: Vec<i32>,
    width: i32,
    height: i32,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point {
    row: i32,
    column: i32,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct RiskPoint {
    coords: Point,
    risk: i32,
}

impl PartialOrd<Self> for RiskPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RiskPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk.cmp(&self.risk)
    }
}

impl RiskMatrix {
    fn get_index(&self, row: i32, column: i32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_risk(&self, row: i32, column: i32) -> i32 {
        self.risk_values[self.get_index(row, column)]
    }

    fn get_adjacent(&self, point: &RiskPoint) -> Vec<RiskPoint> {
        let row = point.coords.row;
        let col = point.coords.column;
        let coords = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];

        coords
            .into_iter()
            .filter(|(dr, dc)| {
                let r = row + dr;
                let c = col + dc;
                let oob = self.out_of_bounds(r, c);
                !oob
            })
            .map(|(dr, dc)| {
                let new_row = row + dr;
                let new_col = col + dc;
                let risk = self.get_risk(new_row, new_col);
                RiskPoint {
                    coords: Point {
                        row: new_row,
                        column: new_col,
                    },
                    risk,
                }
            })
            .collect()
    }

    fn out_of_bounds(&self, row: i32, column: i32) -> bool {
        if row < 0 || row >= self.height {
            return true;
        }
        if column < 0 || column >= self.width {
            return true;
        }
        false
    }

    fn find_risk(&self, start: Point, end: Point) -> i32 {
        let mut came_from: HashMap<Point, Point> = HashMap::new();
        let mut risk_so_far: HashMap<Point, i32> = HashMap::new();
        let mut priority_queue: BinaryHeap<RiskPoint> = BinaryHeap::new();

        let current_point = start;

        let mut current = RiskPoint {
            risk: 0,
            coords: current_point,
        };

        priority_queue.push(current);
        *came_from.entry(current_point).or_insert(current_point) = current_point;
        *risk_so_far.entry(current_point).or_insert(0) = 0;

        while !priority_queue.is_empty() {
            current = priority_queue.pop().unwrap();
            if current.coords == end {
                break;
            }

            for adjacent in self.get_adjacent(&current) {
                // set new risk of adjacent to the cost of path so far + adjacent risk
                let new_risk = risk_so_far.get(&current.coords).unwrap() + adjacent.risk;

                if !risk_so_far.contains_key(&adjacent.coords)
                    || new_risk < *risk_so_far.get(&adjacent.coords).unwrap()
                {
                    *risk_so_far.entry(adjacent.coords).or_insert(new_risk) = new_risk;

                    let priority = new_risk;
                    priority_queue.push(RiskPoint {
                        coords: adjacent.coords,
                        risk: priority,
                    });
                    *came_from.entry(adjacent.coords).or_insert(current.coords) = current.coords;
                }
            }
        }

        *risk_so_far.get(&end).unwrap()
    }

    fn tl_br_risk(&self) -> i32 {
        let top_left: Point = Point { row: 0, column: 0 };
        let bottom_right: Point = Point {
            row: self.height - 1,
            column: self.width - 1,
        };
        self.find_risk(top_left, bottom_right)
    }
}

fn parse_input(input: &[String]) -> RiskMatrix {
    let height = input.len() as i32;
    let width = input
        .iter()
        .next()
        .unwrap()
        .split("")
        .filter(|n| !n.is_empty())
        .count() as i32;

    let values = parse_values(input).iter().flatten().cloned().collect();

    RiskMatrix {
        risk_values: values,
        height,
        width,
    }
}

fn parse_values(input: &[String]) -> Vec<Vec<i32>> {
    input
        .iter()
        .map(|l| {
            l.trim()
                .split("")
                .filter(|n| !n.is_empty())
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

fn parse_extended(input: &[String]) -> RiskMatrix {
    let values = parse_and_extend(input);
    let height = values.len() as i32;
    let width = values[0].len() as i32;
    let flat = values.iter().flatten().cloned().collect();
    RiskMatrix {
        risk_values: flat,
        height,
        width,
    }
}

fn parse_and_extend(input: &[String]) -> Vec<Vec<i32>> {
    let initial = parse_values(input);

    let mut lines: Vec<Vec<i32>> = vec![];

    for idx in 0..5 {
        let shifted: Vec<Vec<i32>> = initial
            .iter()
            .map(|n| n.iter().map(|n| conv(*n, idx)).collect())
            .collect();

        for row in shifted.iter() {
            let mut result: Vec<i32> = vec![];
            for i in 0..5 {
                result.extend(row.iter().map(|n| conv(*n, i)))
            }
            lines.push(result)
        }
    }

    lines
}

fn conv(n: i32, idx: i32) -> i32 {
    let shifted = n + idx;
    if shifted == 9 {
        return shifted;
    }
    (shifted) % 9
}

#[cfg(test)]
mod tests {
    use crate::common::parse::test_utils::vec_of_strings;
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u8 = 15;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "595";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "2914";
        assert_eq!(result, expected.to_string());
    }

    fn mock_input() -> Vec<String> {
        vec_of_strings![
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581"
        ]
    }

    #[test]
    fn test_risk_1() {
        let input = mock_input();
        let mat = parse_input(&input);
        let risk = mat.tl_br_risk();
        assert_eq!(risk, 40);
    }

    #[test]
    fn test_risk_extended() {
        let input = mock_input();
        let mat = parse_extended(&input);
        let risk = mat.tl_br_risk();
        assert_eq!(risk, 315);
    }
}
