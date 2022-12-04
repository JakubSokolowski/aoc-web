use std::cmp::Ordering;

use itertools::Itertools;

pub fn run_first(input: &str) -> String {
    let trimmed = input.trim();
    let area = parse_area(trimmed);
    let res = find_max_vel(&area);
    res.1.to_string()
}

pub fn run_second(input: &str) -> String {
    let trimmed = input.trim();
    let area = parse_area(trimmed);
    count_reachable(&area).to_string()
}

pub struct Position {
    x: i32,
    y: i32,
}

pub struct Probe {
    pos: Position,
    x_vel: i32,
    y_vel: i32,
}

impl Probe {
    pub fn new(x_vel: i32, y_vel: i32) -> Probe {
        Probe {
            pos: Position { x: 0, y: 0 },
            x_vel,
            y_vel,
        }
    }

    pub fn step(&mut self) {
        self.pos.x += self.x_vel;
        self.pos.y += self.y_vel;
        self.x_vel = apply_drag(self.x_vel);
        self.y_vel = apply_gravity(self.y_vel);
    }
}

fn apply_drag(x_vel: i32) -> i32 {
    match x_vel.cmp(&0) {
        Ordering::Less => x_vel + 1,
        Ordering::Equal => x_vel,
        Ordering::Greater => x_vel - 1,
    }
}

fn apply_gravity(y_vel: i32) -> i32 {
    y_vel - 1
}

fn find_max_y(x_vel: i32, y_vel: i32, area: &Area) -> Option<i32> {
    let mut probe = Probe::new(x_vel, y_vel);
    let mut y_max = 0;

    loop {
        probe.step();
        let curr_y = probe.pos.y;

        if curr_y > y_max {
            y_max = curr_y;
        }

        if is_in_area(area, &probe.pos) {
            break;
        }

        if ngmi(area, &probe.pos) {
            return None;
        }
    }

    Some(y_max)
}

fn find_max_vel(area: &Area) -> ((i32, i32), i32) {
    let x_bound = area.x_start;
    let y_bound = 1000;
    let mut y_max = 0;
    let mut velocities = (0, 0);

    for (x_vel, y_vel) in (1..x_bound).cartesian_product(1..y_bound) {
        let y = find_max_y(x_vel, y_vel, area).unwrap_or(0);
        if y > y_max {
            y_max = y;
            velocities = (x_vel, y_vel)
        }
    }

    (velocities, y_max)
}

fn count_reachable(area: &Area) -> i32 {
    let x_bound = 1000;
    let y_bound = 1000;
    let mut reachable = 0;

    for (x_vel, y_vel) in (1..x_bound).cartesian_product(-y_bound..y_bound) {
        if find_max_y(x_vel, y_vel, area).is_some() {
            reachable += 1
        }
    }

    reachable
}

pub struct Area {
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
}

fn is_in_area(area: &Area, position: &Position) -> bool {
    in_range(position.x, area.x_start, area.x_end) && in_range(position.y, area.y_end, area.y_start)
}

fn ngmi(area: &Area, position: &Position) -> bool {
    position.x > area.x_end || position.y < area.y_end
}

fn in_range(num: i32, from: i32, to: i32) -> bool {
    num >= from && num <= to
}

fn parse_area(input: &str) -> Area {
    let stripped: Vec<_> = input[13..].split(", ").collect();
    let x_tokens: Vec<i32> = stripped[0]
        .replace("x=", "")
        .split("..")
        .map(|t| t.parse().unwrap())
        .collect();

    let y_tokens: Vec<i32> = stripped[1]
        .replace("y=", "")
        .split("..")
        .map(|t| t.parse().unwrap())
        .collect();

    Area {
        x_start: x_tokens[0],
        x_end: x_tokens[1],
        y_start: y_tokens[1],
        y_end: y_tokens[0],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_to_string;

    const YEAR: u32 = 2021;
    const DAY: u8 = 17;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "5671";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "4556";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_max_y() {
        let area = Area {
            x_start: 20,
            x_end: 30,
            y_start: -5,
            y_end: -10,
        };
        let max = find_max_y(7, 2, &area).unwrap();
        assert_eq!(max, 3);
    }

    #[test]
    fn test_max_2() {
        let area = Area {
            x_start: 20,
            x_end: 30,
            y_start: -5,
            y_end: -10,
        };
        let max = find_max_y(9, 0, &area).unwrap();
        assert_eq!(max, 0);
    }

    #[test]
    fn test_max_3() {
        let area = Area {
            x_start: 20,
            x_end: 30,
            y_start: -5,
            y_end: -10,
        };
        let max = find_max_y(6, 3, &area).unwrap();
        assert_eq!(max, 6);
    }

    #[test]
    fn test_max_4() {
        let area = Area {
            x_start: 20,
            x_end: 30,
            y_start: -5,
            y_end: -10,
        };
        let max = find_max_y(6, 9, &area).unwrap();
        assert_eq!(max, 45);
    }

    #[test]
    fn test_find_vel() {
        let area = Area {
            x_start: 20,
            x_end: 30,
            y_start: -5,
            y_end: -10,
        };
        let max = find_max_vel(&area);
        let expected = ((6, 9), 45);
        assert_eq!(max, expected);
    }

    #[test]
    fn test_count_reachable() {
        let area = Area {
            x_start: 20,
            x_end: 30,
            y_start: -5,
            y_end: -10,
        };
        let max = count_reachable(&area);
        let expected = 112;
        assert_eq!(max, expected);
    }
}
