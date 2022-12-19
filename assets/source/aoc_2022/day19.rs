use std::collections::HashMap;

use crate::common::parse::{parse_numbers, to_non_empty_lines};

pub fn run_first(input: &str) -> String {
    let blueprints = parse_input(input);
    blueprints
        .iter()
        .map(|&b| b.id() * find_max(24, b))
        .sum::<i64>()
        .to_string()
}

pub fn run_second(input: &str) -> String {
    let blueprints = parse_input(input);
    blueprints
        .iter()
        .take(3)
        .map(|&b| find_max(32, b))
        .product::<i64>()
        .to_string()
}

fn find_max(time: i64, blueprint: Blueprint) -> i64 {
    let mut queue: Vec<(i64, Robots, Ores)> = Vec::new();

    let initial_robots = Robots([1, 0, 0, 0]);
    let initial_ores = Ores([0, 0, 0, 0]);
    queue.push((time, initial_robots, initial_ores));

    let mut visited = HashMap::new();
    let mut num_geodes = 0;

    while let Some((time, robots, ores)) = queue.pop() {
        if time == 0 {
            num_geodes = num_geodes.max(ores.geode());
            continue;
        } else if ores.geode() + robots.geode() * time + time * time < num_geodes {
            continue;
        } else if let Some(&t) = visited.get(&(robots, ores)) {
            if time > t {
                visited.insert((robots, ores), time);
            } else {
                continue;
            }
        } else {
            visited.insert((robots, ores), time);
        }

        if blueprint.can_build(Mineral::Geode, &ores) {
            queue.push((
                time - 1,
                robots.add(Mineral::Geode),
                ores.spend(&Mineral::Geode, &robots, &blueprint),
            ));
            continue;
        }

        if blueprint.needs(Mineral::Obsidian, &robots)
            && blueprint.can_build(Mineral::Obsidian, &ores)
        {
            queue.push((
                time - 1,
                robots.add(Mineral::Obsidian),
                ores.spend(&Mineral::Obsidian, &robots, &blueprint),
            ));
        }

        if blueprint.needs(Mineral::Ore, &robots) && blueprint.can_build(Mineral::Ore, &ores) {
            queue.push((
                time - 1,
                robots.add(Mineral::Ore),
                ores.spend(&Mineral::Ore, &robots, &blueprint),
            ));
        }

        if blueprint.needs(Mineral::Clay, &robots) && blueprint.can_build(Mineral::Clay, &ores) {
            queue.push((
                time - 1,
                robots.add(Mineral::Clay),
                ores.spend(&Mineral::Clay, &robots, &blueprint),
            ));
        }

        queue.push((time - 1, robots, ores.spend_nothing(&robots)));
    }

    num_geodes
}

enum Mineral {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Blueprint([i64; 7]);

impl Blueprint {
    fn id(&self) -> i64 {
        self.0[0]
    }

    fn cost_in_ore(&self, robot: &Mineral, ore: &Mineral) -> i64 {
        match robot {
            Mineral::Ore => match ore {
                Mineral::Ore => self.ore_ore(),
                _ => 0,
            },
            Mineral::Clay => match ore {
                Mineral::Ore => self.clay_ore(),
                _ => 0,
            },
            Mineral::Obsidian => match ore {
                Mineral::Ore => self.obsidian_ore(),
                Mineral::Clay => self.obsidian_clay(),
                _ => 0,
            },
            Mineral::Geode => match ore {
                Mineral::Ore => self.geode_ore(),
                Mineral::Obsidian => self.geode_obsidian(),
                _ => 0,
            },
        }
    }

    fn ore_ore(&self) -> i64 {
        self.0[1]
    }

    fn clay_ore(&self) -> i64 {
        self.0[2]
    }

    fn obsidian_ore(&self) -> i64 {
        self.0[3]
    }

    fn obsidian_clay(&self) -> i64 {
        self.0[4]
    }

    fn geode_ore(&self) -> i64 {
        self.0[5]
    }

    fn geode_obsidian(&self) -> i64 {
        self.0[6]
    }

    fn needs(&self, robot: Mineral, robots: &Robots) -> bool {
        match robot {
            Mineral::Ore => {
                robots.ore()
                    < self
                        .ore_ore()
                        .max(self.clay_ore())
                        .max(self.obsidian_ore())
                        .max(self.geode_ore())
            }
            Mineral::Clay => robots.clay() < self.obsidian_clay(),
            Mineral::Obsidian => robots.obsidian() < self.geode_obsidian(),
            Mineral::Geode => true,
        }
    }

    fn can_build(&self, robot: Mineral, ores: &Ores) -> bool {
        match robot {
            Mineral::Ore => ores.ore() >= self.ore_ore(),
            Mineral::Clay => ores.ore() >= self.clay_ore(),
            Mineral::Obsidian => {
                ores.ore() >= self.obsidian_ore() && ores.clay() >= self.obsidian_clay()
            }
            Mineral::Geode => {
                ores.ore() >= self.geode_ore() && ores.obsidian() >= self.geode_obsidian()
            }
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Robots([i64; 4]);

impl Robots {
    fn add(&self, ore: Mineral) -> Robots {
        let mut new_ores = self.0;
        new_ores[ore as usize] += 1;
        Robots(new_ores)
    }

    fn ore(&self) -> i64 {
        self.0[0]
    }

    fn clay(&self) -> i64 {
        self.0[1]
    }

    fn obsidian(&self) -> i64 {
        self.0[2]
    }

    fn geode(&self) -> i64 {
        self.0[3]
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Ores([i64; 4]);

impl Ores {
    fn ore(&self) -> i64 {
        self.0[0]
    }

    fn clay(&self) -> i64 {
        self.0[1]
    }

    fn obsidian(&self) -> i64 {
        self.0[2]
    }

    fn geode(&self) -> i64 {
        self.0[3]
    }

    fn spend(&self, robot: &Mineral, r: &Robots, b: &Blueprint) -> Ores {
        Ores([
            self.ore() + r.ore() - b.cost_in_ore(robot, &Mineral::Ore),
            self.clay() + r.clay() - b.cost_in_ore(robot, &Mineral::Clay),
            self.obsidian() + r.obsidian() - b.cost_in_ore(robot, &Mineral::Obsidian),
            self.geode() + r.geode() - b.cost_in_ore(robot, &Mineral::Geode),
        ])
    }

    fn spend_nothing(&self, r: &Robots) -> Ores {
        Ores([
            self.ore() + r.ore(),
            self.clay() + r.clay(),
            self.obsidian() + r.obsidian(),
            self.geode() + r.geode(),
        ])
    }
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    to_non_empty_lines(input)
        .iter()
        .map(|l| Blueprint(parse_numbers(l).try_into().unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 19;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "1528";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "16926";
        assert_eq!(result, expected.to_string());
    }
}
