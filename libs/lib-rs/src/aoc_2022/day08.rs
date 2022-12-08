use itertools::Itertools;

use crate::common::parse::to_non_empty_lines;

pub fn run_first(input: &str) -> String {
    let grid = parse_grid(input);
    grid.count_visible().to_string()
}

pub fn run_second(input: &str) -> String {
    let grid = parse_grid(input);
    grid.max_viewing_distance().to_string()
}

#[derive(Debug, Clone)]
struct Grid {
    width: i64,
    height: i64,
    values: Vec<i64>,
}

impl Grid {
    fn get_index(&self, row: i64, column: i64) -> i64 {
        row * self.width + column
    }

    fn get_value(&self, row: i64, column: i64) -> i64 {
        match self.values.get(self.get_index(row, column) as usize) {
            Some(v) => *v,
            None => unreachable!("Out of bounds: {} {}", row, column),
        }
    }

    fn viewing_distance(&self, tree_row: i64, tree_column: i64) -> i64 {
        self.up(tree_row, tree_column)
            * self.down(tree_row, tree_column)
            * self.left(tree_row, tree_column)
            * self.right(tree_row, tree_column)
    }

    fn up(&self, tree_row: i64, tree_column: i64) -> i64 {
        if tree_row == 0 {
            return 0;
        }
        let tree_height = self.get_value(tree_row, tree_column);
        let mut visible_count = 0;
        let mut curr_row = tree_row - 1;

        loop {
            let val = self.get_value(curr_row, tree_column);
            visible_count += 1;
            curr_row -= 1;
            if curr_row < 0 || val >= tree_height {
                break;
            }
        }

        visible_count
    }

    fn down(&self, tree_row: i64, tree_column: i64) -> i64 {
        if tree_row == self.height - 1 {
            return 0;
        }
        let tree_height = self.get_value(tree_row, tree_column);
        let mut visible_count = 0;
        let mut curr_row = tree_row + 1;

        loop {
            let val = self.get_value(curr_row, tree_column);
            visible_count += 1;
            curr_row += 1;
            if curr_row == self.height || val >= tree_height {
                break;
            }
        }

        visible_count
    }

    fn left(&self, tree_row: i64, tree_column: i64) -> i64 {
        if tree_column == 0 {
            return 0;
        }
        let tree_height = self.get_value(tree_row, tree_column);
        let mut visible_count = 0;
        let mut curr_col = tree_column - 1;

        loop {
            let val = self.get_value(tree_row, curr_col);
            visible_count += 1;
            curr_col -= 1;
            if curr_col < 0 || val >= tree_height {
                break;
            }
        }

        visible_count
    }

    fn right(&self, tree_row: i64, tree_column: i64) -> i64 {
        if tree_column == self.width - 1 {
            return 0;
        }
        let tree_height = self.get_value(tree_row, tree_column);
        let mut visible_count = 0;
        let mut curr_col = tree_column + 1;

        loop {
            let val = self.get_value(tree_row, curr_col);
            visible_count += 1;
            curr_col += 1;
            if curr_col == self.width || val >= tree_height {
                break;
            }
        }

        visible_count
    }

    fn visible_horizontal(&self, tree_row: i64, tree_column: i64) -> bool {
        if self.is_edge(tree_row, tree_column) {
            return true;
        }

        let tree_height = self.get_value(tree_row, tree_column);
        let visible_before = (0..tree_column)
            .map(|col| {
                let val = self.get_value(tree_row, col);
                val < tree_height
            })
            .all(|b| b);

        let visible_after = (tree_column + 1..self.width)
            .map(|col| {
                let val = self.get_value(tree_row, col);
                val < tree_height
            })
            .all(|b| b);

        visible_before || visible_after
    }

    fn visible_vertical(&self, tree_row: i64, tree_column: i64) -> bool {
        if self.is_edge(tree_row, tree_column) {
            return true;
        }
        let tree_height = self.get_value(tree_row, tree_column);

        let visible_before = (0..tree_row)
            .map(|row| self.get_value(row, tree_column) < tree_height)
            .all(|b| b);

        let visible_after = (tree_row + 1..self.height)
            .map(|row| self.get_value(row, tree_column) < tree_height)
            .all(|b| b);

        visible_before || visible_after
    }

    fn is_visible(&self, tree_row: i64, tree_column: i64) -> bool {
        self.visible_vertical(tree_row, tree_column)
            || self.visible_horizontal(tree_row, tree_column)
    }

    fn count_visible(&self) -> i64 {
        (0..self.height)
            .cartesian_product(0..self.width)
            .filter(|&(r, c)| self.is_visible(r, c))
            .count() as i64
    }

    fn max_viewing_distance(&self) -> i64 {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(r, c)| self.viewing_distance(r, c))
            .max()
            .unwrap() as i64
    }

    fn is_edge(&self, row: i64, column: i64) -> bool {
        row == 0 || row == self.height - 1 || column == 0 || column == self.width - 1
    }
}

fn parse_grid(input: &str) -> Grid {
    let lines = to_non_empty_lines(input);
    let width = lines.len() as i64;
    let height = lines[0].len() as i64;

    let values: Vec<_> = lines
        .iter()
        .flat_map(|l| {
            l.split("")
                .filter(|c| !c.is_empty())
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    Grid {
        width,
        height,
        values,
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 8;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "1700";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "470596";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_distance() {
        // given
        let values = vec![
            3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
        ];
        let grid = Grid {
            values,
            width: 5,
            height: 5,
        };

        // when
        assert_eq!(grid.up(1, 2), 1);
        assert_eq!(grid.down(1, 2), 2);
        assert_eq!(grid.left(1, 2), 1);
        assert_eq!(grid.right(1, 2), 2);
        assert_eq!(grid.viewing_distance(1, 2), 4);

        assert_eq!(grid.viewing_distance(3, 2), 8);
    }

    #[test]
    fn test_visible() {
        // given
        let values = vec![
            3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
        ];
        let grid = Grid {
            values,
            width: 5,
            height: 5,
        };

        // when
        assert!(grid.is_visible(1, 2));
        assert!(!grid.is_visible(1, 3));
        assert!(grid.is_visible(1, 4));
        assert!(grid.is_visible(2, 1));
        assert!(!grid.is_visible(2, 2));
        assert!(grid.visible_horizontal(2, 3));
        assert!(grid.is_visible(2, 3));
    }
}
