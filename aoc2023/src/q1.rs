use std::num::ParseIntError;

use anyhow::Result;
use regex;

pub fn output() -> Result<()> {
    let re: regex::Regex = regex::Regex::new(r"[0-9]").unwrap();
    let input_str = include_str!("input1");
    let ans1: u32 = input_str
        .lines()
        .into_iter()
        .map(|s| {
            let str_vec = re.find_iter(s).collect::<Vec<_>>();
            let first = str_vec.first().unwrap().as_str().parse::<u32>()?;
            let last = str_vec.last().unwrap().as_str().parse::<u32>()?;
            std::result::Result::<u32, ParseIntError>::Ok(first * 10 + last)
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    println!("Answer of q1::1st: {}", ans1);

    let re: regex::Regex =
        regex::Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re_rev: regex::Regex =
        regex::Regex::new(r"[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    let ans2: u32 = input_str
        .lines()
        .into_iter()
        .map(|s| {
            let first = parse_num(re.find(s).unwrap().as_str())?;
            let last = parse_num(
                re_rev
                    .find(s.chars().rev().collect::<String>().as_str())
                    .unwrap()
                    .as_str()
                    .chars()
                    .rev()
                    .collect::<String>()
                    .as_str(),
            )?;
            anyhow::Result::<u32>::Ok(first * 10 + last)
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    println!("Answer of q1::2st: {}", ans2);
    Ok(())
}

fn parse_num(s: &str) -> Result<u32> {
    match s {
        "one" => Ok(1),
        "two" => Ok(2),
        "three" => Ok(3),
        "four" => Ok(4),
        "five" => Ok(5),
        "six" => Ok(6),
        "seven" => Ok(7),
        "eight" => Ok(8),
        "nine" => Ok(9),
        _ => Ok(s.parse::<u32>()?),
    }
}
