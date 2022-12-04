use crate::common::parse::{parse_numbers, to_non_empty_lines};

#[derive(Copy, Clone)]
struct Assignment {
    pub from: i64,
    pub to: i64,
}

impl Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        self.from <= other.from && self.to >= other.from
    }
}

pub fn run_first(input: &str) -> String {
    to_non_empty_lines(input)
        .iter()
        .filter(|line| {
            let assignments = parse_assignments(line);
            let first = assignments[0];
            let second = assignments[1];
            first.contains(&second) || second.contains(&first)
        })
        .count()
        .to_string()
}

pub fn run_second(input: &str) -> String {
    to_non_empty_lines(input)
        .iter()
        .filter(|line| {
            let assignments = parse_assignments(line);
            let first = assignments[0];
            let second = assignments[1];

            first.overlaps(&second) || second.overlaps(&first)
        })
        .count()
        .to_string()
}

fn parse_assignments(line: &str) -> Vec<Assignment> {
    line.split(',')
        .map(|part| {
            let nums = parse_numbers(part);
            Assignment {
                from: nums[0],
                to: nums[1],
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 4;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "532";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "854";
        assert_eq!(result, expected.to_string());
    }
}
