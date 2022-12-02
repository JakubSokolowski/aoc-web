pub fn run_first(input: &str) -> String {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let tokens: Vec<&str> = line.split(' ').collect();
            let opponent_move = &tokens[0];
            let your_move = &tokens[1];

            get_move_value(your_move) + get_result(opponent_move, your_move)
        })
        .sum::<i64>()
        .to_string()
}

pub fn get_move_value(m: &str) -> i64 {
    match m {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("Invalid"),
    }
}

pub fn get_result(opponent_move: &str, your_move: &str) -> i64 {
    match (opponent_move, your_move) {
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("A", "Z") => 0,
        ("B", "X") => 0,
        ("B", "Y") => 3,
        ("B", "Z") => 6,
        ("C", "X") => 6,
        ("C", "Y") => 0,
        ("C", "Z") => 3,

        (_, _) => panic!("Invalid input {} {}", opponent_move, your_move),
    }
}

pub fn get_move(opponent: &str, strategy: &str) -> String {
    match (opponent, strategy) {
        ("A", "X") => "Z".to_string(),
        ("A", "Y") => "X".to_string(),
        ("A", "Z") => "Y".to_string(),
        ("B", "X") => "X".to_string(),
        ("B", "Y") => "Y".to_string(),
        ("B", "Z") => "Z".to_string(),
        ("C", "X") => "Y".to_string(),
        ("C", "Y") => "Z".to_string(),
        ("C", "Z") => "X".to_string(),

        (_, _) => panic!("Invalid input {} {}", opponent, strategy),
    }
}

pub fn get_res_value(res: &str) -> i64 {
    match res {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!("Invalid"),
    }
}

pub fn run_second(input: &str) -> String {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let tokens: Vec<&str> = line.split(' ').collect();
            let opponent_move = &tokens[0];
            let strategy = &tokens[1];
            let your_move = get_move(opponent_move, strategy);

            get_res_value(strategy) + get_move_value(&your_move)
        })
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_to_string;

    const YEAR: u32 = 2022;
    const DAY: u8 = 2;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "13682";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "12881";
        assert_eq!(result, expected.to_string());
    }
}
