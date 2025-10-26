use std::{fs, ops::Range};

use itertools::Itertools;
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

fn parse_number_at(lines: &Vec<Vec<char>>, index: &(usize, usize)) -> u32 {
    let mut j_end = index.1 + 1;
    while j_end < lines[index.0].len() && lines[index.0][j_end].is_ascii_digit() {
        j_end += 1;
    }
    let number_str: String = (lines[index.0][index.1..j_end]).into_iter().collect();
    number_str.parse::<u32>().expect(&format!("Failed to parse \"{}\" as number", number_str))
}

fn find_number_begin(lines: &Vec<Vec<char>>, index: &(usize, usize)) -> Option<(usize, usize)> {
    if !lines[index.0][index.1].is_ascii_digit() {
        return Option::None;
    }
    let mut j_begin = index.1;
    while j_begin > 0 && lines[index.0][j_begin-1].is_ascii_digit() {
        j_begin -= 1;
    }
    return Option::Some((index.0, j_begin));
}

fn part2(input: &str) -> u32 {
    let lines: Vec<Vec<char>> = input.split("\n").map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] == '*' {
                let window = [(i-1, j-1), (i-1, j), (i-1, j+1), (i, j-1), (i, j+1), (i+1, j-1), (i+1, j), (i+1, j+1)];
                let indices: Vec<(usize, usize)> = window
                    .iter()
                    .filter(|(a,b)| a >= &(0 as usize) && a < &lines.len() && b >= &(0 as usize) && b < &lines[*a].len())
                    .filter_map(|index| find_number_begin(&lines, index))
                    .unique()
                    .collect();
                if indices.len() == 2 {
                    sum += parse_number_at(&lines, &indices[0]) * parse_number_at(&lines, &indices[1]);
                }
            }
        }
    }
    sum
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
    assert_eq!(part2(&input), 467835);
}