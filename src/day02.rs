use std::collections::HashMap;

use aoc_runner_derive::{ aoc, aoc_generator };
use nom::{ bytes::complete::tag, IResult, combinator::{recognize, map_res}, character::complete::digit1, multi::separated_list0, Parser, sequence::pair, branch::alt, Err, error::Error };
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Game {
    id: u32,
    turns: Vec<Turn>,
}

#[derive(Debug, Clone, Copy)]
pub struct Turn {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, Error)]
pub enum Day02Error {
    #[error("Failed to parse due to remainder: {0}")]
    Rest(String),
    #[error("Failed to parse due to nom error: {0}")]
    ParseError(Err<Error<String>>),
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Result<Vec<Game>, Day02Error> {
    match separated_list0(tag("\n"), game)(input) {
        Ok(("", result)) => Ok(result),
        Ok((rest, _)) => Err(Day02Error::Rest(rest.to_owned())),
        Err(error) => Err(Day02Error::ParseError(error.to_owned())),
    }
}

/// # Examples
/// 
/// ```
/// use aoc_2023::day02::{ parse, part1 };
/// assert_eq!(8, part1(&parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap()))
/// ```
#[aoc(day2, part1)]
pub fn part1(input: &[Game]) -> u32 {
    input.iter().filter(|game| {
        !game.turns.iter().any(|turn| {
            turn.red > 12 || turn.green > 13 || turn.blue > 14
        })
    }).map(|game| game.id).sum()
}

/// # Examples
/// 
/// ```
/// use aoc_2023::day02::{ parse, part2 };
/// assert_eq!(2286, part2(&parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap()))
/// ```
#[aoc(day2, part2)]
pub fn part2(input: &[Game]) -> u32 {
    input.iter().map(|game| {
        let (r, g, b) = game.turns.iter().fold(Default::default(), |(red, green, blue), turn| {
            (std::cmp::max(red, turn.red), std::cmp::max(green, turn.green), std::cmp::max(blue, turn.blue))
        });

        r * g * b
    }).sum()
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = map_res(recognize(digit1), str::parse::<u32>)(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, turns) = separated_list0(tag("; "), turn()).parse(input)?;

    Ok((input, Game {
        id,
        turns,
    }))
}

fn turn<'a>() -> impl Parser<&'a str, Turn, nom::error::Error<&'a str>> {
    separated_list0(tag(", "), pair(
        map_res(recognize(digit1), str::parse::<u32>),
        alt((tag(" red"), tag(" green"), tag(" blue"))),
    )).map(|tuples| {
        let the_colors: HashMap<&str, u32> = tuples.into_iter().map(|(a, b)| (b, a)).collect();
        Turn {
            red: *(the_colors.get(" red").unwrap_or(&0)),
            green: *(the_colors.get(" green").unwrap_or(&0)),
            blue: *(the_colors.get(" blue").unwrap_or(&0)),
        }
    })
}
