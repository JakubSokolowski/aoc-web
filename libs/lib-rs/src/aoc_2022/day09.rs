use std::collections::HashSet;

use crate::common::parse::to_non_empty_lines;

type Position = (i64, i64);

pub fn run_first(input: &str) -> String {
    rope(input, 2)
}

pub fn run_second(input: &str) -> String {
    rope(input, 10)
}

pub fn rope(input: &str, length: usize) -> String {
    let cmds = parse_commands(input);
    let mut rope: Vec<Position> = (0..length).map(|_| (0, 0)).collect();
    let mut visited: HashSet<Position> = HashSet::new();

    for cmd in cmds {
        let (direction, steps) = cmd;
        let (x_delta, y_delta) = step_delta(&direction);
        for _step in 0..steps {
            rope[0] = (rope[0].0 + x_delta, rope[0].1 + y_delta);
            for idx in 1..rope.len() {
                let delta = move_delta(&rope[idx - 1], &rope[idx]);
                rope[idx] = (rope[idx].0 + delta.0, rope[idx].1 + delta.1);
            }
            visited.insert(rope[rope.len() - 1]);
        }
    }
    visited.len().to_string()
}

fn move_delta(head: &Position, tail: &Position) -> Position {
    let delta_x = head.0 - tail.0;
    let delta_y = head.1 - tail.1;
    if (-1..=1).contains(&delta_x) && (-1..=1).contains(&delta_y) {
        (0, 0)
    } else {
        (delta_x.clamp(-1, 1), delta_y.clamp(-1, 1))
    }
}

pub fn step_delta(direction: &str) -> (i64, i64) {
    match direction {
        "U" => (0, 1),
        "D" => (0, -1),
        "R" => (1, 0),
        "L" => (-1, 0),
        _ => unreachable!("Invalid command: {:?}", direction),
    }
}

pub fn parse_commands(input: &str) -> Vec<(String, i64)> {
    to_non_empty_lines(input)
        .iter()
        .map(|l| {
            let tokens: Vec<_> = l.split(' ').filter(|t| !t.is_empty()).collect();
            (
                tokens.first().unwrap().to_string(),
                tokens.last().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 9;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "5930";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "2443";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_example_part_1() {
        // given

        let data = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        // when
        let result = run_first(&data);

        // then
        let expected = "13";
        assert_eq!(result, expected.to_string());
    }
}
