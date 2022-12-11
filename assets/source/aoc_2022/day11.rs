use std::collections::VecDeque;

use itertools::Itertools;

use crate::common::parse::{parse_numbers, to_non_empty_lines};

pub fn run_first(input: &str) -> String {
    let mut monkeys = parse_monkeys(input);
    run_rounds(&mut monkeys, 20, None).to_string()
}

pub fn run_second(input: &str) -> String {
    let mut monkeys = parse_monkeys(input);
    let product: i64 = monkeys.iter().map(|m| m.test).product();
    run_rounds(&mut monkeys, 10_000, Some(product)).to_string()
}

fn run_rounds(monkeys: &mut Vec<Monkey>, rounds: i64, product: Option<i64>) -> i64 {
    for _ in 0..rounds {
        for m_idx in 0..monkeys.len() {
            let m = &mut monkeys[m_idx];
            let mut items: Vec<(i64, i64)> = vec![];

            while m.has_items() {
                items.push(m.inspect_next(product))
            }

            for (item, idx) in items {
                monkeys[idx as usize].add_item(item)
            }
        }
    }

    monkeys
        .iter()
        .map(|m| m.inspected_count)
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    operation: String,
    test: i64,
    if_true: i64,
    if_false: i64,
    inspected_count: i64,
}

impl Monkey {
    fn has_items(&self) -> bool {
        !self.items.is_empty()
    }

    fn inspect_next(&mut self, product: Option<i64>) -> (i64, i64) {
        let item = self.items.pop_front().unwrap();
        let tokens: Vec<_> = self.operation.split(' ').collect();
        let a = self.compute_value(tokens[3], item);
        let op = tokens[4];
        let b = self.compute_value(tokens[5], item);

        let op_result = match op {
            "*" => a * b,
            "+" => a + b,
            _ => unreachable!("Invalid op: {}", op),
        };

        let rounded = match product {
            None => op_result / 3,
            Some(pr) => op_result % pr,
        };
        let is_divisible = rounded % self.test == 0;
        self.inspected_count += 1;

        if is_divisible {
            (rounded, self.if_true)
        } else {
            (rounded, self.if_false)
        }
    }

    fn compute_value(&self, op_token: &str, item: i64) -> i64 {
        match op_token {
            "old" => item,
            _ => op_token.parse().unwrap(),
        }
    }

    fn add_item(&mut self, item: i64) {
        self.items.push_back(item)
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    to_non_empty_lines(input)
        .chunks(6)
        .map(|chunk| {
            let items = parse_numbers(&chunk[1]).into_iter().collect();
            let operation = &chunk[2];
            let divisible = parse_numbers(&chunk[3]);
            let if_true = parse_numbers(&chunk[4]);
            let if_false = parse_numbers(&chunk[5]);

            Monkey {
                items,
                operation: operation.trim().to_string(),
                test: divisible[0],
                if_true: if_true[0],
                if_false: if_false[0],
                inspected_count: 0,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 11;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "121450";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "28244037010";
        assert_eq!(result, expected.to_string());
    }
}
