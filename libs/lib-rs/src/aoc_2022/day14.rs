use std::collections::HashMap;

use crate::common::parse::to_non_empty_lines;

pub fn run_first(input: &str) -> String {
    let mut rockfall = parse_input(input);
    let sand_source = Point { x: 500, y: 0 };
    rockfall.count_till_abyss(&sand_source).to_string()
}

pub fn run_second(input: &str) -> String {
    let mut rockfall = parse_input(input);
    let sand_source = Point { x: 500, y: 0 };
    rockfall.count_till_fill(&sand_source).to_string()
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn move_by(&self, vector: (i64, i64)) -> Point {
        let (x, y) = vector;
        Point {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

#[derive(Debug)]
struct Rockfall {
    filled: HashMap<Point, Fill>,
}

#[derive(Debug)]
enum Fill {
    Rock,
    Sand,
}

#[derive(Debug)]
enum DropResult {
    Rested,
    Abyss,
}

impl Rockfall {
    fn new() -> Self {
        Rockfall {
            filled: HashMap::new(),
        }
    }

    fn count_till_abyss(&mut self, source: &Point) -> i64 {
        let deepest = self.deepest();
        let mut count = 0;
        while let DropResult::Rested = self.drop_into_abyss(source, &deepest) {
            count += 1;
        }
        count
    }

    fn count_till_fill(&mut self, source: &Point) -> i64 {
        let deepest = self.deepest();
        let mut count = 0;
        loop {
            let point = self.drop_on_the_floor(source, &deepest);
            count += 1;
            if point == *source {
                break;
            }
        }
        count
    }

    fn drop_on_the_floor(&mut self, source: &Point, deepest: &Point) -> Point {
        let mut curr = *source;

        loop {
            if curr.y == deepest.y + 1 {
                self.filled.insert(curr, Fill::Sand);
                return curr;
            }

            let down = curr.move_by((0, 1));
            let diag_left = curr.move_by((-1, 1));
            let diag_right = curr.move_by((1, 1));

            if !self.filled.contains_key(&down) {
                curr = down;
                continue;
            }

            if !self.filled.contains_key(&diag_left) {
                curr = diag_left;
                continue;
            }

            if !self.filled.contains_key(&diag_right) {
                curr = diag_right;
                continue;
            }

            self.filled.insert(curr, Fill::Sand);
            return curr;
        }
    }

    fn drop_into_abyss(&mut self, source: &Point, deepest: &Point) -> DropResult {
        let mut curr = *source;

        loop {
            if curr.y > deepest.y {
                return DropResult::Abyss;
            }
            let down = curr.move_by((0, 1));
            let diag_left = curr.move_by((-1, 1));
            let diag_right = curr.move_by((1, 1));

            if !self.filled.contains_key(&down) {
                curr = down;
                continue;
            }

            if !self.filled.contains_key(&diag_left) {
                curr = diag_left;
                continue;
            }

            if !self.filled.contains_key(&diag_right) {
                curr = diag_right;
                continue;
            }

            self.filled.insert(curr, Fill::Sand);
            return DropResult::Rested;
        }
    }

    fn fill_line(&mut self, from: &Point, to: &Point) {
        let x_delta = (to.x - from.x).signum();
        let y_delta = (to.y - from.y).signum();
        let len = i64::max((to.x - from.x).abs(), (to.y - from.y).abs());

        for n in 0..=len {
            let x = from.x + n * x_delta;
            let y = from.y + n * y_delta;
            self.filled.insert(Point { x, y }, Fill::Rock);
        }
    }

    fn deepest(&self) -> Point {
        *self
            .filled
            .iter()
            .max_by(|&a, &b| a.0.y.cmp(&b.0.y))
            .unwrap()
            .0
    }
}

fn parse_input(input: &str) -> Rockfall {
    let lines = to_non_empty_lines(input);

    let mut rockfall = Rockfall::new();

    for line in lines {
        let points: Vec<Point> = line
            .split(" -> ")
            .filter(|t| !t.is_empty())
            .map(|t| {
                let coord: Vec<_> = t.split(',').collect();
                Point {
                    x: coord.first().unwrap().parse().unwrap(),
                    y: coord.last().unwrap().parse().unwrap(),
                }
            })
            .collect();

        points
            .windows(2)
            .for_each(|p| rockfall.fill_line(&p[0], &p[1]));
    }

    rockfall
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 14;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "1068";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "27936";
        assert_eq!(result, expected.to_string());
    }
}
