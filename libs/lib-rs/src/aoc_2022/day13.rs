use crate::common::parse::{parse_numbers, to_non_empty_lines};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::VecDeque;

pub fn run_first(input: &str) -> String {
    let pairs = parse_pairs(input);

    pairs
        .iter()
        .enumerate()
        .map(|(idx, (left, right))| (idx, compare(left, right)))
        .filter(|&(_idx, is_in_order)| is_in_order == 1)
        .map(|(idx, _)| idx + 1)
        .sum::<usize>()
        .to_string()
}

pub fn run_second(input: &str) -> String {
    let mut packets = to_non_empty_lines(input);
    packets.push("[[2]]".to_string());
    packets.push("[[6]]".to_string());

    let sorted: Vec<_> = packets
        .iter()
        .sorted_by(|a, b| match compare(a, b) {
            1 => Ordering::Less,
            -1 => Ordering::Greater,
            _ => Ordering::Equal,
        })
        .collect();

    let first_pos = sorted.iter().position(|&p| p == "[[2]]").unwrap() + 1;
    let second_pos = sorted.iter().position(|&p| p == "[[6]]").unwrap() + 1;

    (first_pos * second_pos).to_string()
}

#[derive(Debug)]
pub enum Type {
    Integer,
    List,
}

pub fn compare(left: &str, right: &str) -> i32 {
    let left_type = get_type(left);
    let right_type = get_type(right);

    match (left_type, right_type) {
        (Type::Integer, Type::Integer) => {
            let left_nums = parse_numbers(left);
            let right_nums = parse_numbers(right);
            let l = left_nums.first().unwrap();
            let r = right_nums.first().unwrap();
            if l == r {
                return 0;
            }
            if l < r {
                1
            } else {
                -1
            }
        }
        (Type::Integer, Type::List) => compare(&format!("[{}]", left), right),
        (Type::List, Type::Integer) => compare(left, &format!("[{}]", right)),
        _ => {
            let mut left_children = get_children(left);
            let mut right_children = get_children(right);
            while !left_children.is_empty() || !right_children.is_empty() {
                if left_children.is_empty() {
                    return 1;
                }
                if right_children.is_empty() {
                    return -1;
                }
                let curr_left = left_children.pop_front().unwrap();
                let curr_right = right_children.pop_front().unwrap();
                let res = compare(&curr_left, &curr_right);
                if res != 0 {
                    return res;
                }
            }
            0
        }
    }
}

pub fn get_type(part: &str) -> Type {
    if part.contains('[') {
        return Type::List;
    }
    Type::Integer
}

pub fn get_children(part: &str) -> VecDeque<String> {
    let without_braces: String = part[1..part.len() - 1].to_string();

    let mut open_count = 0;
    let mut indices: VecDeque<usize> = VecDeque::new();
    for (idx, c) in without_braces.chars().enumerate() {
        match c {
            ',' => {
                if open_count == 0 {
                    indices.push_back(idx)
                }
            }
            '[' => {
                open_count += 1;
            }
            ']' => {
                open_count -= 1;
            }
            _ => {
                continue;
            }
        }
    }

    let mut result: VecDeque<String> = VecDeque::new();
    let mut prev_idx = 0;

    for idx in indices {
        let val = without_braces[prev_idx..idx].to_string();
        result.push_back(val);
        prev_idx = idx + 1;
    }

    result.push_back(without_braces[prev_idx..without_braces.len()].to_string());

    result.into_iter().filter(|i| !i.is_empty()).collect()
}

pub fn parse_pairs(input: &str) -> Vec<(String, String)> {
    let lines = to_non_empty_lines(input);

    lines
        .chunks(2)
        .map(|c| (c[0].clone(), c[1].clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 13;
    use crate::common::parse::test_utils::vec_of_strings;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "6240";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "23142";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_get_children() {
        assert_eq!(
            get_children("[[1],[2,3,4]]"),
            vec_of_strings!["[1]", "[2,3,4]"]
        );
    }

    #[test]
    fn test_compare() {
        assert_eq!(compare("[1,1,3,1,1]", "[1,1,5,1,1]"), 1);
        assert_eq!(compare("[[1],[2,3,4]]", "[[1],4]"), 1);
        assert_eq!(compare("[3]", "[3]"), 0);
        assert_eq!(compare("[9]", "[[8,7,6]]"), -1);
        assert_eq!(compare("[[4,4],4,4]", "[[4,4],4,4,4]"), 1);
        assert_eq!(compare("[7,7,7,7]", "[7,7,7]"), -1);
        assert_eq!(compare("[0]", "[3]"), 1);
        assert_eq!(compare("[[[]]]", "[[]]"), -1);
        assert_eq!(
            compare("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"),
            -1
        );
    }
}
