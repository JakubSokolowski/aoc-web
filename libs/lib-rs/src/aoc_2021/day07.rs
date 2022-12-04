use crate::common::parse::parse_numbers;

pub fn run_first(input: &str) -> String {
    part_1(input).to_string()
}

pub fn run_second(input: &str) -> String {
    part_2(input).to_string()
}

pub fn part_1(input: &str) -> i64 {
    let crabs = parse_numbers(input);
    let max_pos = crabs.iter().max().unwrap();
    let mut min_fuel = i64::MAX;

    for pos in 0..=*max_pos {
        let fuel = total_fuel(pos, &crabs);
        if fuel < min_fuel {
            min_fuel = fuel
        }
    }

    min_fuel
}

pub fn part_2(input: &str) -> i64 {
    let crabs = parse_numbers(input);
    let max_pos = crabs.iter().max().unwrap();

    let mut min_fuel = i64::MAX;

    for pos in 0..=*max_pos {
        let fuel = total_fuel_2(pos, &crabs);
        if fuel < min_fuel {
            min_fuel = fuel
        }
    }

    min_fuel
}

pub fn total_fuel(position: i64, crabs: &[i64]) -> i64 {
    crabs.iter().map(|c| (position - *c).abs()).sum()
}

pub fn total_fuel_2(position: i64, crabs: &[i64]) -> i64 {
    crabs.iter().map(|c| gaussian((position - *c).abs())).sum()
}

fn gaussian(n: i64) -> i64 {
    (n * (n + 1)) / 2
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u8 = 7;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "342641";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "93006301";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_1_example() {
        // given
        let input = "16,1,2,0,4,2,7,1,2,14";

        // when
        let result = part_1(input);

        // then
        let expected = 37_i64;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_2_example() {
        // given
        let input = "16,1,2,0,4,2,7,1,2,14";

        // when
        let result = part_2(input);

        // then
        let expected = 168_i64;
        assert_eq!(result, expected);
    }
}
