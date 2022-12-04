use std::collections::BTreeMap;

pub fn run_first(input: &str) -> String {
    min_max_diff(input, 10).to_string()
}

pub fn run_second(input: &str) -> String {
    min_max_diff(input, 40).to_string()
}

fn char_windows(src: &str, win_size: usize) -> impl Iterator<Item = &str> {
    src.char_indices().flat_map(move |(from, _)| {
        src[from..]
            .char_indices()
            .nth(win_size - 1)
            .map(|(to, c)| &src[from..from + to + c.len_utf8()])
    })
}

pub fn min_max_diff(input: &str, steps: usize) -> usize {
    let (polymer, insertions) = parse_input(input);

    let mut pairs_count: BTreeMap<String, usize> = BTreeMap::new();
    let mut element_count: BTreeMap<char, usize> = BTreeMap::new();

    for c in polymer.chars() {
        *element_count.entry(c).or_insert(0) += 1
    }

    for pair in char_windows(&polymer, 2) {
        *pairs_count.entry(pair.to_string()).or_insert(0) += 1;
    }

    for _ in 0..steps {
        let mut to_add: Vec<(String, usize)> = vec![];
        let mut to_remove: Vec<(String, usize)> = vec![];

        for (pair, insert) in &insertions {
            let pair_str = pair.to_string();
            let curr_pair_count = *pairs_count.get(&pair_str).unwrap_or(&0);

            if curr_pair_count > 0 {
                let left_insert = format!("{}{}", pair_str.chars().next().unwrap(), insert);
                let right_insert = format!("{}{}", insert, pair_str.chars().nth(1).unwrap());

                let inserted_char = insertions.get(&pair_str).unwrap().chars().next().unwrap();
                *element_count.entry(inserted_char).or_insert(0) += curr_pair_count;

                to_add.push((left_insert, curr_pair_count));
                to_add.push((right_insert, curr_pair_count));
                to_remove.push((pair_str, curr_pair_count));
            }
        }

        for (key, value) in to_add {
            // Add all new created pairs
            *pairs_count.entry(key.to_string()).or_insert(0) += value;
        }

        for (key, value) in to_remove {
            // Remove all old split pairs
            *pairs_count.entry(key.to_string()).or_insert(0) -= value;
        }
    }
    let max = *element_count
        .iter()
        .max_by_key(|&(_, count)| count)
        .unwrap()
        .1;
    let min = *element_count
        .iter()
        .min_by_key(|&(_, count)| count)
        .unwrap()
        .1;
    max - min
}

fn parse_input(input: &str) -> (String, BTreeMap<String, String>) {
    let mut parts = input.split("\n\n").filter(|l| !l.is_empty());

    let polymer: String = parts.next().unwrap().to_string();

    let insertions: BTreeMap<_, _> = parts
        .next()
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            let insert_tokens: Vec<_> = l.trim().split(" -> ").filter(|l| !l.is_empty()).collect();
            (insert_tokens[0].to_string(), insert_tokens[1].to_string())
        })
        .collect();

    (polymer, insertions)
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u8 = 14;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "2068";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "2158894777814";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_min_max_diff() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        assert_eq!(min_max_diff(&input, 10), 1588);
    }
}
