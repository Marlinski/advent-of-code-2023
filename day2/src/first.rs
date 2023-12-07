use std::fs::read_to_string;
use regex::Regex;
use apply::Apply;

pub fn parse_game(line: &str) -> (u32, Vec<(u32,u32,u32)>) {
    let game_regex = Regex::new(r"Game (?P<id>\d+): (?P<reveals>.*)").unwrap();
    game_regex.captures(line).unwrap().apply(|c| {
        let id = c.name("id").unwrap().as_str().parse::<u32>().unwrap();
        let reveals = c.name("reveals").unwrap().as_str();
        let reveals: Vec<&str> = reveals.split(";").collect();
        let reveals = reveals.iter().map(|reveal| {
            let mut red: u32 = 0;
            let mut green: u32 = 0;
            let mut blue: u32 = 0;
            
            let reveal_regex = Regex::new(r"(?P<count>\d+) (?P<color>(blue|green|red))").unwrap();
            reveal_regex.captures_iter(reveal).for_each (|c| {
                let count = c.name("count").unwrap().as_str().parse::<u32>().unwrap();
                let color = c.name("color").unwrap().as_str();
                match color {
                    "red" => red += count,
                    "blue" => blue += count,
                    "green" => green += count,
                    _ => panic!("wrong color")
                }
            });
            (red,green,blue)
        }).collect();

        (id, reveals)
    })
}


fn resolve_puzzle<'a, I>(it: I) -> u32 
where
    I: Iterator<Item = &'a str>
{
    it.map(|l| parse_game(l))
    .filter(|(_, reveals)| {
        reveals
        .iter()
        .filter(|(r,g,b)| {
            *r > 12 || *g > 13 || *b > 14
        })
        .collect::<Vec<_>>()
        .len() == 0
    })
    .map(|(game_id, _)| {
        game_id
    })
    .fold(0, |acc,v| acc + v)
}


pub fn puzzle(input: &str) -> u32 {
    let it = read_to_string(input).unwrap();
    resolve_puzzle(it.lines())
}

#[test]
fn test_1() {
    let test_str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let games: Vec<&str> = test_str.split("\n").collect();
    let res = resolve_puzzle(games.into_iter());
    assert_eq!(8, res);
}