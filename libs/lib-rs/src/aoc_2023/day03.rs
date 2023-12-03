use itertools::Itertools;
use std::collections::HashMap;

pub fn run_first(input: &str) -> String {
    let schematic = parse_schematic(input);

    let mut sum = 0;

    for row in 0..schematic.height {
        let mut column = 0;

        while column < schematic.width {
            let mut curr_value = schematic.get_value(row, column);
            let mut digits = String::new();
            let mut digits_adjacent: Vec<bool> = Vec::new();
            while curr_value.is_ascii_digit() {
                digits += &curr_value.to_string();
                digits_adjacent.push(schematic.is_adjacent_to_symbol(row, column));
                column += 1;
                curr_value = schematic.get_value(row, column);
            }

            if digits_adjacent.iter().any(|&b| b) {
                let num = digits.parse::<i32>().unwrap();
                sum += num;
            }
            column += 1;
        }
    }
    sum.to_string()
}

pub fn run_second(input: &str) -> String {
    let schematic = parse_schematic(input);

    let mut gear_nums_lookup: HashMap<(i32, i32), Vec<i32>> = HashMap::new();

    for row in 0..schematic.height {
        let mut column = 0;
        while column < schematic.width {
            let mut curr_value = schematic.get_value(row, column);
            let mut digits = String::new();
            let mut gear_coords: Vec<(i32, i32)> = Vec::new();

            while curr_value.is_ascii_digit() {
                digits += &curr_value.to_string();
                gear_coords.append(&mut schematic.find_adjacent_gear_cords(row, column));
                column += 1;
                curr_value = schematic.get_value(row, column);
            }

            for gear in gear_coords.iter().unique() {
                let gear_nums = gear_nums_lookup.entry(*gear).or_insert_with(Vec::new);
                gear_nums.push(digits.parse::<i32>().unwrap());
            }

            column += 1;
        }
    }

    gear_nums_lookup
        .values()
        .map(|v| if v.len() == 2 { v[0] * v[1] } else { 0 })
        .sum::<i32>()
        .to_string()
}

fn parse_schematic(input: &str) -> Schematic {
    let values = input.chars().filter(|&c| c != '\n').collect_vec();
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;

    Schematic::new(values, width, height)
}

struct Schematic {
    width: i32,
    height: i32,
    values: Vec<char>,
}

impl Schematic {
    pub fn new(values: Vec<char>, width: i32, height: i32) -> Schematic {
        Schematic {
            width,
            height,
            values,
        }
    }
    fn get_index(&self, row: i32, column: i32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_value(&self, row: i32, column: i32) -> char {
        self.values[self.get_index(row, column)]
    }

    fn is_adjacent_to_symbol(&self, row: i32, column: i32) -> bool {
        self.get_adjacent_values(row, column)
            .iter()
            .any(|&c| !c.is_ascii_digit() && c != '.')
    }

    fn find_adjacent_gear_cords(&self, row: i32, column: i32) -> Vec<(i32, i32)> {
        let coords = vec![
            (0, 1),
            (0, -1),
            (-1, 0),
            (1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        coords
            .into_iter()
            .filter_map(|(dr, dc)| {
                let new_row = row + dr;
                let new_column = column + dc;
                if new_row < 0
                    || new_row >= self.height
                    || new_column < 0
                    || new_column >= self.width
                {
                    return None;
                }
                let index = self.get_index(row + dr, column + dc);
                if self.values.get(index).unwrap() == &'*' {
                    return Some((new_row, new_column));
                }
                None
            })
            .collect()
    }

    fn get_adjacent_values(&self, row: i32, column: i32) -> Vec<char> {
        let coords = vec![
            (0, 1),
            (0, -1),
            (-1, 0),
            (1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        coords
            .into_iter()
            .filter_map(|(dr, dc)| {
                let new_row = row + dr;
                let new_column = column + dc;
                if new_row < 0
                    || new_row >= self.height
                    || new_column < 0
                    || new_column >= self.width
                {
                    return None;
                }
                let index = self.get_index(row + dr, column + dc);
                self.values.get(index)
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2023;
    const DAY: u8 = 3;

    const TEST_DATA: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        assert_eq!(result, "532331");
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        assert_eq!(result, "82301120");
    }

    #[test]
    fn test_get_adjacent() {
        // given
        let schematic = parse_schematic(TEST_DATA);

        // then
        assert_eq!(schematic.get_adjacent_values(0, 0), vec!['6', '.', '.']);
        assert_eq!(
            schematic.get_adjacent_values(0, 1),
            vec!['7', '4', '.', '.', '.']
        );
        assert_eq!(
            schematic.get_adjacent_values(0, 2),
            vec!['.', '6', '.', '*', '.']
        );
        assert_eq!(
            schematic.get_adjacent_values(7, 6),
            vec!['5', '.', '.', '.', '.', '*', '.', '.']
        );
    }

    #[test]
    fn test_is_adjacent_to_symbol() {
        // given
        let schematic = parse_schematic(TEST_DATA);

        // then
        assert!(!schematic.is_adjacent_to_symbol(0, 0));
        assert!(!schematic.is_adjacent_to_symbol(0, 1));
        assert!(schematic.is_adjacent_to_symbol(0, 2));
        assert!(schematic.is_adjacent_to_symbol(7, 6));
    }

    #[test]
    fn test_get_adjacent_nums() {
        // given
        let schematic = parse_schematic(TEST_DATA);

        // then
        assert_eq!(schematic.find_adjacent_gear_cords(0, 2), vec![(1, 3)]);
    }

    #[test]
    fn test_part_1_small() {
        // when
        let result = run_first(TEST_DATA);

        // then
        assert_eq!(result, "4361");
    }

    #[test]
    fn test_part_2_small() {
        // when
        let result = run_second(TEST_DATA);

        // then
        assert_eq!(result, "467835");
    }
}
