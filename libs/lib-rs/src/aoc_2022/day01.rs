use itertools::Itertools;

pub fn run_first(input: &str) -> String {
    let as_lines: Vec<String> = input.split('\n').map(|s| s.to_string()).collect();

    top_n_sum(&as_lines, 1).to_string()
}

pub fn run_second(input: &str) -> String {
    let as_lines: Vec<String> = input.split('\n').map(|s| s.to_string()).collect();

    top_n_sum(&as_lines, 3).to_string()
}

pub fn get_calories(input: &[String]) -> Vec<i64> {
    let mut res: Vec<i64> = vec![];
    let mut curr = 0;
    for line in input {
        if line.is_empty() {
            res.push(curr);
            curr = 0;
        } else {
            curr += line.parse::<i64>().unwrap();
        }
    }
    res
}

pub fn top_n_sum(input: &[String], n: usize) -> i64 {
    get_calories(input)
        .iter()
        .sorted_by(|a, b| b.cmp(a))
        .take(n)
        .sum()
}
