pub fn run_first(input: &str) -> String {
    let as_num: Vec<usize> = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    let count_sliding = count_sliding_increased(&as_num);
    count_sliding.to_string()
}

pub fn count_increased(input: &[usize]) -> usize {
    input.windows(2).map(|w| usize::from(w[1] > w[0])).sum()
}

pub fn run_second(input: &str) -> String {
    let as_num: Vec<usize> = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    let count = count_increased(&as_num);
    count.to_string()
}

pub fn count_sliding_increased(input: &[usize]) -> usize {
    input
        .windows(4)
        .map(|w| {
            let prev = w[0] + w[1] + w[2];
            let curr = w[1] + w[2] + w[3];
            usize::from(curr > prev)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts_increasing_measurements() {
        // given
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        // when
        let result = count_increased(&measurements);

        // then
        let expected = 7;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_counts_sliding_window_increasing_measurements() {
        // given
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        // when
        let result = count_sliding_increased(&measurements);

        // then
        let expected = 5;
        assert_eq!(result, expected);
    }
}
