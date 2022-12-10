use std::collections::HashSet;

use crate::common::parse::to_non_empty_lines;

type Instruction = (String, i64);

pub fn run_first(input: &str) -> String {
    let instructions = parse_instructions(input);
    let cycles = vec![20, 60, 100, 140, 180, 220].into_iter().collect();
    let mut program = Program::default();
    program.sum_cycles(&instructions, &cycles).to_string()
}

pub fn run_second(input: &str) -> String {
    let instructions = parse_instructions(input);
    let cycles = vec![20, 60, 100, 140, 180, 220].into_iter().collect();
    let mut program = Program::default();
    program.compute_sprite(&instructions, &cycles)
}

struct Program {
    result: i64,
    cycle: i64,
    sum: i64,
    output: String,
}

impl Default for Program {
    fn default() -> Self {
        Program {
            cycle: 0,
            sum: 0,
            result: 0,
            output: "".to_string(),
        }
    }
}

impl Program {
    fn sum_cycles(&mut self, instructions: &[Instruction], cycles: &HashSet<i64>) -> i64 {
        self.result = 1;
        self.cycle = 0;
        self.sum = 0;

        for instr in instructions {
            let (name, value) = instr;
            match name.as_str() {
                "noop" => {
                    self.sum_tick(cycles);
                }
                _ => {
                    self.sum_tick(cycles);
                    self.sum_tick(cycles);
                    self.result += value;
                }
            }
        }

        self.sum
    }

    fn sum_tick(&mut self, cycles: &HashSet<i64>) {
        self.cycle += 1;
        if cycles.contains(&self.cycle) {
            self.sum += self.cycle * self.result
        }
    }

    fn compute_sprite(&mut self, instructions: &[Instruction], cycles: &HashSet<i64>) -> String {
        self.result = 1;
        self.cycle = 0;
        self.sum = 0;
        self.output = "".to_string();

        for instr in instructions {
            let (name, value) = instr;
            match name.as_str() {
                "noop" => {
                    self.sprite_tick(cycles);
                }
                _ => {
                    self.sprite_tick(cycles);
                    self.sprite_tick(cycles);
                    self.result += value;
                }
            }
        }

        self.output.clone()
    }

    fn sprite_tick(&mut self, cycles: &HashSet<i64>) {
        let sprite = vec![self.result - 1, self.result, self.result + 1];
        let c = if sprite.iter().any(|&y| y == self.cycle % 40) {
            '#'
        } else {
            ' '
        };
        self.output.push(c);
        self.cycle += 1;
        if cycles.contains(&self.cycle) {
            self.sum += self.cycle * self.result
        }
        if self.cycle % 40 == 0 {
            self.output.push('\n');
        }
    }
}

pub fn parse_instructions(input: &str) -> Vec<(String, i64)> {
    to_non_empty_lines(input)
        .iter()
        .map(|l| {
            let tokens: Vec<_> = l.split(' ').filter(|t| !t.is_empty()).collect();

            (
                tokens.first().unwrap().to_string(),
                tokens
                    .get(1)
                    .unwrap_or(&"0")
                    .to_string()
                    .parse::<i64>()
                    .unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 10;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "14920";
        assert_eq!(result, expected.to_string());
    }
}
