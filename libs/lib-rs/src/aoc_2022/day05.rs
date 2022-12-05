use crate::common::parse::{parse_numbers, to_non_empty_lines};
use itertools::Itertools;

pub fn run_first(input: &str) -> String {
    let (mut stacks, commands) = parse_stacks(input);
    for cmd in commands {
        let amount = cmd[0];
        let from = cmd[1] as usize;
        let to = cmd[2] as usize;
        for _ in 0..amount {
            let cr = stacks.get_mut(from - 1).unwrap().pop_crate();
            stacks.get_mut(to - 1).unwrap().push_crate(&cr);
        }
    }
    let top_crates = stacks
        .iter()
        .map(|s| s.get_top())
        .collect::<Vec<String>>()
        .join("");
    top_crates
}

pub fn run_second(input: &str) -> String {
    let (mut stacks, commands) = parse_stacks(input);
    for cmd in commands {
        let amount = cmd[0] as usize;
        let from = cmd[1] as usize;
        let to = cmd[2] as usize;
        let cr = stacks.get_mut(from - 1).unwrap().pop_multiple(amount);
        stacks.get_mut(to - 1).unwrap().push_multiple(&cr);
    }
    let top_crates = stacks
        .iter()
        .map(|s| s.get_top())
        .collect::<Vec<String>>()
        .join("");
    top_crates
}

fn parse_stacks(input: &str) -> (Vec<CrateStack>, Vec<Vec<i64>>) {
    let lines = to_non_empty_lines(input);
    let (stack_lines, cmd_lines) = lines.split_at(8);
    let max_len = stack_lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut stacks: Vec<_> = (0..(max_len + 1) / 4)
        .map(|_| CrateStack { crates: vec![] })
        .collect();

    for line in stack_lines {
        for idx in 0..stacks.len() {
            match get_crate_for_stack(idx + 1, line) {
                None => {}
                Some(c) => {
                    if !c.is_empty() {
                        stacks.get_mut(idx).unwrap().push_crate(&c);
                    }
                }
            }
        }
    }

    for idx in 0..stacks.len() {
        stacks.get_mut(idx).unwrap().reverse();
    }

    let cmds: Vec<_> = cmd_lines.iter().skip(1).map(|l| parse_numbers(l)).collect();

    (stacks, cmds)
}

pub fn get_crate_for_stack(idx: usize, line: &str) -> Option<String> {
    line.chars()
        .nth(1 + (idx - 1) * 4)
        .map(|c| c.to_string().trim_end().to_string())
}

#[derive(Debug)]
struct CrateStack {
    pub crates: Vec<String>,
}

impl CrateStack {
    fn push_crate(&mut self, cr: &str) {
        self.crates.push(cr.to_string());
    }

    fn pop_crate(&mut self) -> String {
        self.crates.pop().unwrap()
    }

    fn pop_multiple(&mut self, count: usize) -> String {
        (0..count).map(|_| self.crates.pop().unwrap()).join("")
    }

    fn push_multiple(&mut self, crates: &str) {
        for c in crates.chars().rev() {
            self.push_crate(&c.to_string())
        }
    }

    fn get_top(&self) -> String {
        self.crates.last().unwrap().clone()
    }

    fn reverse(&mut self) {
        self.crates = self.crates.clone().into_iter().rev().collect();
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 5;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "GFTNRBZPF";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "VRQWPDSGP";
        assert_eq!(result, expected.to_string());
    }
}
