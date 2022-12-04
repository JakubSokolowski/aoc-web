use std::collections::HashSet;

use itertools::Itertools;

use crate::common::parse::to_non_empty_lines;

pub fn run_first(input: &str) -> String {
    let lines = to_non_empty_lines(input);
    flash_count(&lines).to_string()
}

pub fn run_second(input: &str) -> String {
    let lines = to_non_empty_lines(input);
    first_simultaneous(&lines).to_string()
}

struct OctopodesMap {
    width: i32,
    height: i32,
    values: Vec<i32>,
}

impl OctopodesMap {
    pub fn new(values: Vec<i32>, width: i32, height: i32) -> OctopodesMap {
        OctopodesMap {
            width,
            height,
            values,
        }
    }

    fn get_index(&self, row: i32, column: i32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_value(&self, row: i32, column: i32) -> i32 {
        self.values[self.get_index(row, column)]
    }

    fn increase_energy(&mut self, value: i32) {
        for v in self.values.iter_mut() {
            *v += value
        }
    }

    fn increase_for_points(&mut self, points: &[(i32, i32)], value: usize) {
        for &(r, c) in points.iter() {
            let index = self.get_index(r, c);
            *self.values.get_mut(index).unwrap() += value as i32
        }
    }

    fn reset_points(&mut self, points: &HashSet<(i32, i32)>) {
        for &(r, c) in points.iter() {
            let index = self.get_index(r, c);
            *self.values.get_mut(index).unwrap() = 0
        }
    }

    fn get_flashing(&self, flashed: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
        (0..self.height)
            .cartesian_product(0..self.width)
            .filter(|&(row, column)| {
                self.get_value(row, column) > 9 && !flashed.contains(&(row, column))
            })
            .collect()
    }

    fn get_adjacent(&self, row: i32, column: i32) -> Vec<(i32, i32)> {
        let coords = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        coords
            .iter()
            .map(|(dr, dc)| (row + dr, column + dc))
            .collect()
    }

    fn count_flashes(&mut self, num_steps: usize) -> usize {
        (0..num_steps).map(|_| self.step()).sum::<usize>()
    }

    fn first_simultaneous(&mut self) -> usize {
        let mut loop_count = 0;
        loop {
            loop_count += 1;
            let step_count = self.step();
            if step_count == self.values.len() {
                return loop_count;
            }
        }
    }

    fn step(&mut self) -> usize {
        // increase total energy by 1
        self.increase_energy(1);
        let mut all_flashed: HashSet<(i32, i32)> = HashSet::new();
        loop {
            let flashed = self.get_flashing(&all_flashed);
            if flashed.is_empty() {
                break;
            }
            all_flashed.extend(flashed.iter());
            // get all valid adjacent points adjacent points that are not
            // out of bounds, and not already flashed. Some points will be duplicated
            // when they are adjacent to multiple flashing
            let adjacent = self.get_all_valid_adjacent(&flashed, &all_flashed);
            // increase value for adjacent
            self.increase_for_points(&adjacent, 1);
        }
        self.reset_points(&all_flashed);
        all_flashed.len()
    }

    fn get_all_valid_adjacent(
        &self,
        points: &HashSet<(i32, i32)>,
        flashed: &HashSet<(i32, i32)>,
    ) -> Vec<(i32, i32)> {
        points
            .iter()
            .flat_map(|&(r, c)| self.get_adjacent(r, c))
            .filter(|p| !self.out_of_bounds(p.0, p.1) && !flashed.contains(p))
            .collect()
    }

    fn out_of_bounds(&self, row: i32, column: i32) -> bool {
        if row < 0 || row >= self.height {
            return true;
        }
        if column < 0 || column >= self.width {
            return true;
        }
        false
    }
}

fn parse_octopodes(input: &[String]) -> OctopodesMap {
    let width = parse_line(&input[0]).len() as i32;
    let height = input.len() as i32;
    let values: Vec<i32> = input.iter().flat_map(|l| parse_line(l)).collect();

    OctopodesMap::new(values, width, height)
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split("")
        .filter(|&x| !x.is_empty())
        .map(|n| n.parse().unwrap())
        .collect()
}

pub fn flash_count(input: &[String]) -> usize {
    let mut opd = parse_octopodes(input);
    opd.count_flashes(100)
}

pub fn first_simultaneous(input: &[String]) -> usize {
    let mut opd = parse_octopodes(input);
    opd.first_simultaneous()
}

#[cfg(test)]
mod tests {
    use crate::common::parse::test_utils::vec_of_strings;
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u8 = 11;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "1686";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "360";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_get_flashing() {
        // given
        let input = vec_of_strings!["11111", "19991", "19191", "19991", "11111"];
        let mut opd = parse_octopodes(&input);
        let flashed: HashSet<(i32, i32)> = HashSet::new();

        // when
        opd.increase_energy(1);
        let flashing = opd.get_flashing(&flashed);

        // then
        let expected = HashSet::from([
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 1),
            (2, 3),
            (3, 1),
            (3, 2),
            (3, 3),
        ]);
        assert_eq!(flashing.len(), 8);
        assert_eq!(flashing, expected);
    }

    #[test]
    fn test_step_1() {
        // given
        let input = vec_of_strings!["11111", "19991", "19191", "19991", "11111"];
        let mut opd = parse_octopodes(&input);

        // when
        let count = opd.step();

        // then
        assert_eq!(count, 9);
    }

    #[test]
    fn test_step_2() {
        // given
        let input = vec_of_strings!["11111", "19991", "19191", "19991", "11111"];
        let mut opd = parse_octopodes(&input);

        // when
        let first_count = opd.step();
        let second_count = opd.step();

        // then
        assert_eq!(first_count, 9);
        assert_eq!(second_count, 0);
    }

    #[test]
    fn test_count_flashes_1() {
        // given
        let input = vec_of_strings![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526"
        ];
        let mut opd = parse_octopodes(&input);

        // when
        let flashes = opd.count_flashes(1);

        // then
        assert_eq!(flashes, 0);
    }

    #[test]
    fn test_count_flashes_2() {
        // given
        let input = vec_of_strings![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526"
        ];
        let mut opd = parse_octopodes(&input);

        // when
        let flashes = opd.count_flashes(10);

        // then
        assert_eq!(flashes, 204);
    }

    #[test]
    fn test_count_flashes_3() {
        // given
        let input = vec_of_strings![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526"
        ];
        let mut opd = parse_octopodes(&input);

        // when
        let flashes = opd.count_flashes(100);

        // then
        assert_eq!(flashes, 1656);
    }

    #[test]
    fn test_first_sim() {
        // given
        let input = vec_of_strings![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526"
        ];
        let mut opd = parse_octopodes(&input);

        // when
        let flashes = opd.first_simultaneous();

        // then
        assert_eq!(flashes, 195);
    }
}
