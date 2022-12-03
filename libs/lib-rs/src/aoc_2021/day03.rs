use std::cmp::Ordering;

use itertools::Itertools;

pub fn run_first(input: &str) -> String {
    let lines: Vec<_> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    power_consumption(&lines).to_string()
}

pub fn run_second(input: &str) -> String {
    let lines: Vec<_> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    life_support(&lines).to_string()
}

pub fn power_consumption(input: &[String]) -> usize {
    let gamma_vec = get_gamma_vec(input);
    let epsilon_vec = negate(&gamma_vec);

    let gamma_num = binary_vec_to_num(&gamma_vec);
    let epsilon_num = binary_vec_to_num(&epsilon_vec);

    gamma_num * epsilon_num
}

pub fn life_support(input: &[String]) -> usize {
    let oxygen = get_oxygen_rating(input);
    let co2 = get_co2_rating(input);

    oxygen * co2
}

pub fn get_rating(
    input: &[String],
    filter: fn(input: &[String], position: usize) -> Vec<String>,
) -> usize {
    let row_length = input.iter().next().unwrap().len();
    let mut curr_input: Vec<String> = input.to_vec();

    for position in 0..row_length {
        curr_input = filter(&curr_input, position);
        if curr_input.len() == 1 {
            let result_vec = curr_input.get(0).unwrap();
            return usize::from_str_radix(result_vec, 2).unwrap();
        }
    }

    0
}

pub fn get_oxygen_rating(input: &[String]) -> usize {
    get_rating(input, filter_by_oxygen_rating)
}

pub fn get_co2_rating(input: &[String]) -> usize {
    get_rating(input, filter_by_co2_rating)
}

pub fn filter_by_rating(
    input: &[String],
    position: usize,
    ch_rating: fn(usize, usize) -> char,
) -> Vec<String> {
    let (zeros_count, ones_count) = count_at_position(input, position);
    let ch_to_keep = ch_rating(zeros_count, ones_count);

    input
        .iter()
        .filter(|i| i.chars().nth(position).unwrap() == ch_to_keep)
        .cloned()
        .collect()
}

pub fn filter_by_oxygen_rating(input: &[String], position: usize) -> Vec<String> {
    filter_by_rating(input, position, |zeros_count, ones_count| match zeros_count
        .partial_cmp(&ones_count)
        .unwrap()
    {
        Ordering::Greater => '0',
        Ordering::Less => '1',
        Ordering::Equal => '1',
    })
}

pub fn filter_by_co2_rating(input: &[String], position: usize) -> Vec<String> {
    filter_by_rating(input, position, |zeros_count, ones_count| match zeros_count
        .partial_cmp(&ones_count)
        .unwrap()
    {
        Ordering::Greater => '1',
        Ordering::Less => '0',
        Ordering::Equal => '0',
    })
}

pub fn binary_vec_to_num(vec: &[usize]) -> usize {
    let as_str = vec.iter().map(|n| n.to_string()).join("");
    usize::from_str_radix(&as_str, 2).unwrap()
}

pub fn get_gamma_vec(input: &[String]) -> Vec<usize> {
    let row_length = input.iter().next().unwrap().len();
    (0..row_length)
        .map(|pos| most_common_at_position(input, pos))
        .collect()
}

pub fn negate(input: &[usize]) -> Vec<usize> {
    input.iter().map(|&n| if n == 1 { 0 } else { 1 }).collect()
}

pub fn count_at_position(input: &[String], position: usize) -> (usize, usize) {
    let total = input.len();

    let ones_count = input
        .iter()
        .map(|s| {
            let char = s.chars().nth(position).unwrap();

            match char {
                '1' => 1,
                '0' => 0,
                _ => panic!("Invalid char"),
            }
        })
        .sum();

    let zeros_count = total - ones_count;
    (zeros_count, ones_count)
}

pub fn most_common_at_position(input: &[String], position: usize) -> usize {
    let (zeros_count, ones_count) = count_at_position(input, position);
    usize::from(ones_count > zeros_count)
}

#[cfg(test)]
mod tests {
    use crate::common::parse::test_utils::vec_of_strings;
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u8 = 3;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "3959450";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "7440311";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_most_common_at_position() {
        // given
        let nums = vec_of_strings![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010"
        ];

        // when
        let result = most_common_at_position(&nums, 0);

        // then
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_gamma_vec() {
        // given
        let nums = vec_of_strings![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010"
        ];

        // when
        let result = get_gamma_vec(&nums);

        // then
        let expected: Vec<usize> = vec![1, 0, 1, 1, 0];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_power_consumption_1() {
        // given
        let nums = vec_of_strings![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010"
        ];

        // when
        let result = power_consumption(&nums);

        // then
        let expected = 198;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_reduce_oxy_by_ratings() {
        // given
        let nums = vec_of_strings![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010"
        ];

        // when
        let result = filter_by_oxygen_rating(&nums, 0);

        // then
        let expected =
            vec_of_strings!["11110", "10110", "10111", "10101", "11100", "10000", "11001"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_oxygen_rating() {
        // given
        let nums = vec_of_strings![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010"
        ];

        // when
        let result = get_oxygen_rating(&nums);

        // then
        let expected = 23;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_co2_rating() {
        // given
        let nums = vec_of_strings![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010"
        ];

        // when
        let result = get_co2_rating(&nums);

        // then
        let expected = 10;
        assert_eq!(result, expected);
    }
}
