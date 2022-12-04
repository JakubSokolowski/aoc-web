use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;
use std::iter::FromIterator;

pub fn run_first(input: &str) -> String {
    first_fold(input).to_string()
}

pub fn run_second(input: &str) -> String {
    code(input)
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point {
    row: usize,
    column: usize,
}

impl Point {
    fn fold_x(&self, value: usize) -> Point {
        Point {
            row: self.row,
            column: self.column - 2 * (self.column - value),
        }
    }
    fn fold_y(&self, value: usize) -> Point {
        Point {
            row: self.row - 2 * (self.row - value),
            column: self.column,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Axis {
    X = 0,
    Y = 1,
}

struct Fold {
    axis: Axis,
    value: usize,
}

struct Paper {
    points: HashSet<Point>,
}

impl Paper {
    fn new(points: &[Point]) -> Paper {
        Paper {
            points: HashSet::from_iter(points.iter().cloned()),
        }
    }

    fn apply_fold(&mut self, fold: &Fold) {
        match fold.axis {
            Axis::X => self.apply_x_fold(fold.value),
            Axis::Y => self.apply_y_fold(fold.value),
        }
    }

    fn apply_x_fold(&mut self, value: usize) {
        // Remove folded points
        let points_to_fold: Vec<_> = self
            .points
            .iter()
            .cloned()
            .filter(|p| p.column > value)
            .collect();
        let folded: Vec<_> = points_to_fold.iter().map(|p| p.fold_x(value)).collect();

        self.remove_points(points_to_fold);

        // Add points after fold
        self.add_points(folded);
    }

    fn remove_points(&mut self, points: Vec<Point>) {
        for p in points {
            self.points.remove(&p);
        }
    }

    fn add_points(&mut self, points: Vec<Point>) {
        for p in points {
            self.points.insert(p);
        }
    }

    fn apply_y_fold(&mut self, value: usize) {
        let points_to_fold: Vec<_> = self
            .points
            .iter()
            .cloned()
            .filter(|p| p.row > value)
            .collect();
        let folded: Vec<_> = points_to_fold.iter().map(|p| p.fold_y(value)).collect();

        self.remove_points(points_to_fold);
        self.add_points(folded);
    }

    fn count_dots(&self) -> usize {
        self.points.len()
    }

    fn max_row(&self) -> usize {
        self.points.iter().map(|p| p.row).max().unwrap()
    }

    fn max_column(&self) -> usize {
        self.points.iter().map(|p| p.column).max().unwrap()
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut display_str = "".to_string();
        for row in 0..=self.max_row() {
            let mut line = "".to_string();
            for column in 0..=self.max_column() {
                if self.points.contains(&Point { row, column }) {
                    line += "##"
                } else {
                    line += "  "
                }
            }
            display_str += &*format!("{}\n", line);
        }

        write!(f, "{}", display_str)
    }
}

fn first_fold(input: &str) -> usize {
    let (mut paper, folds) = parse_input(input);
    paper.apply_fold(&folds[0]);
    paper.count_dots()
}

fn code(input: &str) -> String {
    let (mut paper, folds) = parse_input(input);
    for fold in &folds {
        paper.apply_fold(fold);
    }
    println!("{}", paper);
    paper.to_string()
}

fn parse_input(input: &str) -> (Paper, Vec<Fold>) {
    let mut parts = input.split("\n\n");

    let points: Vec<Point> = parts
        .next()
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            let nums: Vec<usize> = l.split(',').map(|n| n.trim().parse().unwrap()).collect();
            Point {
                column: nums[0],
                row: nums[1],
            }
        })
        .collect();

    let paper = Paper::new(&points);

    let folds = parts
        .next()
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            let fold_token = l.split(' ').filter(|l| !l.is_empty()).nth(2).unwrap();
            let parts: Vec<_> = fold_token.split('=').collect();
            let axis = match parts[0] {
                "y" => Axis::Y,
                "x" => Axis::X,
                &_ => unreachable!(),
            };

            let value: usize = parts[1].parse().unwrap();
            Fold { axis, value }
        })
        .collect();

    (paper, folds)
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u8 = 13;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "701";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_first_fold() {
        // given
        let input = "
            6,10
            0,14
            9,10
            0,3
            10,4
            4,11
            6,0
            6,12
            4,1
            0,13
            10,12
            3,4
            3,0
            8,4
            1,10
            2,14
            8,10
            9,0

            fold along y=7
            fold along x=5";

        // when
        let count = first_fold(input);

        // then
        assert_eq!(count, 17);
    }
}
