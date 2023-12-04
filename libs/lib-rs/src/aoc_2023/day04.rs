use crate::common::parse::parse_numbers;
use std::collections::HashMap;

pub fn run_first(input: &str) -> String {
    input
        .lines()
        .map(|line| match count_wins(line) {
            0 => 0,
            c => i64::pow(2, (c - 1) as u32),
        })
        .sum::<i64>()
        .to_string()
}

pub fn count_wins(card_line: &str) -> i64 {
    let mut parts = card_line.split(':').nth(1).unwrap().split('|');
    let winning_nums = parse_numbers(parts.next().unwrap());
    let card_nums = parse_numbers(parts.next().unwrap());

    card_nums
        .iter()
        .filter(|num| winning_nums.contains(num))
        .count() as i64
}

pub fn run_second(input: &str) -> String {
    let card_lines = input.lines().collect::<Vec<_>>();
    let mut card_count_lookup = (0..card_lines.len())
        .map(|idx| (idx, 1))
        .collect::<HashMap<usize, i64>>();

    let computed_wins = card_lines
        .iter()
        .map(|line| count_wins(line))
        .collect::<Vec<_>>();

    for (idx, wins) in computed_wins.iter().enumerate() {
        let num_cards = card_count_lookup.get(&idx).unwrap();

        for _ in 0..*num_cards {
            for i in 0..*wins as usize {
                card_count_lookup
                    .entry(idx + i + 1)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    card_count_lookup.values().sum::<i64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_to_string;
    const YEAR: u32 = 2023;
    const DAY: u8 = 4;

    const TEST_DATA: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_run_first() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        assert_eq!(result, "32609");
    }

    #[test]
    fn test_run_second() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        assert_eq!(result, "14624680");
    }

    #[test]
    fn test_run_second_small() {
        let result = run_second(TEST_DATA);
        assert_eq!(result, "30");
    }
}
