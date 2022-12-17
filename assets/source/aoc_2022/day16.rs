use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::hash::Hash;

use crate::common::parse::to_non_empty_lines;

pub fn run_first(input: &str) -> String {
    let matrix = parse_input(input);
    let paths = matrix.paths_lookup();
    matrix.max_pressure(30, &paths).to_string()
}

pub fn run_second(input: &str) -> String {
    let matrix = parse_input(input);
    let paths = matrix.paths_lookup();
    matrix.me_and_my_bro(26, &paths).to_string()
}

#[derive(Debug)]
struct ValveMatrix {
    neighbours: HashMap<String, Vec<String>>,
    flows: HashMap<String, i64>,
    valves: Vec<String>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Valve {
    name: String,
    cost: i64,
}

impl PartialOrd<Self> for Valve {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Valve {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[derive(Hash, Debug, Eq, PartialEq)]
struct Simulation {
    pub last_valve: String,
    pub open: BTreeSet<String>,
    pub time_left: i64,
}

impl ValveMatrix {
    fn max_pressure(&self, minutes: i64, paths: &HashMap<(String, String), i64>) -> i64 {
        let valves = HashSet::new();
        let nonzero = self.nonzero_valves();
        self.get_pressure(minutes, &valves, "AA", &nonzero, paths)
    }

    fn me_and_my_bro(&self, minutes: i64, paths: &HashMap<(String, String), i64>) -> i64 {
        let mut max = 0;

        let nonzero: Vec<_> = self.nonzero_valves().into_iter().collect();

        let limit = 32767;
        for i in 0..limit {
            let mut me: HashSet<String> = HashSet::new();
            let mut elebro: HashSet<String> = HashSet::new();

            for (j, item) in nonzero.iter().enumerate().take(15) {
                if (i >> j) & 1 == 1 {
                    me.insert(item.to_string());
                } else {
                    elebro.insert(item.to_string());
                }
            }

            if me.len() < 4 || elebro.len() < 4 {
                continue;
            }
            let my_res = self.get_pressure(minutes, &HashSet::new(), "AA", &me, paths);
            let elebro_res = self.get_pressure(minutes, &HashSet::new(), "AA", &elebro, paths);
            max = i64::max(max, my_res + elebro_res)
        }

        max
    }

    fn nonzero_valves(&self) -> HashSet<String> {
        self.valves
            .iter()
            .cloned()
            .filter(|v| *self.flows.get(v).unwrap() > 0)
            .collect()
    }

    fn paths_lookup(&self) -> HashMap<(String, String), i64> {
        let mut nonzero = self.nonzero_valves();
        let mut lookup = HashMap::new();
        nonzero.insert("AA".to_string());

        for from in &nonzero {
            for to in &nonzero {
                if from == to {
                    continue;
                }
                let key = (from.to_string(), to.to_string());
                if lookup.contains_key(&key) {
                    continue;
                }
                let cost = self.shortest_path(from, to);
                lookup.insert((from.to_string(), to.to_string()), cost);
                lookup.insert((to.to_string(), from.to_string()), cost);
            }
        }

        lookup
    }

    fn get_pressure(
        &self,
        minutes_left: i64,
        open: &HashSet<String>,
        curr_valve: &str,
        nonzero_valves: &HashSet<String>,
        paths: &HashMap<(String, String), i64>,
    ) -> i64 {
        if minutes_left <= 0 {
            return 0;
        }

        let rpm = self.pressure(open);
        let mut max_rp_sub = minutes_left * rpm;

        for next_valve in nonzero_valves.iter().filter(|v| !open.contains(*v)) {
            let path_dur = paths
                .get(&(curr_valve.to_string(), next_valve.to_string()))
                .unwrap()
                + 1;
            let mut this_rp = i64::min(minutes_left, path_dur) * rpm;
            let mut new_open = open.clone();
            new_open.insert(next_valve.to_string());
            if path_dur < minutes_left {
                this_rp += self.get_pressure(
                    minutes_left - path_dur,
                    &new_open,
                    next_valve,
                    nonzero_valves,
                    paths,
                );
            }
            max_rp_sub = i64::max(max_rp_sub, this_rp);
        }

        max_rp_sub
    }

    fn pressure(&self, open: &HashSet<String>) -> i64 {
        open.iter().map(|v| self.flows.get(v).unwrap()).sum()
    }

    fn shortest_path(&self, from: &str, to: &str) -> i64 {
        let mut q: VecDeque<(HashSet<String>, String, i64)> = VecDeque::new();
        q.push_back((HashSet::new(), from.to_string(), 0));

        while !q.is_empty() {
            let (traversed, next, cost) = q.pop_front().unwrap();
            if next == to {
                return cost;
            }

            for n in self
                .neighbours
                .get(&next)
                .unwrap()
                .iter()
                .filter(|n| !traversed.contains(*n))
            {
                let mut new_traversed = traversed.clone();
                new_traversed.insert(next.clone());
                q.push_back((new_traversed, n.to_string(), cost + 1))
            }
        }

        i64::MAX
    }
}

fn parse_input(input: &str) -> ValveMatrix {
    let mut neighbours: HashMap<String, Vec<String>> = HashMap::new();
    let mut flows: HashMap<String, i64> = HashMap::new();
    let mut valves: Vec<String> = vec![];

    let lines = to_non_empty_lines(input);

    for line in lines {
        let (valve_str, tunnels_str) = line.split_once(';').unwrap();
        let valve_tokens: Vec<&str> = valve_str.split(' ').filter(|t| !t.is_empty()).collect();
        let name = valve_tokens[1];

        let flow_rate = valve_tokens[4]
            .split('=')
            .nth(1)
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let tunnels: Vec<_> = tunnels_str
            .split(' ')
            .filter(|t| !t.is_empty())
            .skip(4)
            .map(|t| t.replace(',', ""))
            .collect();

        flows.insert(name.to_string(), flow_rate);
        neighbours.insert(name.to_string(), tunnels);
        valves.push(name.to_string())
    }

    ValveMatrix {
        flows,
        neighbours,
        valves,
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u8 = 16;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "2056";
        assert_eq!(result, expected.to_string());
    }
}
