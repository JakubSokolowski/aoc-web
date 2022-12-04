use crate::common::parse::parse_numbers;

pub fn run_first(input: &str) -> String {
    let days = 80;
    population_size(input, days).to_string()
}

pub fn run_second(input: &str) -> String {
    let days = 256;
    population_size(input, days).to_string()
}

fn population_size(input: &str, num_days: i64) -> i64 {
    let fish = parse_numbers(input);
    let mut groups: Vec<i64> = vec![0; 9];

    for fish in fish {
        groups[fish as usize] += 1;
    }

    for _ in 0..num_days {
        groups[7] += groups[0];
        groups.rotate_left(1);
    }

    groups.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_to_string;

    const YEAR: u32 = 2021;
    const DAY: u8 = 6;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "372300";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "1675781200288";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_population_size_example_1() {
        // given
        let input = "3,4,3,1,2";
        let num_days = 18;

        // when
        let result = population_size(input, num_days);

        // then
        assert_eq!(result, 26)
    }

    #[test]
    fn test_population_size_v2_example_1() {
        // given
        let input = "3,4,3,1,2";

        // then
        assert_eq!(population_size(input, 1), 5);
        assert_eq!(population_size(input, 2), 6);
        assert_eq!(population_size(input, 3), 7);
        assert_eq!(population_size(input, 4), 9);
        assert_eq!(population_size(input, 5), 10);
        assert_eq!(population_size(input, 6), 10);
        assert_eq!(population_size(input, 7), 10);
        assert_eq!(population_size(input, 8), 10);
        assert_eq!(population_size(input, 9), 11);
        assert_eq!(population_size(input, 18), 26);
    }
}
