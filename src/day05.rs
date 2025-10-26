use std::fs;

fn main() {
    let (seeds, garden_maps) = parse_input(fs::read_to_string("data/day05.txt").expect("Unable to open data/day05.txt"));

    print!("Part1: {}\n", part1(&seeds, &garden_maps));
    print!("Part2: {}\n", part2(&seeds, &garden_maps));
}

struct GardenMapEntry {
    source: u64,
    dest: u64,
    range: u64
}

impl GardenMapEntry {
    fn new(dest: u64, source: u64, range: u64) -> Self {
        Self { source: source, dest: dest, range: range }
    }
}
struct GardenMap {
    entries: Vec<GardenMapEntry>
}

impl GardenMap {
    fn new(lines: Vec<&str>) -> Self {
        Self {
            entries: lines[1..]
                .iter()
                .map(|line| line.split(" ").collect::<Vec<&str>>())
                .map(|items| GardenMapEntry::new(
                    items[0].parse::<u64>().unwrap(),
                    items[1].parse::<u64>().unwrap(),
                    items[2].parse::<u64>().unwrap()
                ))
                .collect()
        }
    }

    fn lookup(&self, seed: u64) -> u64 {
        for entry in &self.entries {
            if seed >= entry.source && seed < entry.source + entry.range {
                return entry.dest + seed - entry.source;
            }
        }
        seed
    }
}

fn parse_input(input: String) -> (Vec<u64>, Vec<GardenMap> ){
    let binding: Vec<&str> = input
        .split("\n")
        .collect();
    let seeds = binding[0]
        .split(" ")
        .filter_map(|e| e.parse::<u64>().ok())
        .collect();
    let garden_map = binding[1..]
        .split(|line| line.is_empty())
        .filter(|group| !group.is_empty())
        .map(|e| e.to_vec())
        .map(GardenMap::new)
        .collect();
    (seeds, garden_map)
}

fn part1(seeds: &Vec<u64>, garden_maps: &Vec<GardenMap>) -> u64 {
    let mut min_result = std::u64::MAX;
    for seed in seeds {
        let mut result = *seed;
        for garden_map in garden_maps {
            result = garden_map.lookup(result)

        }
        if result < min_result {
            min_result = result;
        } 
    }
    min_result
}

fn part2(seeds: &Vec<u64>, garden_maps: &Vec<GardenMap>) -> u64 {
    let mut min_result = std::u64::MAX;
    let mut i = 0;
    while i < seeds.len() {
        for seed in seeds[i]..seeds[i]+seeds[i+1] {
            let mut result = seed;
            for garden_map in garden_maps {
                result = garden_map.lookup(result)

            }
            if result < min_result {
                min_result = result;
            } 
        }
        i += 2;
    }
    min_result
}

#[test]
fn test_part1() {
    let input = String::from("seeds: 79 14 55 13

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
56 93 4");
    let (seeds, garden_maps) = parse_input(input);
    assert_eq!(part1(&seeds, &garden_maps), 35);
}

#[test]
fn test_part2() {
    let input = String::from("seeds: 79 14 55 13

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
56 93 4");
    let (seeds, garden_maps) = parse_input(input);
    assert_eq!(part2(&seeds, &garden_maps), 46);
}
