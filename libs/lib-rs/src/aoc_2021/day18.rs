use itertools::Itertools;

use crate::common::parse::to_non_empty_lines;

pub fn run_first(input: &str) -> String {
    let lines = to_non_empty_lines(input);
    part_1(&lines)
}

pub fn run_second(input: &str) -> String {
    let lines = to_non_empty_lines(input);
    part_2(&lines)
}

fn part_1(input: &[String]) -> String {
    let final_sum = add_all(input);
    magnitude(&final_sum).to_string()
}

fn part_2(input: &[String]) -> String {
    input
        .iter()
        .permutations(2)
        .map(|p| {
            let left = p[0].to_string();
            let right = p[1].to_string();
            magnitude(&add_and_process(&left, &right))
        })
        .max()
        .unwrap()
        .to_string()
}

fn pair_values(num: &str, index: usize) -> (i32, i32) {
    let pair_str: String = num
        .chars()
        .skip(index + 1)
        .take_while(|&c| c != ']')
        .collect();

    let values: Vec<_> = pair_str
        .split(',')
        .map(|p| p.parse::<i32>().unwrap())
        .collect();

    (values[0], values[1])
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PairInfo {
    left: i32,
    right: i32,
    index: usize,
}

impl PairInfo {
    fn str_rep(&self) -> String {
        format!("[{},{}]", self.left, self.right)
    }
}

fn leftmost_pair(num: &str, level: usize) -> Option<PairInfo> {
    leftmost_lvl_index(num, level).map(|index| {
        let (left, right) = pair_values(num, index);
        PairInfo { left, right, index }
    })
}

fn can_explode(num: &str) -> bool {
    leftmost_pair(num, 4).is_some()
}

fn explode(num: &str) -> String {
    match leftmost_pair(num, 4) {
        None => num.to_string(),
        Some(pair) => {
            let start = pair.index;
            let end = start + num.chars().skip(start).take_while(|&c| c != ']').count();
            let can_right = can_explode_right(num, end);
            let can_left = can_explode_left(num, start);

            if can_right && can_left {
                let (left_part, after_left) = left_part(num, pair, start);
                let (right_part, after_right) = right_part(num, pair, end);
                let to_replace = format!("{left_part}[{},{}]{right_part}", pair.left, pair.right);
                let replace_with = format!("{after_left}0{after_right}");

                return num.replacen(&to_replace, &replace_with, 1);
            }

            if can_right {
                explode_right(num, pair, end)
            } else {
                explode_left(num, pair, start)
            }
        }
    }
}

fn left_part(num: &str, pair: PairInfo, start: usize) -> (String, String) {
    let part = num
        .chars()
        .rev()
        .skip(num.len() - start)
        .take_while(|c| !c.is_ascii_digit())
        .collect::<String>();

    let num_str = num
        .chars()
        .rev()
        .skip(num.len() - start + part.len())
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();

    let rev_part: String = part.chars().rev().collect();
    let rev_num_str: String = num_str.chars().rev().collect();

    let left_num = rev_num_str.parse::<i32>().unwrap();
    let new_num = left_num + pair.left;

    // first new number, then old part
    let before = format!("{rev_num_str}{rev_part}");
    let after = format!("{new_num}{rev_part}");

    (before, after)
}

fn right_part(num: &str, pair: PairInfo, end: usize) -> (String, String) {
    let part = num
        .chars()
        .skip(end)
        .take_while(|c| !c.is_ascii_digit())
        .collect::<String>();
    let num_str = num
        .chars()
        .skip(end + part.len())
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();
    let num: i32 = num_str.parse().unwrap();
    let after = format!("{}{}", &part[1..], num + pair.right);
    (format!("{}{}", &part[1..], num_str), after)
}

fn split(num: &str) -> String {
    split_num(num, find_split(num).unwrap())
}

fn split_num(num: &str, split: i32) -> String {
    let half = (split as f64) / 2.0;
    let left = half.floor() as i32;
    let right = half.ceil() as i32;
    let to_replace = format!("{split}");
    let replace_with = format!("[{left},{right}]");
    num.replacen(&to_replace, &replace_with, 1)
}

fn add(left: &str, right: &str) -> String {
    format!("[{left},{right}]")
}

fn add_all(pairs: &[String]) -> String {
    let mut curr = pairs.iter().next().unwrap().clone();
    for pair in pairs.iter().skip(1) {
        curr = add_and_process(&curr, pair)
    }
    curr
}

fn add_and_process(left: &str, right: &str) -> String {
    process(&add(left, right))
}

fn process(num: &str) -> String {
    let mut curr_num = num.to_string();
    loop {
        if can_explode(&curr_num) {
            curr_num = explode(&curr_num);
            continue;
        }

        if can_split(&curr_num) {
            curr_num = split(&curr_num);
            continue;
        }
        break;
    }
    curr_num
}

fn can_split(num: &str) -> bool {
    find_split(num).is_some()
}

fn find_split(num: &str) -> Option<i32> {
    let mut digits = "".to_string();
    for c in num.chars() {
        if c.is_ascii_digit() {
            digits.push_str(&c.to_string())
        } else {
            digits = "".to_string();
        }

        if digits.len() > 1 {
            return Some(digits.parse().unwrap());
        }
    }

    None
}

fn can_explode_right(num: &str, pair_end: usize) -> bool {
    num.chars().skip(pair_end).any(|c| c != ']')
}

fn can_explode_left(num: &str, pair_start: usize) -> bool {
    num.chars()
        .rev()
        .skip(num.len() - (pair_start - 1))
        .any(|c| c != '[')
}

fn explode_left(num: &str, pair: PairInfo, start: usize) -> String {
    let left_num_str = num
        .chars()
        .rev()
        .skip(num.len() - (start - 1))
        .take_while(|&c| c != '[')
        .collect::<String>();

    let rev = left_num_str.chars().rev().collect::<String>();
    let left_num = rev.parse::<i32>().unwrap();

    let new_left = left_num + pair.left;
    let to_replace = format!("[{left_num},[{},{}]]", pair.left, pair.right);
    let replace_with = format!("[{new_left},0]");
    num.replacen(&to_replace, &replace_with, 1)
}

fn explode_right(num: &str, pair: PairInfo, end: usize) -> String {
    let (before, after) = right_part(num, pair, end);
    let to_replace = format!("{}{}", pair.str_rep(), before);
    let replace_with = format!("0{after}");
    num.replacen(&to_replace, &replace_with, 1)
}

fn leftmost_lvl_index(num: &str, level: usize) -> Option<usize> {
    let mut open_count = 0;
    for (idx, c) in num.chars().enumerate() {
        match c {
            '[' => open_count += 1,
            ']' => open_count -= 1,
            _ => {}
        }
        if open_count == level + 1 {
            return Some(idx);
        }
    }

    None
}

fn find_pivot_index(num: &str) -> usize {
    let mut open_count = num.chars().take_while(|&c| c == '[').count();

    if open_count == 0 {
        return 0;
    }

    let mut close_count = 0;

    for (idx, c) in num.chars().enumerate().skip(open_count) {
        if close_count == open_count - 1 && c == ',' {
            return idx;
        }

        if idx != 0 {
            match c {
                '[' => open_count += 1,
                ']' => close_count += 1,
                _ => {}
            }
        }
    }

    0
}

fn magnitude(num: &str) -> i32 {
    if num.is_empty() {
        return 0;
    }
    let pivot_index = find_pivot_index(num);
    let mut stripped = "";

    if pivot_index != 0 {
        stripped = &num[1..num.len() - 1];
    }

    let (left_str, mut right_str) = stripped.split_at(pivot_index - 1);

    right_str = &right_str[1..];
    let left = left_str.parse::<i32>();
    let right = right_str.parse::<i32>();

    let left_res = match left {
        Ok(res) => 3 * res,
        Err(_) => 3 * magnitude(left_str),
    };

    let right_res = match right {
        Ok(res) => 2 * res,
        Err(_) => 2 * magnitude(right_str),
    };

    left_res + right_res
}

#[cfg(test)]
mod tests {
    use crate::common::parse::test_utils::vec_of_strings;
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u8 = 18;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "4132";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "4685";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_lvl_index() {
        let input = "[[[[[9,8],1],2],3],4]";
        let idx = leftmost_lvl_index(input, 4).unwrap();
        pair_values(input, idx);
        assert_eq!(idx, 4)
    }

    #[test]
    fn test_pair_values_1() {
        let input = "[[[[[9,8],1],2],3],4]";
        let values = pair_values(input, 4);
        assert_eq!((9, 8), values)
    }

    #[test]
    fn test_leftmost_level_1() {
        let input = "[[[[[9,8],1],2],3],4]";
        let values = leftmost_pair(input, 4).unwrap();
        assert_eq!(
            PairInfo {
                left: 9,
                right: 8,
                index: 4,
            },
            values
        )
    }

    #[test]
    fn test_leftmost_level_2() {
        let input = "[7,[6,[5,[4,[3,2]]]]]";
        let values = leftmost_pair(input, 4).unwrap();
        assert_eq!(
            PairInfo {
                left: 3,
                right: 2,
                index: 12,
            },
            values
        )
    }

    #[test]
    fn test_leftmost_level_3() {
        let input = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
        let values = leftmost_pair(input, 4).unwrap();
        assert_eq!(
            PairInfo {
                left: 7,
                right: 3,
                index: 10,
            },
            values
        )
    }

    #[test]
    fn test_can_explode_right() {
        assert_eq!(can_explode_right("[[6,[5,[4,[3,2]]]],1]", 14), true);
        assert_eq!(can_explode_right("[7,[6,[5,[4,[3,2]]]]]", 16), false);
    }

    #[test]
    fn test_can_explode_left() {
        assert_eq!(can_explode_left("[[[[[9,8],1],2],3],4]", 4), false);
        assert_eq!(can_explode_left("[[6,[5,[4,[3,2]]]],1]", 11), true);
    }

    #[test]
    fn test_explode_pair_right() {
        let input = "[[[[[9,8],1],2],3],4]";
        let exploded = explode(input);
        assert_eq!(exploded, "[[[[0,9],2],3],4]")
    }

    #[test]
    fn test_explode_pair_right_2() {
        let input = "[[6,[5,[4,[3,2]]]],1]";
        let exploded = explode(input);
        assert_eq!(exploded, "[[6,[5,[7,0]]],3]")
    }

    #[test]
    fn test_explode_pair_left() {
        let input = "[7,[6,[5,[4,[3,2]]]]]";
        let exploded = explode(input);
        assert_eq!(exploded, "[7,[6,[5,[7,0]]]]")
    }

    #[test]
    fn test_explode_multiple_lvl_4() {
        let input = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
        let exploded = explode(input);
        assert_eq!(exploded, "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
    }

    #[test]
    fn test_explode_multiple_2_explodes() {
        let input = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let exploded = explode(input);
        assert_eq!(exploded, "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
    }

    #[test]
    fn test_explode_2_deep_4_me() {
        let input = "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]";
        let exploded = explode(input);
        assert_eq!(exploded, "[[[[0,7],4],[15,[0,13]]],[1,1]]")
    }

    #[test]
    fn test_explode_addition() {
        let input = "[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]]";
        let exploded = explode(input);
        assert_eq!(exploded, "[[[[0,[3,2]],[3,3]],[4,4]],[5,5]]")
    }

    #[test]
    fn test_find_split() {
        let input = "[[3,[2,[10,11]]],[9,[5,[4,[3,2]]]]]";
        let exploded = find_split(input).unwrap();
        assert_eq!(exploded, 10)
    }

    #[test]
    fn test_split() {
        let input = "[11,0]";
        let exploded = split_num(input, 11);
        assert_eq!(exploded, "[[5,6],0]")
    }

    #[test]
    fn test_can_split() {
        let input = "[[[[0,7],4],[15,[0,13]]],[1,1]]";
        assert_eq!(can_split(input), true);
    }

    #[test]
    fn test_process() {
        let input = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
        assert_eq!(process(input), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    }

    #[test]
    fn test_end_pr() {
        let input = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
        assert_eq!(process(input), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    }

    #[test]
    fn test_add_all_1() {
        let input = vec_of_strings!["[1,1]", "[2,2]", "[3,3]", "[4,4]"];
        assert_eq!(add_all(&input), "[[[[1,1],[2,2]],[3,3]],[4,4]]")
    }

    #[test]
    fn test_add_all_2() {
        let input = vec_of_strings!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"];
        assert_eq!(add_all(&input), "[[[[3,0],[5,3]],[4,4]],[5,5]]")
    }

    #[test]
    fn test_add_all_3() {
        let input = vec_of_strings!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"];
        assert_eq!(add_all(&input), "[[[[5,0],[7,4]],[5,5]],[6,6]]")
    }

    #[test]
    fn test_add_all_4() {
        let input = vec_of_strings![
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]"
        ];
        assert_eq!(
            add_all(&input),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        )
    }

    #[test]
    fn test_add_all_5() {
        let input = vec_of_strings![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
        ];
        assert_eq!(
            add_all(&input),
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
        )
    }

    #[test]
    fn test_magnitude() {
        let res = magnitude("[5,8]");
        assert_eq!(res, 31);
    }

    #[test]
    fn test_magnitude_2() {
        let res = magnitude("[[9,1],[1,9]]");
        assert_eq!(res, 129);
    }

    #[test]
    fn test_magnitude_3() {
        let res = magnitude("[[1,2],[[3,4],5]]");
        assert_eq!(res, 143);
    }

    #[test]
    fn test_max_magnitude() {
        let input = vec_of_strings![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
        ];
        let res = part_2(&input);
        assert_eq!(res, "3993".to_string());
    }
}
