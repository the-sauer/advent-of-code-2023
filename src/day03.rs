use std::{fs, ops::Range};

use regex::Regex;


fn main() {
    let input = fs::read_to_string("data/day03.txt").expect("Unable to open data/day03.txt");
    print!("Part1: {}\n", part1(&input));
    print!("Part1: {}\n", part2(&input));
}

fn exists_special_char(line: &str, range: Range<usize>) -> bool {
    let start: usize = if range.start > 0 {range.start - 1 } else { 0 };
    let end: usize = if range.end < line.len() - 1 { range.end + 1 } else { line.len() - 1 };
    for i in start..end {
        if line.chars().collect::<Vec<char>>()[i] != '.' {
            return true;
        }
    }
    return false
}

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let mut sum: u32 = 0;
    let lines: Vec<&str> = input.split("\n").collect();
    for i in 0..lines.len() {
        for m in re.find_iter(lines[i]) {
            if m.start() > 0 && lines[i].chars().collect::<Vec<char>>()[m.start()-1] != '.' 
            || m.end() < lines[i].len() && lines[i].chars().collect::<Vec<char>>()[m.end()] != '.' 
            || i > 1 && exists_special_char(lines[i-1], m.range())
            || i < lines.len() - 1 && exists_special_char(lines[i+1], m.range()) {
                sum += lines[i][m.range()].parse::<u32>().unwrap();
            }
        }
    }
    sum
}

fn part2(_input: &str) -> u32 {
    0
}

#[test]
fn test_part1() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(part1(&input), 4361);
}

#[test]
fn test_part2() {
    let input = "";
    assert_eq!(part2(&input), 0);
}