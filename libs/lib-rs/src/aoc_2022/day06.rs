use std::collections::HashSet;
use std::iter::FromIterator;

pub fn run_first(input: &str) -> String {
    first_uniq(input, 4).to_string()
}

pub fn run_second(input: &str) -> String {
    first_uniq(input, 14).to_string()
}

pub fn first_uniq(input: &str, size: usize) -> usize {
    let mut char_count = 0;
    let chars = input.chars().collect::<Vec<char>>();

    for (idx, window) in chars.windows(size).enumerate() {
        let uniq: HashSet<&char> = HashSet::from_iter(window.iter());
        if uniq.len() == size {
            char_count = idx + size;
            break;
        }
    }

    char_count
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 6;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "1042";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "2980";
        assert_eq!(result, expected.to_string());
    }
}
