use itertools::Itertools;

use crate::common::parse::to_non_empty_lines;

pub fn run_first(input: &str) -> String {
    let lines = to_non_empty_lines(input);
    syntax_error_score(&lines).to_string()
}

pub fn run_second(input: &str) -> String {
    let lines = to_non_empty_lines(input);
    line_completion_score(&lines).to_string()
}

pub fn line_completion_score(input: &[String]) -> usize {
    let scores: Vec<_> = only_incomplete_lines(input)
        .iter()
        .map(|l| score_line_completion(l))
        .sorted()
        .collect();

    let middle_index = (scores.len() - 1) / 2;
    *scores.get(middle_index).unwrap()
}

pub fn syntax_error_score(input: &[String]) -> usize {
    input
        .iter()
        .filter_map(|l| first_broken_char(l))
        .map(score_char)
        .sum()
}

pub fn only_incomplete_lines(input: &[String]) -> Vec<&String> {
    input
        .iter()
        .filter(|l| first_broken_char(l).is_none())
        .collect()
}

pub fn score_line_completion(line: &str) -> usize {
    let to_complete: Vec<_> = reduce_line(line)
        .iter()
        .rev()
        .map(|&c| get_closing(c))
        .collect();
    let mut score = 0;

    for c in to_complete {
        score *= 5;
        let points = match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!(),
        };
        score += points;
    }

    score
}

fn get_closing(ch: char) -> char {
    match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

pub fn score_char(ch: char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

pub fn reduce_line(line: &str) -> Vec<char> {
    let mut non_closed: Vec<char> = vec![];

    for ch in line.chars() {
        non_closed.push(ch);

        loop {
            let len = non_closed.len();
            if len > 1 {
                let open = non_closed[len - 2];
                let close = non_closed[len - 1];
                if closes(open, close) {
                    non_closed.pop();
                    non_closed.pop();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    non_closed
}

pub fn first_broken_char(line: &str) -> Option<char> {
    let mut non_closed: Vec<char> = vec![];

    for ch in line.chars() {
        non_closed.push(ch);

        loop {
            let len = non_closed.len();
            if len > 1 {
                let open = non_closed[len - 2];
                let close = non_closed[len - 1];
                if closes(open, close) {
                    non_closed.pop();
                    non_closed.pop();
                } else {
                    if mismatch(open, close) {
                        return Some(close);
                    }
                    break;
                }
            } else {
                break;
            }
        }
    }

    None
}

pub fn is_open_char(ch: char) -> bool {
    matches!(ch, '(' | '{' | '[' | '<')
}

pub fn is_close_char(ch: char) -> bool {
    matches!(ch, ')' | '}' | ']' | '>')
}

pub fn mismatch(open: char, close: char) -> bool {
    is_open_char(open) && is_close_char(close) && !closes(open, close)
}

pub fn closes(open: char, close: char) -> bool {
    match (open, close) {
        ('(', ')') => true,
        ('{', '}') => true,
        ('[', ']') => true,
        ('<', '>') => true,
        (_, _) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_to_string;

    const YEAR: u32 = 2021;
    const DAY: u8 = 10;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "399153";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "2995077699";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_chunks_1() {
        let expected: Vec<char> = vec![];
        assert_eq!(reduce_line("([])"), expected);
        assert_eq!(reduce_line("<([{}])>"), expected);
        assert_eq!(reduce_line("[<>({}){}[([])<>]]"), expected);
        assert_eq!(reduce_line("(((((((((())))))))))"), expected);
    }

    #[test]
    fn reduce_incomplete_line() {
        let line = "[({(<(())[]>[[{[]{<()<>>";
        let result = reduce_line(line);
        let expected: Vec<char> = vec!['[', '(', '{', '(', '[', '[', '{', '{'];
        assert_eq!(result, expected);
    }

    #[test]
    fn first_broken_char_1() {
        let line = "(]";
        let result = first_broken_char(line).unwrap();
        let expected = ']';
        assert_eq!(result, expected);
    }

    #[test]
    fn first_broken_char_2() {
        let line = "{([(<{}[<>[]}>{[]{[(<()>";
        let result = first_broken_char(line).unwrap();
        let expected = '}';
        assert_eq!(result, expected);
    }

    #[test]
    fn first_broken_char_3() {
        let line = "<{([([[(<>()){}]>(<<{{";
        let result = first_broken_char(line).unwrap();
        let expected = '>';
        assert_eq!(result, expected);
    }

    #[test]
    fn first_broken_char_incomplete_line() {
        let line = "[({(<(())[]>[[{[]{<()<>>";
        let result = first_broken_char(line);
        let expected = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_score_line_completion_1() {
        let line = "<{([{{}}[<[[[<>{}]]]>[]]";
        let result = score_line_completion(line);
        let expected = 294;
        assert_eq!(result, expected);
    }
}
