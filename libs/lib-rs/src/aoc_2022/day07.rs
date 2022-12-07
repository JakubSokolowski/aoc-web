use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::common::parse::to_non_empty_lines;

pub fn run_first(input: &str) -> String {
    let lookup = dir_size_lookup(input);
    lookup
        .values()
        .filter(|&&dir_size| dir_size <= 100_000)
        .sum::<i64>()
        .to_string()
}

pub fn run_second(input: &str) -> String {
    let lookup = dir_size_lookup(input);
    let root_size = *lookup.get("").unwrap();
    lookup
        .values()
        .filter(|&&dir_size| dir_size >= root_size - 40_000_000)
        .min()
        .unwrap()
        .to_string()
}

fn dir_size_lookup(input: &str) -> HashMap<String, i64> {
    let lines = to_non_empty_lines(input);

    let mut path: Vec<String> = vec!["".to_string()];
    let mut size_lookup: HashMap<String, i64> = HashMap::new();

    for line in lines {
        let args: Vec<_> = line.split(' ').collect();
        match args[..] {
            ["$", "ls"] => {
                continue;
            }
            ["dir", _] => {
                continue;
            }
            ["$", "cd", "/"] => {
                continue;
            }
            ["$", "cd", ".."] => {
                path.pop();
            }
            ["$", "cd", dir] => {
                path.push(dir.to_string());
            }
            [size_str, _] => {
                let size = size_str.parse::<i64>().unwrap();
                for i in 1..=path.len() {
                    let subpath = path[0..i].join("/");
                    if let Entry::Vacant(e) = size_lookup.entry(subpath.clone()) {
                        e.insert(size);
                    } else {
                        *size_lookup.get_mut(&subpath).unwrap() += size;
                    }
                }
            }
            _ => unreachable!(""),
        }
    }

    size_lookup
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 7;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "1427048";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "2940614";
        assert_eq!(result, expected.to_string());
    }
}
