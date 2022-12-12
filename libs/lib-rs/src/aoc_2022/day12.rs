use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::common::parse::to_non_empty_lines;

pub fn run_first(input: &str) -> String {
    let mut matrix = parse_matrix(input);
    let start = matrix.update_start();
    let end = matrix.update_end();
    let len = matrix.find_shortest_path_len(start, end);
    len.to_string()
}

pub fn run_second(input: &str) -> String {
    let mut matrix = parse_matrix(input);
    let _start = matrix.update_start();
    let end = matrix.update_end();
    let all_starts = matrix.get_all_starts('a');
    all_starts
        .iter()
        .map(|p| matrix.find_shortest_path_len(*p, end))
        .min()
        .unwrap()
        .to_string()
}

#[derive(Debug)]
pub struct HillMatrix {
    cost_values: Vec<i32>,
    width: i32,
    height: i32,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Point {
    row: i32,
    column: i32,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct HillPoint {
    coords: Point,
    cost: i32,
}

impl PartialOrd<Self> for HillPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HillPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl HillMatrix {
    fn get_first_of_value(&self, val: char) -> Point {
        let idx = self
            .cost_values
            .iter()
            .position(|&p| p == val as i32)
            .unwrap() as i32;
        let col = idx % self.width;
        let row = (idx - col) / self.width;
        Point { row, column: col }
    }

    fn get_all_starts(&self, val: char) -> Vec<Point> {
        self.cost_values
            .iter()
            .enumerate()
            .filter(|(_idx, &v)| v == val as i32)
            .map(|(idx, _v)| {
                let col = idx as i32 % self.width;
                let row = (idx as i32 - col) / self.width;
                Point { row, column: col }
            })
            .collect()
    }

    fn update_start(&mut self) -> Point {
        let point = self.get_first_of_value('S');
        let idx = self.get_index(point.row, point.column);
        self.cost_values[idx] = 'a' as i32;
        point
    }

    fn update_end(&mut self) -> Point {
        let point = self.get_first_of_value('E');
        let idx = self.get_index(point.row, point.column);
        self.cost_values[idx] = 'z' as i32;
        point
    }

    fn get_index(&self, row: i32, column: i32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_height(&self, row: i32, column: i32) -> i32 {
        self.cost_values[self.get_index(row, column)]
    }

    fn get_available_adjacent(&self, point: &HillPoint) -> Vec<HillPoint> {
        let row = point.coords.row;
        let col = point.coords.column;
        let coords = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];

        coords
            .into_iter()
            .filter(|(dr, dc)| {
                let r = row + dr;
                let c = col + dc;
                let oob = self.out_of_bounds(r, c);
                if oob {
                    return false;
                }

                let height = self.get_height(r, c);
                point.cost >= height - 1
            })
            .map(|(dr, dc)| {
                let new_row = row + dr;
                let new_col = col + dc;
                let height = self.get_height(new_row, new_col);
                HillPoint {
                    coords: Point {
                        row: new_row,
                        column: new_col,
                    },
                    cost: height,
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

    fn find_shortest_path_len(&self, start: Point, end: Point) -> i32 {
        let mut came_from: HashMap<Point, Point> = HashMap::new();
        let mut path_len_so_far: HashMap<Point, i32> = HashMap::new();
        let mut priority_queue: BinaryHeap<HillPoint> = BinaryHeap::new();

        let current_point = start;

        let mut current = HillPoint {
            cost: self.get_height(current_point.row, current_point.column),
            coords: current_point,
        };

        priority_queue.push(current);
        *came_from.entry(current_point).or_insert(current_point) = current_point;
        *path_len_so_far.entry(current_point).or_insert(0) = 0;

        while !priority_queue.is_empty() {
            current = priority_queue.pop().unwrap();
            if current.coords == end {
                break;
            }

            for adjacent in self.get_available_adjacent(&current) {
                let new_path_len = path_len_so_far.get(&current.coords).unwrap() + 1;

                if !path_len_so_far.contains_key(&adjacent.coords)
                    || new_path_len < *path_len_so_far.get(&adjacent.coords).unwrap()
                {
                    *path_len_so_far
                        .entry(adjacent.coords)
                        .or_insert(new_path_len) = new_path_len;

                    priority_queue.push(adjacent);
                    *came_from.entry(adjacent.coords).or_insert(current.coords) = current.coords;
                }
            }
        }

        *path_len_so_far.get(&end).unwrap_or(&i32::MAX)
    }
}

fn parse_matrix(input: &str) -> HillMatrix {
    let lines = to_non_empty_lines(input);
    let height = lines.len() as i32;
    let width = lines.first().unwrap().len() as i32;
    let values: Vec<_> = lines
        .iter()
        .flat_map(|l| l.chars().map(|c| c as i32))
        .collect();

    HillMatrix {
        cost_values: values,
        height,
        width,
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 12;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "361";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "354";
        assert_eq!(result, expected.to_string());
    }
}
