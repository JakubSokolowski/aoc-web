use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;

pub fn run_first(input: &str) -> String {
    let jets = parse_jets(input);
    let mut tetris = Rocktris::new();
    tetris.drop_rocks(2022, &jets);
    tetris.max_height().to_string()
}

pub fn run_second(input: &str) -> String {
    let jets = parse_jets(input);
    let num_rocks = 1_000_000_000_000_i64;

    // Magic numbers
    let cycle_start = 1713;
    let height_at_start = 2651;
    let cycle_increase = 2660;
    let rocks_per_cycle = 1700;

    let num_left_after_start = num_rocks - cycle_start;
    let num_left_after_cycles = num_left_after_start % rocks_per_cycle;
    let num_cycles = (num_left_after_start - num_left_after_cycles) / rocks_per_cycle;
    let cycles_height = num_cycles * cycle_increase;

    let mut tetris = Rocktris::new();

    // After initial part without cycle and the cycle itself there will be some trailing rocks
    // We can't just take that number and calculate the height from start, because we don't know
    // what will be the "bottom" and rocks may fall differently
    // However, we know the bottom is the same after each cycle, so just calculate the height
    // for start_rocks + 1 cycle rocks + trailing rocks
    // the bottom between cycle and trailing will be correct
    tetris.drop_rocks(num_left_after_cycles + rocks_per_cycle + cycle_start, &jets);
    // The height will be too high, so we need to subtract the start height and cycle height
    let height_after_cycles = tetris.max_height() - height_at_start - cycle_increase;
    let sum = height_at_start + cycles_height + height_after_cycles;
    sum.to_string()
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
struct Shape {
    points: Vec<Point>,
}

impl Shape {
    fn move_by(&self, vector: (i64, i64)) -> Shape {
        let new_points = self.points.iter().map(|p| p.move_by(vector)).collect();

        Shape { points: new_points }
    }
}

#[derive(Debug)]
struct Rocktris {
    rocks: HashSet<Point>,
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Push,
    Fall,
}

impl Rocktris {
    pub fn new() -> Self {
        Self {
            rocks: HashSet::new(),
        }
    }

    fn drop_rocks(&mut self, count: i64, jets: &[i64]) {
        let shapes = all_shapes();
        let max_width = 7;
        let left_wall_offset = 2;
        let bottom_offset = 3;
        let mut bottom = 0;

        let mut jet_idx = 0;
        let mut action = Action::Push;

        for i in 0..count {
            let shape_idx = i as usize % shapes.len();
            let shape: &Shape = &shapes[shape_idx];
            let initial = shape.move_by((left_wall_offset, bottom_offset + bottom));
            let mut curr = initial;
            loop {
                match action {
                    Action::Push => {
                        let idx = jet_idx % jets.len();
                        jet_idx += 1;
                        let jet_push_x_delta = &jets[idx];
                        let after_push = curr.move_by((*jet_push_x_delta, 0));

                        if self.can_be_pushed(&after_push, max_width) {
                            curr = after_push;
                        }
                        action = Action::Fall;
                    }
                    Action::Fall => {
                        let after_fall = curr.move_by((0, -1));

                        if self.can_fall_down(&after_fall) {
                            curr = after_fall;
                            action = Action::Push;
                        } else {
                            self.place_shape(&curr);
                            action = Action::Push;
                            break;
                        }
                    }
                }
            }
            bottom = self.max_height();
        }
    }

    fn max_height(&self) -> i64 {
        self.rocks.iter().map(|r| r.y).max().unwrap_or(0) + 1
    }

    fn place_shape(&mut self, shape: &Shape) {
        shape.points.iter().for_each(|p| {
            self.rocks.insert(*p);
        });
    }

    fn can_be_pushed(&self, shape: &Shape, max_x: i64) -> bool {
        shape.points.iter().all(|p| {
            let is_in_wall_bounds = (0..max_x).contains(&p.x);
            if !is_in_wall_bounds {
                return false;
            }
            !self.rocks.contains(p)
        })
    }

    fn can_fall_down(&self, shape: &Shape) -> bool {
        shape.points.iter().all(|p| {
            let is_above_rock_bottom = p.y >= 0;
            if !is_above_rock_bottom {
                return false;
            }
            !self.rocks.contains(p)
        })
    }
}

fn points_to_string(points: &HashSet<Point>, max_height: i64) -> String {
    let mut display_str = "Rocktris:\n".to_string();
    for y in (0..max_height).rev() {
        let mut line = "|".to_string();
        for x in 0..7 {
            let point = Point { x, y };
            if points.contains(&point) {
                line += "#";
            } else {
                line += " ";
            }
        }
        display_str += &*format!("{}|\n", line);
    }
    display_str += "+-------+";
    display_str
}

impl fmt::Display for Rocktris {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let display_str = points_to_string(&self.rocks, self.max_height() + 3);
        write!(f, "{}", display_str)
    }
}

fn all_shapes() -> Vec<Shape> {
    vec![
        // Horizontal Line
        Shape {
            points: vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 },
            ],
        },
        // Cross
        Shape {
            points: vec![
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
                Point { x: 1, y: 0 },
                Point { x: 1, y: 2 },
            ],
        },
        // L
        Shape {
            points: vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 2, y: 1 },
                Point { x: 2, y: 2 },
            ],
        },
        // Vertical Line
        Shape {
            points: vec![
                Point { x: 0, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 0, y: 2 },
                Point { x: 0, y: 3 },
            ],
        },
        // Fat square
        Shape {
            points: vec![
                Point { x: 0, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 0 },
                Point { x: 1, y: 1 },
            ],
        },
    ]
}

fn parse_jets(input: &str) -> Vec<i64> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => 1,
            '<' => -1,
            _ => panic!("Unexpected character: {}", c),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 17;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "3137";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "1564705882327";
        assert_eq!(result, expected.to_string());
    }
}
