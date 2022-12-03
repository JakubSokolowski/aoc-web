use std::collections::HashSet;

use crate::common::parse::parse_numbers;

pub fn run_first(input: &str) -> String {
    let (bingo_line, matrices) = parse(input);
    first_bingo_winner(&bingo_line, matrices)
}

pub fn run_second(input: &str) -> String {
    let (bingo_line, matrices) = parse(input);
    last_bingo_winner(&bingo_line, matrices)
}

fn first_bingo_winner(bingo_line: &[i64], mut matrices: Vec<BingoBoard>) -> String {
    let mut result = "".to_string();
    for num in bingo_line {
        for m in matrices.iter_mut() {
            m.mark(*num);
            if m.has_bingo() {
                result = (m.sum_unmarked() * num).to_string();
                return result;
            }
        }
    }
    result
}

fn last_bingo_winner(bingo_line: &[i64], mut matrices: Vec<BingoBoard>) -> String {
    let mut num_winners = 0;
    let num_players = matrices.len();
    let mut solved: HashSet<i64> = HashSet::new();

    let mut result = "".to_string();

    for num in bingo_line {
        for (idx, player_matrix) in matrices.iter_mut().enumerate() {
            player_matrix.mark(*num);

            if player_matrix.has_bingo() && !solved.contains(&(idx as i64)) {
                num_winners += 1;
                solved.insert(idx as i64);

                if num_winners == num_players {
                    result = (player_matrix.sum_unmarked() * num).to_string();
                    return result;
                }
            }
        }
    }

    result
}

fn parse(input: &str) -> (Vec<i64>, Vec<BingoBoard>) {
    let bingo_line: Vec<i64> = input
        .split("\n\n")
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    let matrices: Vec<BingoBoard> = input.split("\n\n").skip(1).map(parse_matrix).collect();

    (bingo_line, matrices)
}

fn parse_matrix(input: &str) -> BingoBoard {
    let values = parse_numbers(input);
    BingoBoard::new(values)
}

#[derive(Debug, Clone)]
struct BingoBoard {
    width: i64,
    height: i64,
    values: Vec<BingoNum>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct BingoNum {
    marked: bool,
    value: i64,
}

impl BingoBoard {
    pub fn new(values: Vec<i64>) -> BingoBoard {
        let width = (values.len() as f64).sqrt() as i64;
        let height = width;

        BingoBoard {
            width,
            height,
            values: values
                .into_iter()
                .map(|v| BingoNum {
                    value: v,
                    marked: false,
                })
                .collect(),
        }
    }

    fn get_index(&self, row: i64, column: i64) -> i64 {
        row * self.width + column
    }

    fn get_value(&self, row: i64, column: i64) -> BingoNum {
        match self.values.get(self.get_index(row, column) as usize) {
            Some(v) => *v,
            None => BingoNum {
                value: 0,
                marked: false,
            },
        }
    }

    fn mark(&mut self, number: i64) {
        match self.values.iter().position(|n| n.value == number) {
            None => {}
            Some(index) => self.values.get_mut(index).unwrap().marked = true,
        };
    }

    fn any_column_has_bingo(&self) -> bool {
        (0..self.width)
            .map(|column| self.column_has_bingo(column))
            .any(|bingo| bingo)
    }

    fn column_has_bingo(&self, column: i64) -> bool {
        (0..self.height)
            .map(|row| self.get_value(row, column))
            .all(|n| n.marked)
    }

    fn any_row_has_bingo(&self) -> bool {
        (0..self.height)
            .map(|column| self.row_has_bingo(column))
            .any(|bingo| bingo)
    }

    fn row_has_bingo(&self, row: i64) -> bool {
        (0..self.width)
            .map(|column| self.get_value(row, column))
            .all(|n| n.marked)
    }

    fn has_bingo(&self) -> bool {
        self.any_row_has_bingo() || self.any_column_has_bingo()
    }

    fn sum_unmarked(&self) -> i64 {
        self.values
            .iter()
            .filter_map(|&n| if !n.marked { Some(n.value) } else { None })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u8 = 4;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "58412";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "10030";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_mark_marks_value_as_bingoed() {
        // given
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut board = BingoBoard::new(values);

        // when
        board.mark(3);
        let result = board.get_value(0, 2);

        // then
        let expected = BingoNum {
            value: 3,
            marked: true,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn any_row_has_bingo_returns_returns_true_if_all_nums_in_some_row_are_marked() {
        // given
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut board = BingoBoard::new(values);

        // when
        board.mark(4);
        board.mark(5);
        board.mark(6);
        let row_bingo = board.any_row_has_bingo();
        let column_bingo = board.any_column_has_bingo();

        // then
        let expected_row = true;
        let expected_column = false;
        assert_eq!(row_bingo, expected_row);
        assert_eq!(column_bingo, expected_column);
    }

    #[test]
    fn any_row_has_bingo_returns_returns_true_if_all_nums_in_some_column_are_marked() {
        // given
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut board = BingoBoard::new(values);

        // when
        board.mark(3);
        board.mark(6);
        board.mark(9);
        let row_bingo = board.any_row_has_bingo();
        let column_bingo = board.any_column_has_bingo();

        // then
        let expected_row = false;
        let expected_column = true;
        assert_eq!(row_bingo, expected_row);
        assert_eq!(column_bingo, expected_column);
    }
}
