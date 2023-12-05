use crate::common::parse::parse_numbers;

pub fn run_first(input: &str) -> String {
    let seeds = parse_numbers(input.lines().next().unwrap());

    let groups = input
        .split("\n\n")
        .skip(1)
        .map(parse_resource_group)
        .collect::<Vec<_>>();

    seeds
        .iter()
        .map(|seed| get_location(*seed, &groups))
        .min()
        .unwrap()
        .to_string()
}

fn get_location(seed: i64, groups: &[ResourceGroup]) -> i64 {
    let mut value = seed;
    let mut curr_group = "seed";

    while curr_group != "location" {
        let group = groups.iter().find(|g| g.from == curr_group).unwrap();
        let new_value = group.map(value);
        let new_group = &group.to;
        curr_group = new_group;
        value = new_value;
    }

    value
}

pub fn run_second(input: &str) -> String {
    let seeds = parse_numbers(input.lines().next().unwrap());

    let groups = input
        .split("\n\n")
        .skip(1)
        .map(parse_resource_group)
        .collect::<Vec<_>>();

    seeds
        .chunks(2)
        .map(|chunk| {
            let start = chunk[0];
            let length = chunk[1];

            (start..start + length)
                .map(|seed| get_location(seed, &groups))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
        .to_string()
}

fn parse_range(line: &str) -> ResourceRange {
    let nums = parse_numbers(line);
    ResourceRange::new(nums[0], nums[1], nums[2])
}
#[derive(Debug)]
struct ResourceGroup {
    pub from: String,
    pub to: String,
    ranges: Vec<ResourceRange>,
}

fn parse_resource_group(input: &str) -> ResourceGroup {
    let mut lines = input.lines();
    let mut name_parts = lines.next().unwrap().split(' ').next().unwrap().split('-');
    let from = name_parts.next().unwrap().trim();
    let to = name_parts.nth(1).unwrap().trim();

    let ranges = lines.map(parse_range).collect();
    ResourceGroup::new(from, to, ranges)
}

impl ResourceGroup {
    pub fn new(from: &str, to: &str, ranges: Vec<ResourceRange>) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            ranges,
        }
    }

    pub fn map(&self, value: i64) -> i64 {
        for range in &self.ranges {
            if range.can_map(value) {
                return range.map(value);
            }
        }
        value
    }
}

#[derive(Debug)]
struct ResourceRange {
    dest_start: i64,
    source_start: i64,
    length: i64,
}

impl ResourceRange {
    pub fn new(dest_start: i64, source_start: i64, length: i64) -> Self {
        Self {
            source_start,
            dest_start,
            length,
        }
    }

    pub fn can_map(&self, source: i64) -> bool {
        source >= self.source_start && source < self.source_start + self.length
    }

    pub fn map(&self, source: i64) -> i64 {
        let delta = self.dest_start - self.source_start;
        source + delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_run_first() {
        // given
        let data = TEST_DATA;

        // when
        let result = run_first(data);

        // then
        assert_eq!(result, "35");
    }

    #[test]
    fn test_run_second() {
        // given
        let data = TEST_DATA;

        // when
        let result = run_second(data);

        // then
        assert_eq!(result, "46");
    }
    #[test]
    fn test_parse_group() {
        let input = "seed-to-soil map:
50 98 2
52 50 48";
        let group = parse_resource_group(input);
        println!("{:?}", group);
        assert_eq!(group.from, "seed");
        assert_eq!(group.to, "soil");
        assert_eq!(group.ranges.len(), 2);
    }

    #[test]
    fn test_map_range() {
        let range = ResourceRange::new(52, 50, 48);
        assert_eq!(range.map(79), 81);
    }

    #[test]
    fn test_can_map() {
        let range = ResourceRange::new(52, 50, 48);
        assert!(range.can_map(79));
        assert!(!range.can_map(14));
    }
}
