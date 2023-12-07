/*
--- Part Two ---
Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

What is the sum of all of the calibration values?
*/

use std::fs::read_to_string;
use regex::Regex;
use regex::RegexSet;

pub fn puzzle(input: &str) -> u32 {
    read_to_string(input).unwrap().lines()
    .map(|l| extract_first_and_last_digit(l))
    .fold::<u32, _>(0, |acc, (f,l)| {
        acc + format!("{}{}", f, l).parse::<u32>().unwrap()
    })
}

fn str_to_digit(digit_str: &str) -> char {
    match digit_str {
        "zero" | "0" => '0',
        "one" | "1" => '1',
        "two" | "2" => '2',
        "three" | "3" => '3',
        "four" | "4" => '4',
        "five" | "5" => '5',
        "six" | "6" => '6',
        "seven" | "7" => '7',
        "eight" | "8" =>'8',
        "nine" |"9" => '9',
        _ => panic!("not a digit")
    }
}

fn extract_first_and_last_digit(line: &str) -> (char,char) {
    let patterns = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let set = RegexSet::new(patterns).unwrap();

    let mut matches: Vec<(usize,String)> = set.matches(line)
    .into_iter()
    .flat_map(|index| {
        let pattern = &patterns[index];
        let regex = Regex::new(pattern).unwrap();
        let ret: Vec<(usize,String)> = regex.captures_iter(line)
        .map(|capture| {
            let start = capture.get(0).unwrap().start().to_owned();
            let matched_text = &capture[0];
            (start, matched_text.to_owned())
        })
        .collect();
        ret
    })
    .collect();

    matches.sort_by(|(a1,_),(a2,_)| a1.cmp(a2) );

    let ordered: Vec<char> = matches.into_iter()
    .map(|(_,s)| s)
    .map(|s| str_to_digit(s.as_str()))
    .collect();

    (ordered.first().unwrap().clone(), ordered.last().unwrap().clone())
}


#[test]
fn test_1() {
    let strings = [
        (('2','9'),"two1nine"),
        (('8','3'), "eightwothree"),
        (('1','3'), "abcone2threexyz"),
        (('2','4'), "xtwone3four"),
        (('4','2'), "4nineeightseven2"),
        (('1','4'), "zoneight234"),
        (('7','6'), "7pqrstsixteen"),
    ];

    for (expected,str) in strings {
        assert_eq!(expected, extract_first_and_last_digit(str), "{}", str);
    }

    let res = strings
    .into_iter()
    .map(|(a,_)| a)
    .fold::<u32, _>(0, |acc, (f,l)| {
        acc + format!("{}{}", f, l).parse::<u32>().unwrap()
    });

    assert_eq!(281, res, "res is not correct");
}