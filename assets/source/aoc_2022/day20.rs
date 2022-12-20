use std::collections::VecDeque;

use crate::common::parse::parse_signed_numbers;

pub fn run_first(input: &str) -> String {
    let initial_numbers = parse_signed_numbers(input);
    mix_sum(1, &initial_numbers).to_string()
}

pub fn run_second(input: &str) -> String {
    let initial_numbers: Vec<_> = parse_signed_numbers(input)
        .iter()
        .map(|n| n * 811589153)
        .collect();
    mix_sum(10, &initial_numbers).to_string()
}

fn mix_sum(times: usize, vec: &[i64]) -> i64 {
    let mut deque: VecDeque<_> = vec.iter().cloned().enumerate().collect();

    for _ in 0..times {
        for (i, &n) in vec.iter().enumerate() {
            let idx = deque.iter().position(|&(a, b)| a == i && b == n).unwrap();
            deque.rotate_left(idx);
            deque.pop_front();
            if n <= 0 {
                deque.rotate_right((n.abs() % deque.len() as i64) as usize);
            } else {
                deque.rotate_left((n % deque.len() as i64) as usize);
            }
            deque.push_back((i, n));
        }
    }

    let values: Vec<_> = deque.into_iter().map(|(_, value)| value).collect();

    vec![1000, 2000, 3000]
        .iter()
        .map(|idx| {
            let pos = values.iter().position(|&a| a == 0).unwrap();
            values[(pos + idx) % values.len()]
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 20;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "17490";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "1632917375836";
        assert_eq!(result, expected.to_string());
    }
}
