use aoc_runner_derive::aoc;
use lazy_regex::{ Regex, regex };

use super::{ Day01Error, firstlast };
use super::optimized::part1_specialized;

/// # Examples
/// 
/// ```
/// use aoc_2023::day01::naive::part1_generic;
/// assert_eq!(142, part1_generic("1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet").unwrap())
/// ```
#[aoc(day1, part1, Generic)]
pub fn part1_generic(input: &str) -> Result<u32, Day01Error> {
    let re = regex!(r"^[0-9]");

    solve(re, input)
}

/// # Examples
/// 
/// ```
/// use aoc_2023::day01::naive::part2_generic;
/// assert_eq!(281, part2_generic("two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen").unwrap())
/// ```
/// 
/// ```
/// use aoc_2023::day01::naive::part2_generic;
/// assert_eq!(79, part2_generic("sevenine").unwrap())
/// ```
#[aoc(day1, part2, Generic)]
pub fn part2_generic(input: &str) -> Result<u32, Day01Error> {
    let re = regex!(r"^(?:[0-9]|one|two|three|four|five|six|seven|eight|nine)");

    solve(re, input)
}

#[aoc(day1, part2, SlightlyFaster)]
pub fn part2_slightly_faster(input: &str) -> Result<u32, Day01Error> {
    part1_specialized(
        &input.replace("zero", "z0o")
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9n"))
}

fn solve(regex: &Regex, input: &str) -> Result<u32, Day01Error> {
    input.lines()
        .map(|line| solve_line(regex, line))
        .sum()
}

fn solve_line(regex: &Regex, line: &str) -> Result<u32, Day01Error> {
    let (first, last) = firstlast(
        (0..line.len())
            .map(|i| regex.find(&line[i..]))
            .filter_map(std::convert::identity)
        ).ok_or_else(|| Day01Error::NoMatchesFoundOnLine(line.to_string()))?;

    
    let first = as_numval(first.as_str())
        .ok_or_else(|| Day01Error::NumberUnrecognized(first.as_str().to_string()))?;
    let last = as_numval(last.as_str())
        .ok_or_else(|| Day01Error::NumberUnrecognized(last.as_str().to_string()))?;
    
    Ok(first * 10 + last)
}

fn as_numval(the_match: &str) -> Option<u32> {
    match the_match {
        "0" | "zero" => Some(0),
        "1" | "one" => Some(1),
        "2" | "two" => Some(2),
        "3" | "three" => Some(3),
        "4" | "four" => Some(4),
        "5" | "five" => Some(5),
        "6" | "six" => Some(6),
        "7" | "seven" => Some(7),
        "8" | "eight" => Some(8),
        "9" | "nine" => Some(9),
        _ => None,
    }
}
