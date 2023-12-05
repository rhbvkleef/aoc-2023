use std::{collections::HashSet, hash::BuildHasher};

use aoc_runner_derive::{aoc_generator, aoc};
use identity_hash::BuildIdentityHasher;
use nom::{bytes::complete::tag, character::complete::{char, u32, space1, line_ending}, multi::{separated_list0, fold_many1}, Err, error::Error, sequence::{preceded, separated_pair, tuple}, Parser, IResult};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Card<H> {
    winning: HashSet<u32, H>,
    drawn: HashSet<u32, H>,
}

#[derive(Debug, Error)]
pub enum Day04Error {
    #[error("Failed to parse due to remainder: {0}")]
    Rest(String),
    #[error("Failed to parse due to nom error: {0}")]
    ParseError(Err<Error<String>>),
}

/// # Examples
/// 
/// ```
/// use aoc_2023::day04::parse;
/// assert_eq!(6, parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
/// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
/// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
/// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
/// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
/// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").unwrap().len());
/// ```
#[aoc_generator(day4)]
pub fn parse(input: &str) -> Result<Vec<Card<BuildIdentityHasher<u32>>>, Day04Error> {
    match separated_list0(line_ending, Card::parser)(input) {
        Ok(("", cards)) => Ok(cards),
        Ok((rest, _)) => Err(Day04Error::Rest(rest.to_owned())),
        Err(err) => Err(Day04Error::ParseError(err.to_owned())),
    }
}

impl<T: BuildHasher> Card<T> {
    fn num_wins(&self) -> usize {
        self.winning.intersection(&self.drawn).count()
    }
}

impl<H: Default + BuildHasher> Card<H> {
    fn parser(input: &str) -> IResult<&str, Self> {
        preceded(
            tuple((
                tag("Card"),
                space1,
                u32,
                char(':'))),
        
            separated_pair(
                fold_many1(
                    preceded(space1, u32),
                    || HashSet::with_hasher(Default::default()),
                    |mut acc, item|  {acc.insert(item); acc}),
                tag(" |"),
                fold_many1(
                    preceded(space1, u32),
                    || HashSet::with_hasher(Default::default()),
                    |mut acc, item|  {acc.insert(item); acc})))
                .map(|(winning, drawn)| Card{winning, drawn})
                .parse(input)
    }
}

/// # Examples
/// 
/// ```
/// use aoc_2023::day04::{ parse, part1 };
/// 
/// assert_eq!(13, part1(&parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
/// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
/// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
/// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
/// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
/// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").unwrap()))
/// ```
#[aoc(day4, part1)]
pub fn part1<H: BuildHasher>(cards: &[Card<H>]) -> u32 {
    cards.iter().map(|card| match card.num_wins() {
        0 => 0,
        num_wins => 2u32.pow(num_wins as u32 - 1),
    }).sum()
}

/// # Examples
/// 
/// ```
/// use aoc_2023::day04::{ parse, part2 };
/// 
/// assert_eq!(30, part2(&parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
/// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
/// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
/// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
/// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
/// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").unwrap()))
/// ```
#[aoc(day4, part2)]
pub fn part2<H: BuildHasher>(cards: &[Card<H>]) -> u32 {
    let mut scratch_card_counts = vec![1; cards.len()];

    cards.iter().enumerate().fold(0, |acc, (idx, card)| {
        let num_wins = card.num_wins();

        for idx_to_incr in (idx + 1)..std::cmp::min(idx + 1 + num_wins, scratch_card_counts.len()) {
            scratch_card_counts[idx_to_incr] += scratch_card_counts[idx];
        }

        acc + scratch_card_counts[idx]
    })
}
