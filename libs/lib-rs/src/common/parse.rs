use lazy_static::lazy_static;
use regex::Regex;

#[cfg(test)]
pub mod test_utils {
    #[macro_export]
    macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}
    #[allow(clippy::all)]
    pub(crate) use vec_of_strings;
}

pub fn parse_numbers(input: &str) -> Vec<i64> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    RE.find_iter(input)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect()
}

pub fn to_non_empty_lines(input: &str) -> Vec<String> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect()
}
