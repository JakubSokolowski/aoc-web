use std::collections::HashSet;

use itertools::Itertools;

pub fn run_first(input: &str) -> String {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let half = line.len() / 2;
            let first: HashSet<_> = line[..half].to_string().chars().collect();
            let second: HashSet<_> = line[half..].to_string().chars().collect();
            first
                .intersection(&second)
                .map(|&c| get_char_value(c))
                .sum::<i64>()
        })
        .sum::<i64>()
        .to_string()
}

pub fn run_second(input: &str) -> String {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let first: HashSet<_> = chunk.next().unwrap().chars().collect();
            let second: HashSet<_> = chunk.next().unwrap().chars().collect();
            let third: HashSet<_> = chunk.next().unwrap().chars().collect();

            first
                .iter()
                .filter(|c| second.contains(c))
                .filter(|c| third.contains(c))
                .map(|&c| get_char_value(c))
                .sum::<i64>()
        })
        .sum::<i64>()
        .to_string()
}

fn get_char_value(c: char) -> i64 {
    match c {
        'a'..='z' => c as i64 - 96,
        'A'..='Z' => c as i64 - 65 + 27,
        _ => unreachable!("Unexpected char: {}", c),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 3;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "8401";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "2641";
        assert_eq!(result, expected.to_string());
    }
}
