use crate::common::parse::parse_numbers;

pub fn run_first(input: &str) -> String {
    let lines: Vec<String> = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    count_overlapping(&lines).to_string()
}

pub fn run_second(input: &str) -> String {
    let lines: Vec<String> = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    count_overlapping_with_diagonal(&lines).to_string()
}

#[derive(Debug, Clone)]
pub struct Line {
    start_x: i64,
    start_y: i64,
    end_x: i64,
    end_y: i64,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start_y == self.end_y
    }

    fn is_vertical(&self) -> bool {
        self.start_x == self.end_x
    }

    fn max_x(&self) -> i64 {
        self.start_x.max(self.end_x)
    }

    fn max_y(&self) -> i64 {
        self.start_y.max(self.end_y)
    }

    fn delta_x(&self) -> i64 {
        self.end_x - self.start_x
    }

    fn delta_y(&self) -> i64 {
        self.end_y - self.start_y
    }

    fn length(&self) -> i64 {
        i64::max(self.delta_x().abs(), self.delta_y().abs())
    }
}

#[derive(Debug, Clone)]
struct Grid {
    width: i64,
    values: Vec<i64>,
}

impl Grid {
    pub fn new(width: i64, height: i64) -> Grid {
        let values = vec![0; (width * height) as usize];

        Grid { width, values }
    }

    fn get_index(&self, x: i64, y: i64) -> i64 {
        y * self.width + x
    }

    fn inc_value(&mut self, row: i64, column: i64) {
        let index = self.get_index(row, column);
        self.values[index as usize] += 1
    }

    fn draw_lines(&mut self, lines: &[Line]) {
        for line in lines {
            self.draw_line(line)
        }
    }

    fn draw_line(&mut self, line: &Line) {
        let x_delta = (line.delta_x()).signum();
        let y_delta = (line.delta_y()).signum();
        let len = line.length();

        for n in 0..=len {
            let row = line.start_y + n * y_delta;
            let column = line.start_x + n * x_delta;
            self.inc_value(row, column);
        }
    }

    fn count_overlapping(&self) -> i64 {
        self.values.iter().filter(|&&v| v > 1).count() as i64
    }
}

fn parse_line(input: &str) -> Line {
    let values = parse_numbers(input);
    Line {
        start_x: values[0],
        start_y: values[1],
        end_x: values[2],
        end_y: values[3],
    }
}

pub fn count_overlapping(input: &[String]) -> i64 {
    let lines: Vec<_> = input
        .iter()
        .map(|l| parse_line(l))
        .filter(|l| l.is_vertical() || l.is_horizontal())
        .collect();
    count_overlapping_points(&lines)
}

pub fn count_overlapping_points(lines: &[Line]) -> i64 {
    let (width, height) = max_coords(lines);
    let mut grid = Grid::new(width + 1, height + 1);
    grid.draw_lines(lines);
    grid.count_overlapping()
}

pub fn count_overlapping_with_diagonal(input: &[String]) -> i64 {
    let lines: Vec<_> = input.iter().map(|l| parse_line(l)).collect();
    count_overlapping_points(&lines)
}

pub fn max_coords(lines: &[Line]) -> (i64, i64) {
    let mut max_x = 0;
    let mut max_y = 0;

    for line in lines {
        max_x = max_x.max(line.max_x());
        max_y = max_y.max(line.max_y());
    }

    (max_x, max_y)
}

#[cfg(test)]
mod tests {
    use crate::common::parse::test_utils::vec_of_strings;
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u8 = 5;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "6572";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "21466";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_draw_line() {
        // given
        let line = Line {
            start_x: 1,
            start_y: 1,
            end_x: 3,
            end_y: 1,
        };
        let mut grid = Grid::new(4, 4);

        // when
        grid.draw_line(&line);
        assert_eq!(grid.count_overlapping(), 0);

        grid.draw_line(&line);
        assert_eq!(grid.count_overlapping(), 3);
    }

    #[test]
    fn test_count_overlapping() {
        // given
        let lines = vec_of_strings![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2"
        ];

        // when
        let result = count_overlapping(&lines);

        //then
        assert_eq!(result, 5);
    }

    #[test]
    fn test_count_overlapping_with_diagonal() {
        // given
        let lines = vec_of_strings![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2"
        ];

        // when
        let result = count_overlapping_with_diagonal(&lines);

        //then
        assert_eq!(result, 12);
    }
}
