use std::collections::HashSet;

use itertools::Itertools;

use crate::common::parse::to_non_empty_lines;

pub fn run_first(input: &str) -> String {
    let lines = to_non_empty_lines(input);
    let heightmap = parse_heightmap(&lines);
    heightmap.sum_risk().to_string()
}

pub fn run_second(input: &str) -> String {
    let lines = to_non_empty_lines(input);
    let heightmap = parse_heightmap(&lines);
    heightmap.largest_basins_product(3).to_string()
}

struct HeightMap {
    width: i32,
    height: i32,
    values: Vec<i32>,
}

impl HeightMap {
    pub fn new(values: Vec<i32>, width: i32, height: i32) -> HeightMap {
        HeightMap {
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

    fn is_low_point(&self, row: i32, column: i32) -> bool {
        self.get_adjacent_values(row, column)
            .iter()
            .all(|&v| v > self.get_value(row, column))
    }

    fn get_adjacent_values(&self, row: i32, column: i32) -> Vec<i32> {
        let coords = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];

        coords
            .into_iter()
            .filter_map(|(dr, dc)| {
                let index = self.get_index(row + dr, column + dc);
                self.values.get(index)
            })
            .cloned()
            .collect()
    }

    fn get_adjacent_points(
        &self,
        row: i32,
        column: i32,
        basin: &HashSet<(i32, i32)>,
    ) -> Vec<(i32, i32)> {
        let coords = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];

        coords
            .into_iter()
            .map(|(dr, dc)| (row + dr, column + dc))
            .filter(|p| {
                let &(r, c) = p;

                if r < 0 || c < 0 {
                    return false;
                }

                if r >= self.height || c >= self.width {
                    return false;
                }

                if basin.contains(p) {
                    return false;
                }

                let index = self.get_index(r, c);
                match self.values.get(index) {
                    None => false,
                    Some(value) => *value != 9,
                }
            })
            .collect()
    }

    fn find_low_points_values(&self) -> Vec<i32> {
        self.find_low_points()
            .iter()
            .map(|&(r, c)| self.get_value(r, c))
            .collect()
    }

    fn find_low_points(&self) -> Vec<(i32, i32)> {
        (0..self.height)
            .cartesian_product(0..self.width)
            .filter(|&(r, c)| self.is_low_point(r, c))
            .collect()
    }

    fn sum_risk(&self) -> i32 {
        let low_points = self.find_low_points_values();

        low_points.iter().sum::<i32>() + low_points.len() as i32
    }

    fn get_basin_points(&self, low_point: (i32, i32)) -> HashSet<(i32, i32)> {
        let mut basin: HashSet<(i32, i32)> = HashSet::from([low_point]);
        // add initial low point point to basin
        basin.insert(low_point);

        // add initial point to outer points
        let mut outer_points: Vec<_> = vec![low_point];
        loop {
            // get points adjacent to outer points (points that are already not in basin and that are not 9)
            let adjacent_to_outer = self.get_adjacent_to_outer(&outer_points, &basin);
            // if there are no such points, break
            if adjacent_to_outer.is_empty() {
                break;
            }
            // add points to basin
            basin.extend(adjacent_to_outer.iter());
            // set these points as outer points
            outer_points = adjacent_to_outer;
        }

        // return basin
        basin
    }

    fn largest_basins_product(&self, num_basins: usize) -> usize {
        self.find_low_points()
            .iter()
            .map(|&p| self.get_basin_points(p).len())
            .sorted()
            .rev()
            .take(num_basins)
            .product()
    }

    fn get_adjacent_to_outer(
        &self,
        points: &[(i32, i32)],
        basin: &HashSet<(i32, i32)>,
    ) -> Vec<(i32, i32)> {
        points
            .iter()
            .flat_map(|&(r, c)| self.get_adjacent_points(r, c, basin))
            .unique()
            .collect()
    }
}

fn parse_heightmap(input: &[String]) -> HeightMap {
    let width = parse_line(&input[0]).len() as i32;
    let height = input.len() as i32;
    let values: Vec<i32> = input.iter().flat_map(|l| parse_line(l)).collect();

    HeightMap::new(values, width, height)
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split("")
        .filter(|&x| !x.is_empty())
        .map(|n| n.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::common::parse::test_utils::vec_of_strings;
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u8 = 9;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "502";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "1330560";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_get_adjacent_1() {
        // given
        let input = vec_of_strings![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"
        ];
        let heightmap = parse_heightmap(&input);

        // when
        let result = heightmap.get_adjacent_values(0, 1);

        // then
        let expected: Vec<i32> = vec![9, 2, 9];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_adjacent_2() {
        // given
        let input = vec_of_strings![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"
        ];
        let heightmap = parse_heightmap(&input);

        // when
        let result = heightmap.get_adjacent_values(1, 1);

        // then
        let expected: Vec<i32> = vec![8, 3, 1, 8];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_low_point_1() {
        // given
        let input = vec_of_strings![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"
        ];
        let heightmap = parse_heightmap(&input);

        // when
        let result = heightmap.is_low_point(0, 1);

        // then
        assert_eq!(result, true);
    }

    #[test]
    fn test_find_low_points_1() {
        // given
        let input = vec_of_strings![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"
        ];
        let heightmap = parse_heightmap(&input);

        // when
        let result = heightmap.find_low_points_values();

        // then
        let expected: Vec<i32> = vec![1, 0, 5, 5];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_risk_1() {
        // given
        let input = vec_of_strings![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"
        ];
        let heightmap = parse_heightmap(&input);

        // when
        let result = heightmap.sum_risk();

        // then
        let expected = 15;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_adjacent_to_outer_1() {
        // given
        let input = vec_of_strings![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"
        ];
        let heightmap = parse_heightmap(&input);
        let basin: HashSet<(i32, i32)> = HashSet::from([(0, 1)]);
        let outer: Vec<(i32, i32)> = vec![(0, 1)];

        // when
        let result = heightmap.get_adjacent_to_outer(&outer, &basin);

        // then
        let expected: Vec<(i32, i32)> = vec![(0, 0)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_adjacent_to_outer_2() {
        // given
        let input = vec_of_strings![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"
        ];
        let heightmap = parse_heightmap(&input);
        let basin: HashSet<(i32, i32)> = HashSet::from([(0, 1), (0, 0)]);
        let outer: Vec<(i32, i32)> = vec![(0, 0)];

        // when
        let result = heightmap.get_adjacent_to_outer(&outer, &basin);

        // then
        let expected: Vec<(i32, i32)> = vec![(1, 0)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_adjacent_to_outer_3() {
        // given
        let input = vec_of_strings![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"
        ];
        let heightmap = parse_heightmap(&input);
        let basin: HashSet<(i32, i32)> = HashSet::from([(0, 1)]);
        let outer: Vec<(i32, i32)> = vec![(0, 0)];

        // when
        let result = heightmap.get_adjacent_to_outer(&outer, &basin);

        // then
        let expected: Vec<(i32, i32)> = vec![(1, 0)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_adjacent_to_outer_4() {
        // given
        let input = vec_of_strings![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"
        ];
        let heightmap = parse_heightmap(&input);
        let basin: HashSet<(i32, i32)> = HashSet::from([(0, 9)]);
        let outer: Vec<(i32, i32)> = vec![(0, 9)];

        // when
        let result = heightmap.get_adjacent_to_outer(&outer, &basin);

        // then
        let expected: Vec<(i32, i32)> = vec![(0, 8), (1, 9)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_basin_points_1() {
        // given
        let input = vec_of_strings![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"
        ];
        let heightmap = parse_heightmap(&input);
        let low_point = (0, 1);

        // when
        let result = heightmap.get_basin_points(low_point);

        // then
        let expected: HashSet<(i32, i32)> = HashSet::from([(1, 0), (0, 1), (0, 0)]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_basin_points_2() {
        // given
        let input = vec_of_strings![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"
        ];
        let heightmap = parse_heightmap(&input);
        let low_point = (2, 2);

        // when
        let result = heightmap.get_basin_points(low_point);

        // then
        assert_eq!(result.len(), 14);
    }

    #[test]
    fn test_get_basin_points_3() {
        // given
        let input = vec_of_strings![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"
        ];
        let heightmap = parse_heightmap(&input);
        let low_point = (0, 9);

        // when
        let result = heightmap.get_basin_points(low_point);

        // then
        let expected: HashSet<(i32, i32)> = HashSet::from([
            (1, 6),
            (0, 7),
            (1, 9),
            (0, 8),
            (0, 6),
            (0, 9),
            (0, 5),
            (1, 8),
            (2, 9),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_largest_basins_product() {
        // given
        let input = vec_of_strings![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"
        ];
        let heightmap = parse_heightmap(&input);

        // when
        let result = heightmap.largest_basins_product(3);

        assert_eq!(result, 1134);
    }
}
