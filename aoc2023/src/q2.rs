use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::streaming::{char, multispace0, multispace1, u32},
    combinator::eof,
    multi::separated_list1,
    IResult,
};

enum Color {
    Green(u32),
    Red(u32),
    Blue(u32),
}

#[derive(Debug, Default)]
struct Trial {
    green: u32,
    red: u32,
    blue: u32,
}

#[derive(Debug, Default)]
struct Game {
    seq: u32,
    trials: Vec<Trial>,
}

pub fn output() -> Result<()> {
    let input = include_str!("input2");
    let games = input
        .lines()
        .map(parse_game)
        .collect::<Result<Vec<_>, _>>()?;
    let games = games.into_iter().map(|(_, game)| game).collect::<Vec<_>>();
    let sum_ans1 = games
        .iter()
        .filter(|game| {
            game.trials
                .iter()
                .all(|trial| trial.red <= 12 && trial.green <= 13 && trial.blue <= 14)
        })
        .fold(0, |acc, game| acc + game.seq);
    println!("Answer of q2::1st: {}", sum_ans1);
    let sum_ans2 = games.iter().fold(0, |acc, game| {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        game.trials.iter().for_each(|trial| {
            red = red.max(trial.red);
            green = green.max(trial.green);
            blue = blue.max(trial.blue);
        });
        acc + red * green * blue
    });
    println!("Answer of q2::2nd: {}", sum_ans2);
    Ok(())
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = parse_prefix(input)?;
    let (input, seq) = u32(input)?;
    let (input, _) = char(':')(input)?;
    let (input, trials) = separated_list1(tag(";"), parse_trial)(input)?;
    let (input, _) = eof(input)?;
    Ok((input, Game { seq, trials }))
}

fn parse_prefix(input: &str) -> IResult<&str, &str> {
    tag("Game ")(input)
}

fn parse_trial(input: &str) -> IResult<&str, Trial> {
    let (input, color_vec) = separated_list1(tag(","), parse_color)(input)?;
    let mut trial = Trial::default();
    for color in color_vec {
        match color {
            Color::Green(num) => {
                trial.green = num;
            }
            Color::Red(num) => {
                trial.red = num;
            }
            Color::Blue(num) => {
                trial.blue = num;
            }
        }
    }
    Ok((input, trial))
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = multispace0(input)?;
    let (input, num) = u32(input)?;
    let (input, _) = multispace1(input)?;
    let (input, color) = alt((tag("green"), tag("red"), tag("blue")))(input)?;
    match color {
        "green" => Ok((input, Color::Green(num))),
        "red" => Ok((input, Color::Red(num))),
        "blue" => Ok((input, Color::Blue(num))),
        _ => unreachable!(),
    }
}
