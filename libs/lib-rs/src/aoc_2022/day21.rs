use crate::common::parse::to_non_empty_lines;
use std::collections::HashMap;

pub fn run_first(input: &str) -> String {
    let monkeys = parse_monkeys(input);
    solve(&monkeys, "root").to_string()
}

pub fn run_second(input: &str) -> String {
    let monkeys = parse_monkeys(input);
    bounded_solve(&monkeys).to_string()
}

fn bounded_solve(monkeys: &Vec<Monkey>) -> i64 {
    let mut delta = 100_000_000_000;
    let mut bound = 0;
    let mut sign = 1;

    let mut solution = 0;

    'outer: while delta >= 1 {
        for num in 1..=10 {
            let next = bound + num * delta * sign;
            let result = solve_for_eq(monkeys, "root", next);

            if result == 0 {
                solution = next;
                break 'outer;
            }

            if sign.signum() != result.signum() {
                bound = next;
                break;
            }
            bound = next;
        }

        if delta > 1 {
            delta /= 10;
        }

        sign *= -1;
    }

    solution
}

fn solve(monkeys: &Vec<Monkey>, name: &str) -> i64 {
    let mut solved: HashMap<_, _> = monkeys
        .iter()
        .filter(|m| m.value.is_some())
        .map(|m| (m.name.clone(), m.value.unwrap()))
        .collect();

    while solved.get(name).is_none() {
        for m in monkeys {
            if solved.contains_key(&m.name) {
                continue;
            }
            let can_solve = m.needs.iter().all(|n| solved.contains_key(n));
            if can_solve {
                let left = *solved.get(&m.needs[0]).unwrap();
                let right = *solved.get(&m.needs[1]).unwrap();

                let result = match m.operation {
                    '+' => left + right,
                    '-' => left - right,
                    '*' => left * right,
                    '/' => left / right,
                    _ => panic!("Unexpected operation: {}", m.operation),
                };

                solved.insert(m.name.clone(), result);
            }
        }
    }

    *solved.get(name).unwrap()
}

fn solve_for_eq(monkeys: &Vec<Monkey>, name: &str, number: i64) -> i64 {
    let mut solved: HashMap<_, _> = monkeys
        .iter()
        .filter(|m| m.value.is_some())
        .map(|m| {
            if m.name == "humn" {
                (m.name.clone(), number)
            } else {
                (m.name.clone(), m.value.unwrap())
            }
        })
        .collect();

    while solved.get(name).is_none() {
        for m in monkeys {
            if solved.contains_key(&m.name) {
                continue;
            }
            let can_solve = m.needs.iter().all(|n| solved.contains_key(n));
            if can_solve {
                let left = *solved.get(&m.needs[0]).unwrap();
                let right = *solved.get(&m.needs[1]).unwrap();

                let result = match m.operation {
                    '+' => left + right,
                    '-' => left - right,
                    '*' => left * right,
                    '/' => left / right,
                    _ => panic!("Unexpected operation: {}", m.operation),
                };

                solved.insert(m.name.clone(), result);
            }
        }
    }

    let root = monkeys.iter().find(|m| m.name == "root").unwrap();
    let left = *solved.get(&root.needs[0]).unwrap();
    let right = *solved.get(&root.needs[1]).unwrap();

    left - right
}

#[derive(Debug)]
struct Monkey {
    name: String,
    value: Option<i64>,
    needs: Vec<String>,
    operation: char,
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let lines = to_non_empty_lines(input);

    lines
        .iter()
        .map(|l| {
            let tokens: Vec<_> = l.split(": ").collect();

            let name = tokens[0];
            let op_tokens: Vec<_> = tokens[1].split(' ').filter(|t| !t.is_empty()).collect();

            match op_tokens.len() {
                1 => {
                    let value = op_tokens[0].parse::<i64>().unwrap();
                    Monkey {
                        name: name.to_string(),
                        needs: vec![],
                        value: Some(value),
                        operation: ' ',
                    }
                }
                3 => {
                    let left_op = op_tokens[0];
                    let op = op_tokens[1];
                    let right_op = op_tokens[2];

                    Monkey {
                        name: name.to_string(),
                        needs: vec![left_op.to_string(), right_op.to_string()],
                        value: None,
                        operation: op.chars().next().unwrap(),
                    }
                }
                _ => panic!("Unexpected tokens: {op_tokens:?}"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 21;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "145167969204648";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "3330805295850";
        assert_eq!(result, expected.to_string());
    }
}
