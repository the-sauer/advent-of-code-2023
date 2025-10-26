use std::fs;
use nom::{bytes::complete::tag, character::complete::{space1, digit1, char, space0}, combinator::map, multi::separated_list0, IResult, sequence::tuple};


fn main() {
    let input = fs::read_to_string("data/day04.txt").expect("Unable to open data/day04.txt");
    print!("Part1: {}\n", part1(&input));
    print!("Part2: {}\n", part2(&input));
}

fn parse_line(input: &str) -> IResult<&str, (u32, Vec<u32>, Vec<u32>)> {
    let (input, _) = tuple((tag("Card"), space1))(input)?;
    let (input, card_id) = map(digit1, |s: &str| s.parse::<u32>().unwrap())(input)?;
    let (input, _) = tuple((char(':'), space0))(input)?;
    let (input, winning_numbers) = separated_list0(space1, map(digit1,  |s: &str| s.parse::<u32>().unwrap()))(input)?;
    let (input, _) = tuple((space0, char('|'), space0))(input)?;
    let (input, numbers) = separated_list0(space1, map(digit1,  |s: &str| s.parse::<u32>().unwrap()))(input)?;
    let (input, _) = space0(input)?;
    Ok((input, (card_id, winning_numbers, numbers)))
}

fn parse_puzzle_input(input: &str) -> Vec<(u32, Vec<u32>, Vec<u32>)> {
    if let Ok((_, res)) = separated_list0(char('\n'), parse_line)(input) {
        res
    } else {
        panic!("Could not parse puzzle input")
    }
}

fn num_matches(card: &(u32, Vec<u32>, Vec<u32>)) -> u32 {
    let mut winning_matches = 0;
    for num in &card.2 {
        if  card.1.contains(&num) {
            winning_matches += 1;
        }
    }
    winning_matches
}

fn part1(input: &str) -> u32 {
    let cards = parse_puzzle_input(input);
    let mut sum: u32 = 0;
    for card in cards {
        let winning_matches = num_matches(&card);
        if winning_matches > 0 {
            sum += 2_u32.pow(winning_matches-1);
        }
    }
    sum
}

fn part2(input: &str) -> u32 {
    let res = parse_puzzle_input(input);
    let mut all_cards: Vec<u32> = vec![1; res.len()];
    // let mut sum: u32 = 0;
    for card in &res {
        let winning_matches = num_matches(&card);
        for i in 1..winning_matches+1 {
            all_cards[(card.0 + i) as usize] += all_cards[card.0 as usize];
        }
    }

    // for card_id in 0..res.len() {
    //     let winning_matches = num_matches(&res[card_id as usize]);
    //     if winning_matches > 0 {
    //         sum += 2_u32.pow(winning_matches-1) * all_cards[card_id as usize];
    //     }
    // }
    all_cards.iter().fold(0, |acc, num| acc + num)
}

#[test]
fn test_part1() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(part1(&input), 13);
}

#[test]
fn test_part2() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(part2(&input), 30);
}