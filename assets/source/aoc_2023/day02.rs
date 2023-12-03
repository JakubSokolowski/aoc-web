pub fn run_first(input: &str) -> String {
    let constraints = (12, 13, 14);

    input
        .split('\n')
        .map(parse_line)
        .filter(|(_, game_draws)| is_valid_game(constraints, game_draws))
        .map(|(game_id, _)| game_id)
        .sum::<usize>()
        .to_string()
}

pub fn run_second(input: &str) -> String {
    input
        .split('\n')
        .map(|l| {
            let (_game_id, game_draws) = parse_line(l);
            let (red, green, blue) = max_color_values(&game_draws);
            red * green * blue
        })
        .sum::<usize>()
        .to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Constraints {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Draw {
    color: Color,
    value: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GameDraws {
    draws: Vec<Draw>,
}

fn is_valid_game(constraints: (i64, i64, i64), game_draws: &[GameDraws]) -> bool {
    game_draws
        .iter()
        .all(|game_draw| is_valid_game_draw(constraints, game_draw))
}

fn max_color_values(game_draws: &Vec<GameDraws>) -> (usize, usize, usize) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for game_draw in game_draws {
        for draw in &game_draw.draws {
            match draw.color {
                Color::Red => red = red.max(draw.value),
                Color::Green => green = green.max(draw.value),
                Color::Blue => blue = blue.max(draw.value),
            }
        }
    }

    (red, green, blue)
}

fn is_valid_game_draw(constraints: (i64, i64, i64), game_draw: &GameDraws) -> bool {
    let (mut red, mut green, mut blue) = constraints;

    for draw in &game_draw.draws {
        match draw.color {
            Color::Red => red -= draw.value as i64,
            Color::Green => green -= draw.value as i64,
            Color::Blue => blue -= draw.value as i64,
        }
    }

    red >= 0 && green >= 0 && blue >= 0
}

fn parse_line(line: &str) -> (usize, Vec<GameDraws>) {
    let parts = line.split(':').collect::<Vec<_>>();
    let game_str = parts[0];
    let game_id = game_str
        .split(' ')
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let draws_str = parts[1];
    let draws = draws_str
        .split(';')
        .map(parse_game_draws)
        .collect::<Vec<_>>();

    (game_id, draws)
}

fn parse_game_draws(line: &str) -> GameDraws {
    GameDraws {
        draws: line.split(',').map(parse_draw).collect::<Vec<_>>(),
    }
}

fn parse_draw(line: &str) -> Draw {
    let parts = line.trim().split(' ').collect::<Vec<_>>();
    let value = parts[0].parse::<usize>().unwrap();
    let color = match parts[1] {
        "blue" => Color::Blue,
        "red" => Color::Red,
        "green" => Color::Green,
        c => panic!("Unknown color {}", c),
    };
    Draw { color, value }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_draw() {
        assert_eq!(
            parse_draw("3 blue"),
            Draw {
                color: Color::Blue,
                value: 3
            }
        );
    }

    #[test]
    fn test_parse_game_draws() {
        let expected = GameDraws {
            draws: vec![
                Draw {
                    color: Color::Blue,
                    value: 3,
                },
                Draw {
                    color: Color::Red,
                    value: 4,
                },
            ],
        };
        assert_eq!(parse_game_draws("3 blue, 4 red"), expected);
    }

    #[test]
    fn test_parse_line() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = (
            1,
            vec![
                GameDraws {
                    draws: vec![
                        Draw {
                            color: Color::Blue,
                            value: 3,
                        },
                        Draw {
                            color: Color::Red,
                            value: 4,
                        },
                    ],
                },
                GameDraws {
                    draws: vec![
                        Draw {
                            color: Color::Red,
                            value: 1,
                        },
                        Draw {
                            color: Color::Green,
                            value: 2,
                        },
                        Draw {
                            color: Color::Blue,
                            value: 6,
                        },
                    ],
                },
                GameDraws {
                    draws: vec![Draw {
                        color: Color::Green,
                        value: 2,
                    }],
                },
            ],
        );
        assert_eq!(parse_line(line), expected);
    }

    #[test]
    fn test_is_valid_game_draw_valid() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game_draw = parse_line(line).1;

        let constraints = (12, 13, 14);
        assert!(is_valid_game_draw(constraints, &game_draw[0]));
    }

    #[test]
    fn test_is_valid_game_draw_not_valid() {
        let line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game_draw = parse_line(line).1;

        let constraints = (12, 13, 14);
        assert!(!is_valid_game_draw(constraints, &game_draw[0]));
    }

    #[test]
    fn test_run_first() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(run_first(input), "8");
    }

    #[test]
    fn test_run_second() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(run_second(input), "2286");
    }
}
