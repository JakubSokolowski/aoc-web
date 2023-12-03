const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn run_first(input: &str) -> String {
    input
        .split('\n')
        .map(|s| s.to_string())
        .map(|l| {
            let digits_only = l.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
            let first_digit = digits_only.chars().next().unwrap();
            let last_digit = digits_only.chars().last().unwrap();

            let num = str::parse::<i64>(format!("{}{}", first_digit, last_digit).as_str()).unwrap();

            num
        })
        .sum::<i64>()
        .to_string()
}

pub fn run_second(input: &str) -> String {
    input
        .split('\n')
        .map(|s| s.to_string())
        .map(|l| {
            let first_digit = find_first_digit(&l);
            let last_digit = find_last_digit(&l);

            let num = str::parse::<i64>(format!("{}{}", first_digit, last_digit).as_str()).unwrap();

            num
        })
        .sum::<i64>()
        .to_string()
}

fn find_first_digit(line: &str) -> i64 {
    let digits_str = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for digit in digits_str {
        if line.starts_with(digit) {
            return get_digit_value(digit).unwrap();
        }
    }

    match line[0..1].parse() {
        Ok(val) => val,
        Err(_) => find_first_digit(&line[1..]),
    }
}

fn find_last_digit(line: &str) -> i64 {
    for digit in DIGITS {
        if line.ends_with(digit) {
            return get_digit_value(digit).unwrap();
        }
    }

    let last_c = &line[line.len() - 1..];

    match last_c.parse() {
        Ok(val) => val,
        Err(_) => {
            let new_line = &line[0..line.len() - 1];
            find_last_digit(new_line)
        }
    }
}

fn get_digit_value(digit: &str) -> Option<i64> {
    let idx = DIGITS.iter().position(|&d| d == digit).unwrap();
    Some((idx + 1) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_digit() {
        assert_eq!(find_first_digit("one"), 1);
        assert_eq!(find_first_digit("two"), 2);
        assert_eq!(find_first_digit("three"), 3);
        assert_eq!(find_first_digit("four"), 4);
        assert_eq!(find_first_digit("five"), 5);
        assert_eq!(find_first_digit("six"), 6);
        assert_eq!(find_first_digit("seven"), 7);
        assert_eq!(find_first_digit("eight"), 8);
        assert_eq!(find_first_digit("nine"), 9);
        assert_eq!(find_first_digit("1abc2"), 1);
        assert_eq!(find_first_digit("pqr3stu8vwx"), 3);
        assert_eq!(find_first_digit("a1b2c3d4e5f"), 1);
        assert_eq!(find_first_digit("treb7uchet"), 7);
        assert_eq!(
            find_first_digit("8sevengzfvjrhnsb6ddb8ninerkgkxthtfkvbcmqs"),
            8
        );
        assert_eq!(find_first_digit("1seven336"), 1);
    }

    #[test]
    fn test_find_last_digit() {
        assert_eq!(find_last_digit("two1nine"), 9);
        assert_eq!(find_last_digit("eightwothree"), 3);
        assert_eq!(find_last_digit("abcone2threexyz"), 3);
    }

    #[test]
    fn test_run_first() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(run_first(input), "142");
    }

    #[test]
    fn test_run_second() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(run_second(input), "281");
    }
}
