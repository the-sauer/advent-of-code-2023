use nom::{
    branch::alt, 
    bytes::complete::tag,
    character::complete::{digit1, space1, char, space0}, 
    combinator::map, 
    IResult,
    multi::{separated_list1, separated_list0}, 
    sequence::{tuple, separated_pair}, 
};
use core::panic;
use std::fs;

#[derive(Debug, PartialEq)]
enum CubeColor {
    Red,
    Green,
    Blue,
}
#[derive(Debug)]
struct Game {
    game_id: u32,
    hands: Vec<Vec<(u32, CubeColor)>>
}


fn main() {
    let input = fs::read_to_string("data/day02.txt").expect("Unable to open data/day02.txt");
    print!("Part1: {}\n", part1(&input));
    print!("Part1: {}\n", part2(&input));
}

fn hand_parser(input: &str) -> IResult<&str, Vec<Vec<(u32, CubeColor)>>> {
    separated_list1(
        tuple((char(';'), space0)), 
        separated_list1(
            tuple((char(','), space0)), 
            separated_pair(
                map(digit1, |s: &str| s.parse::<u32>().unwrap()), 
                space1, 
                color_parser
            )
        )
    )(input)
    
}

fn color_parser(input: &str) -> IResult<&str, CubeColor> {
    let (input, cstring) = alt((tag("red"), tag("blue"), tag("green")))(input)?;
    match cstring {
        "red" => Ok((input, CubeColor::Red)),
        "green" => Ok((input, CubeColor::Green)),
        "blue" => Ok((input, CubeColor::Blue)),
        _ => panic!("Unrecognised color {}\n", cstring)
    }
}

fn parse_line(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game")(input)?;
    let (input, _) = space1(input)?;
    let (input, game_id) = map(digit1, str::parse::<u32>)(input)?;
    let (input, _) = tuple((char(':'), space1))(input)?;
    let (input, hands) = hand_parser(input)?;
    Ok((input, Game {game_id: game_id.unwrap(), hands: hands}))
}

fn parse_input(input: &str) -> Vec<Game> {
    if let Ok((_, res)) = separated_list0(char('\n'),parse_line)(input) {
        res
    } else {
        panic!("could not parse input")
    }
}

fn part1(input: &str) -> u32 {
    fn possible (hands: Vec<Vec<(u32, CubeColor)>>) -> bool {
        for hand in hands {
            for (num, cube_color) in hand {
                if cube_color == CubeColor::Red && num > 12 || cube_color == CubeColor::Green && num > 13 || cube_color == CubeColor::Blue && num > 14 {
                    return false;
                }
            }
        }
        true
    }

    let games = parse_input(input);
    let mut sum: u32 = 0;
    for game in games {
        if possible(game.hands) {
            sum += game.game_id;
        }
    }
    sum
}

fn part2(input: &str) -> u32 {
    let games = parse_input(input);
    let mut sum: u32 = 0;
    for game in games {
        let mut min_r: u32 = 0;
        let mut min_g: u32 = 0;
        let mut min_b: u32 = 0;

        for hand in game.hands {
            for (num, cube_color) in hand {
                if cube_color == CubeColor::Red {
                    min_r = if num > min_r { num } else { min_r }
                } else if cube_color == CubeColor::Green {
                    min_g = if num > min_g { num } else { min_g }
                } else if cube_color == CubeColor::Blue {
                    min_b = if num > min_b { num } else { min_b }
                }
            }
        }

        sum += min_r * min_g * min_b;
    }
    sum
}

#[test]
fn test_part1() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(part1(&input), 8);
}

#[test]
fn test_part2() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(part2(&input), 2286);
}