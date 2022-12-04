use std::collections::HashSet;

use itertools::Itertools;

use crate::common::parse::to_non_empty_lines;

pub fn run_first(input: &str) -> String {
    let lines = to_non_empty_lines(input);
    part_1(&lines).to_string()
}

pub fn run_second(input: &str) -> String {
    let lines = to_non_empty_lines(input);
    part_2(&lines).to_string()
}

pub fn part_1(input: &[String]) -> usize {
    input
        .iter()
        .map(|l| {
            let tokens: Vec<&str> = l.split('|').collect();
            let out_tokens: Vec<&str> = tokens[1].split(' ').collect();

            out_tokens
                .iter()
                .filter(|t| t.len() == 2 || t.len() == 4 || t.len() == 3 || t.len() == 7)
                .count()
        })
        .sum()
}

pub fn part_2(input: &[String]) -> usize {
    input.iter().map(|l| get_line_value(l)).sum()
}

pub fn from_digits(digits: &[usize]) -> usize {
    digits
        .iter()
        .map(|d| d.to_string())
        .join("")
        .parse()
        .unwrap()
}

pub fn get_line_value(input: &str) -> usize {
    let tokens: Vec<Vec<String>> = input
        .splitn(2, '|')
        .map(|s| s.trim().split(' ').map(|s| s.to_string()).collect())
        .collect();

    let segments = tokens.get(0).unwrap();
    let values = tokens.get(1).unwrap();
    let combination = find_encoding(segments);
    decode(values, &combination)
}

pub fn decode(segments: &[String], combination: &str) -> usize {
    let digits: Vec<_> = segments
        .iter()
        .map(|s| find_value(s, combination))
        .collect();

    from_digits(&digits)
}

pub fn find_value(segment: &str, combination: &str) -> usize {
    let valid_segments = vec![
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];

    return valid_segments
        .iter()
        .position(|&s| same_chars(segment, &map_segment(s, combination)))
        .unwrap();
}

fn same_chars(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let set: HashSet<char> = a.chars().collect();

    b.chars().all(|c| set.contains(&c))
}

pub fn find_encoding(segments: &[String]) -> String {
    let signals = "abcdefg".to_string();
    for combination in signals
        .chars()
        .permutations(signals.len())
        .map(|s| s.iter().collect::<String>())
    {
        if all_segments_match(segments, &combination) {
            return combination;
        }
    }
    panic!("Did not found combination")
}

pub fn all_segments_match(segments: &[String], combination: &str) -> bool {
    let valid_segments: Vec<_> = vec![
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];

    valid_segments
        .iter()
        .all(|s| segment_matches(s, combination, segments))
}

pub fn segment_matches(valid_segment: &str, combination: &str, signals: &[String]) -> bool {
    let mapped = map_segment(valid_segment, combination);

    return signals.iter().any(|s| same_chars(s, &mapped));
}

pub fn map_segment(valid_segment: &str, combination: &str) -> String {
    valid_segment
        .chars()
        .map(|s| map_signal(s, combination))
        .collect()
}

fn map_signal(c: char, combination: &str) -> char {
    let index = (c as u32 - 96 - 1) as usize;
    combination.chars().nth(index).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::common::parse::test_utils::vec_of_strings;
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u8 = 8;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "412";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "978171";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_map_segment_1() {
        // given
        let segment = "cf";
        let combination = "deafgbc";

        // when
        let result = map_segment(segment, combination);

        // then
        assert_eq!(result, "ab");
    }

    #[test]
    fn test_map_segment_2() {
        // given
        let segment = "bcdf";
        let combination = "deafgbc";

        // when
        let result = map_segment(segment, combination);

        // then
        assert_eq!(result, "eafb");
    }

    #[test]
    fn test_segment_matches() {
        // given
        let segments = vec_of_strings![
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab"
        ];
        let combination = "deafgbc";

        // when
        let result = all_segments_match(&segments, combination);

        // then
        assert_eq!(result, true);
    }

    #[test]
    fn test_find_encoding_1() {
        // given
        let segments = vec_of_strings![
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab"
        ];

        // when
        let result = find_encoding(&segments);

        // then
        assert_eq!(result, "deafgbc");
    }

    #[test]
    fn test_decode_1() {
        // given
        let segments = vec_of_strings!["cdfeb", "fcadb", "cdfeb", "cdbaf"];
        let combination = "deafgbc";

        // when
        let result = decode(&segments, combination);

        // then
        assert_eq!(result, 5353);
    }

    #[test]
    fn test_get_line_value() {
        // given
        let line =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

        // when
        let result = get_line_value(line);

        // then
        assert_eq!(result, 5353);
    }
}
