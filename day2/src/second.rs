/*
 */
use std::fs::read_to_string;
use crate::first;


fn resolve_puzzle<'a, I>(it: I) -> u32 
where
    I: Iterator<Item = &'a str>
{
    it.map(|l| first::parse_game(l))
    .map(|(game_id, reveals)| {
        let mut max_red: u32 = 0;
        let mut max_green: u32 = 0;
        let mut max_blue: u32 = 0;

        reveals
        .iter()
        .for_each(|(r,g,b)| {
            if *r > max_red {
                max_red = *r;
            }
            if *g > max_green {
                max_green = *g;
            }
            if *b > max_blue {
                max_blue = *b
            }
        });

        let power = max_red * max_green * max_blue;
        (game_id, power)        
    })
    .map(|(_, power)| {
        power
    })
    .fold(0, |acc,v| acc + v)
}

pub fn puzzle(input: &str) -> u32 {
    let it = read_to_string(input).unwrap();
    resolve_puzzle(it.lines())
}


#[test]
fn test_2() {
    let test_str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let games: Vec<&str> = test_str.split("\n").collect();
    let res = resolve_puzzle(games.into_iter());
    assert_eq!(2286, res);
}