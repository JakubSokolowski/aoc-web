use crate::common::parse::to_non_empty_lines;
use std::collections::HashMap;

pub fn run_first(input: &str) -> String {
    let lines = to_non_empty_lines(input);
    count_paths(&lines).to_string()
}

pub fn run_second(input: &str) -> String {
    let lines = to_non_empty_lines(input);
    count_paths_2(&lines).to_string()
}

#[derive(Debug)]
pub struct Distance {
    from: String,
    to: String,
}

#[derive(Debug)]
pub struct CaveMatrix {
    neighbours: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct CavePath {
    caves: Vec<String>,
    small_twice_visited: bool,
    visits: HashMap<String, usize>,
}

impl CavePath {
    fn new() -> CavePath {
        CavePath {
            small_twice_visited: false,
            caves: vec![],
            visits: HashMap::new(),
        }
    }

    fn contains(&self, cave: &str) -> bool {
        self.caves.contains(&cave.to_string())
    }

    fn push(&mut self, cave: &str) {
        self.caves.push(cave.to_string());
        self.visits.insert(
            cave.to_string(),
            1 + if self.visits.contains_key(cave) {
                self.visits[cave]
            } else {
                0
            },
        );
        if !self.small_twice_visited && is_small_cave(cave) {
            let count = self.count_visits(cave);
            if count == 2 {
                self.small_twice_visited = true;
            }
        }
    }

    fn count_visits(&self, cave: &str) -> usize {
        *self.visits.get(cave).unwrap_or(&0)
    }

    fn can_push(&self, cave: &str) -> bool {
        if !is_small_cave(cave) {
            return true;
        }

        let cave_visits = self.count_visits(cave);
        match cave_visits {
            0 => true,
            1 => {
                if cave == "start" || cave == "end" {
                    return false;
                }

                !self.small_twice_visited
            }
            _ => false,
        }
    }
}

impl CaveMatrix {
    fn new(distances: Vec<Distance>) -> CaveMatrix {
        let mut neighbours: HashMap<String, Vec<String>> = HashMap::new();

        for distance in distances {
            let from = distance.from.clone();
            let to = distance.to.clone();
            if neighbours.contains_key(&from) {
                let cave = neighbours.get_mut(&from).unwrap();
                cave.push(to.clone());
            } else {
                neighbours.insert(from.clone(), vec![to.clone()]);
            }
            if neighbours.contains_key(&to) {
                let cave = neighbours.get_mut(&to).unwrap();
                cave.push(from.clone());
            } else {
                neighbours.insert(distance.to.clone(), vec![distance.from.clone()]);
            }
        }

        CaveMatrix { neighbours }
    }

    fn count_paths(&self) -> usize {
        let mut curr_path = CavePath::new();
        let mut all_paths: Vec<CavePath> = vec![];
        let start = "start";
        self.dfs(start, &mut curr_path, &mut all_paths);
        all_paths.len()
    }

    fn count_paths_2(&self) -> usize {
        let mut curr_path = CavePath::new();
        let mut all_paths: Vec<CavePath> = vec![];
        let start = "start";
        self.dfs_2(start, &mut curr_path, &mut all_paths);
        all_paths.len()
    }

    fn dfs(&self, current: &str, path: &mut CavePath, all_paths: &mut Vec<CavePath>) {
        path.push(current);

        if current == "end" {
            all_paths.push(path.clone());
        }

        for cave in self.neighbours.get(current).unwrap().iter() {
            if !(is_small_cave(cave) && path.contains(cave)) {
                self.dfs(cave, &mut path.clone(), all_paths)
            }
        }
    }

    fn dfs_2(&self, current: &str, path: &mut CavePath, all_paths: &mut Vec<CavePath>) {
        path.push(current);

        if current == "end" {
            all_paths.push(path.clone());
        }

        for cave in self.neighbours.get(current).unwrap().iter() {
            if path.can_push(cave) {
                self.dfs_2(cave, &mut path.clone(), all_paths)
            }
        }
    }
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().any(|c| c.is_lowercase())
}

fn parse_input(input: &[String]) -> Vec<Distance> {
    input
        .iter()
        .map(|line| {
            let tokens: Vec<&str> = line.split('-').collect();
            let from = tokens[0].to_string();
            let to = tokens[1].to_string();
            Distance { from, to }
        })
        .collect()
}

fn parse_cave(input: &[String]) -> CaveMatrix {
    CaveMatrix::new(parse_input(input))
}

fn count_paths(input: &[String]) -> usize {
    let cave = parse_cave(input);
    cave.count_paths()
}

fn count_paths_2(input: &[String]) -> usize {
    let cave = parse_cave(input);
    cave.count_paths_2()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse::test_utils::vec_of_strings;
    use crate::utils::read_to_string;

    const YEAR: u32 = 2021;
    const DAY: u8 = 12;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "5212";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "134862";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_count_paths_1() {
        let input = vec_of_strings!["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];
        let cave = parse_cave(&input);
        assert_eq!(cave.count_paths(), 10)
    }

    #[test]
    fn test_count_paths_2() {
        let input = vec_of_strings![
            "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa",
            "kj-HN", "kj-dc"
        ];
        let cave = parse_cave(&input);
        assert_eq!(cave.count_paths(), 19)
    }

    #[test]
    fn test_count_paths_2_1() {
        let input = vec_of_strings!["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];
        let cave = parse_cave(&input);
        assert_eq!(cave.count_paths_2(), 36)
    }
}
