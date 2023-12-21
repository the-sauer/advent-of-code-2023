use std::fs;


fn main() {
    let input = fs::read_to_string("data/day01.txt").expect("Unable to open data/day01.txt");
    print!("Part1: {}\n", part1(&input));
    print!("Part1: {}\n", part2(&input));
}

fn part1(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for  line in input.split("\n") {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        for c in line.chars() {
            if first == None && c.is_digit(10) {
                first = c.to_digit(10);
            }
            if c.is_digit(10) {
                last = c.to_digit(10);
            }
        }
        if first == None || last == None {
            panic!("Could not find two numbers in \"{}\"", line);
        }
        sum += first.unwrap() * 10 + last.unwrap();
    }
    return sum;
}

fn to_num(c: &str) -> Option<u32> {
    if c[..1].eq( "1") { 
        Some(1)
    } else if c[..1].eq("2") { 
        Some(2)
    } else if c[..1].eq("3") { 
        Some(3)
    } else if c[..1].eq("4") { 
        Some(4)
    } else if c[..1].eq("5") { 
        Some(5)
    } else if c[..1].eq("6") { 
        Some(6)
    } else if c[..1].eq("7") { 
        Some(7)
    } else if c[..1].eq("8") { 
        Some(8)
    } else if c[..1].eq("9") { 
        Some(9)
    } else if c.len() >= 3 && c[..3].eq ("one")   { 
        Some(1)
    } else if c.len() >= 3 && c[..3].eq("two"  ) { 
        Some(2)
    } else if c.len() >= 5 && c[..5].eq("three") { 
        Some(3)
    } else if c.len() >= 4 && c[..4].eq("four" ) { 
        Some(4)
    } else if c.len() >= 4 && c[..4].eq("five" ) { 
        Some(5)
    } else if c.len() >= 3 && c[..3].eq("six"  ) { 
        Some(6)
    } else if c.len() >= 5 && c[..5].eq("seven") { 
        Some(7)
    } else if c.len() >= 5 && c[..5].eq("eight") { 
        Some(8)
    } else if c.len() >= 4 && c[..4].eq("nine" ) { 
        Some(9)
    } else   { 
        None 
    }
}


fn part2(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for  line in input.split("\n") {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        for i in 0..line.len() {
            let res = to_num(&line[i..]);
            if res != None {
                if first == None {
                    first = res;
                }
                last = res;
            }
        }
        if first == None || last == None {
            panic!("Could not find two numbers in \"{}\"", line);
        } 
        sum += first.unwrap() * 10 + last.unwrap();
    }
    return sum;
}

#[test]
fn test_part1() {
    let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    assert_eq!(part1(&input), 142);
}

#[test]
fn test_part2() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    assert_eq!(part2(&input), 281);
}