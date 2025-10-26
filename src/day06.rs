use std::{fs, iter::zip};

fn main() {
    let input = fs::read_to_string("data/day06.txt").expect("Unable to open data/day06.txt");
    print!("Part1: {}\n", part1(&input));
    print!("Part2: {}\n", part2(&input));
}

fn can_beat_time(time: u64, distance: u64, held: u64) -> bool {
    (time - held) * held > distance
}

fn ways_to_win(time: u64, distance: u64) -> u64 {
    let mut lower = 0;
    let mut upper = 0;
    while !can_beat_time(time, distance, lower) {
        lower += 1;
        upper += 1;
    }
    while can_beat_time(time, distance, upper) {
        upper += 1;
    }
    upper - lower
}

fn part1(input: &str) -> u64 {
    let lines: Vec<&str> = input.split("\n").collect();
    let races = zip(
        lines[0].split_whitespace().filter_map(|e| e.parse::<u64>().ok()), 
        lines[1].split_whitespace().filter_map(|e| e.parse::<u64>().ok())
    );
    let mut result = 1;
    for (time, distance) in races {
        result *= ways_to_win(time, distance);
    }
    result
}

fn part2(input: &str) -> u64 {
    let lines: Vec<&str> = input.split("\n").collect();
    let time = lines[0].chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u64>().unwrap();
    let distance = lines[1].chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u64>().unwrap();
    ways_to_win(time, distance)
}

#[test]
fn test_part1() {
    let input = "Time:      7  15   30\nDistance:  9  40  200";
    assert_eq!(part1(&input), 288);
}

#[test]
fn test_part2() {
    let input =  "Time:      7  15   30\nDistance:  9  40  200";
    assert_eq!(part2(&input), 71503);
}