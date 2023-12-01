use aoc_runner_derive::aoc;
use lazy_regex::{ Regex, regex };
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day01Error {
    #[error("Could not convert {0} to number.")]
    NumberUnrecognized(String),
    #[error("Could not find numbers on line {0}")]
    NoMatchesFoundOnLine(String),
}

/// # Examples
/// 
/// ```
/// use aoc_2023::day01::part1_generic;
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
/// use aoc_2023::day01::part1_specialized;
/// assert_eq!(142, part1_specialized("1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet").unwrap())
/// ```
#[aoc(day1, part1, Specialized)]
pub fn part1_specialized(input: &str) -> Result<u32, Day01Error> {
    input.lines()
        .map(|l| -> Result<u32, Day01Error> {
            let x = firstlast(l.chars().filter(char::is_ascii_digit))
                .ok_or_else(|| Day01Error::NoMatchesFoundOnLine(l.to_string()))?;

            let first = x.0.to_digit(10).ok_or_else(|| Day01Error::NumberUnrecognized(x.0.to_string()))?;
            let last = x.1.to_digit(10).ok_or_else(|| Day01Error::NumberUnrecognized(x.0.to_string()))?;

            Ok(first * 10 + last)
        }).sum()
}

/// # Examples
/// 
/// ```
/// use aoc_2023::day01::part2_generic;
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
/// use aoc_2023::day01::part2_generic;
/// assert_eq!(79, part2_generic("sevenine").unwrap())
/// ```
#[aoc(day1, part2, Generic)]
pub fn part2_generic(input: &str) -> Result<u32, Day01Error> {
    let re = regex!(r"^(?:[0-9]|one|two|three|four|five|six|seven|eight|nine)");

    solve(re, input)
}

/// # Examples
/// 
/// ```
/// use aoc_2023::day01::part2_statem;
/// assert_eq!(281, part2_statem("two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen").unwrap())
/// ```
/// 
/// ```
/// use aoc_2023::day01::part2_statem;
/// assert_eq!(79, part2_statem("sevenine").unwrap())
/// ```
#[aoc(day1, part2, Statemachine)]
pub fn part2_statem(input: &str) -> Result<u32, Day01Error> {
    input.lines().map(|line| -> Result<u32, Day01Error> {
        let mut forward_state = ForwardAutomaton::Empty;
        let mut forward_iter = line.chars().into_iter();

        while ! matches!(forward_state, ForwardAutomaton::Done(_)) {
            if let Some(chr) = forward_iter.next() {
                forward_state = forward_state.take(chr);
            } else {
                return Err(Day01Error::NoMatchesFoundOnLine(line.to_string()));
            }
        }

        let mut reverse_state = ReverseAutomaton::Empty;
        let mut reverse_iter = line.chars().rev();

        while ! matches!(reverse_state, ReverseAutomaton::Done(_)) {
            if let Some(chr) = reverse_iter.next() {
                reverse_state = reverse_state.take(chr);
            } else {
                return Err(Day01Error::NoMatchesFoundOnLine(line.to_string()));
            }
        }

        Ok(forward_state.unwrap() + reverse_state.unwrap())
    }).sum()
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

fn firstlast<T: Clone>(mut it: impl Iterator<Item = T>) -> Option<(T, T)> {
    if let Some(first) = it.next() {
        let last = it.last().unwrap_or(first.clone());

        Some((first, last))
    } else {
        None
    }
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ForwardAutomaton {
    Empty,
    O,
    T,
    F,
    S,
    E,
    N,
    ON,
    TW,
    TH,
    FO,
    FI,
    SI,
    SE,
    EI,
    NI,
    THR,
    FOU,
    FIV,
    SEV,
    EIG,
    NIN,
    THRE,
    SEVE,
    EIGH,
    Done(u32),
}

impl ForwardAutomaton {
    fn unwrap(self) -> u32 {
        if let Self::Done(num) = self {
            num
        } else {
            panic!("Unwrapped automaton state that was not done {:?}", self)
        }
    }

    fn take(self, chr: char) -> Self {
        match self {
            ForwardAutomaton::Empty => match chr {
                'o' => Self::O,
                't' => Self::T,
                'f' => Self::F,
                's' => Self::S,
                'e' => Self::E,
                'n' => Self::N,
                '0' => Self::Done(0),
                '1' => Self::Done(10),
                '2' => Self::Done(20),
                '3' => Self::Done(30),
                '4' => Self::Done(40),
                '5' => Self::Done(50),
                '6' => Self::Done(60),
                '7' => Self::Done(70),
                '8' => Self::Done(80),
                '9' => Self::Done(90),
                _ => Self::Empty,
            },
            ForwardAutomaton::O => match chr {
                'n' => Self::ON,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::T => match chr {
                'w' => Self::TW,
                'h' => Self::TH,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::F => match chr {
                'o' => Self::FO,
                'i' => Self::FI,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::S => match chr {
                'i' => Self::SI,
                'e' => Self::SE,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::E => match chr {
                'i' => Self::EI,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::N => match chr {
                'i' => Self::NI,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::ON => match chr {
                'e' => Self::Done(10),
                _ => Self::N.take(chr),
            },
            ForwardAutomaton::TW => match chr {
                'o' => Self::Done(20),
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::TH => match chr {
                'r' => Self::THR,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::FO => match chr {
                'u' => Self::FOU,
                _ => Self::O.take(chr),
            },
            ForwardAutomaton::FI => match chr {
                'v' => Self::FIV,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::SI => match chr {
                'x' => Self::Done(60),
                _ => Self::Empty.take(chr),

            },
            ForwardAutomaton::SE => match chr {
                'v' => Self::SEV,
                _ => Self::E.take(chr),
            },
            ForwardAutomaton::EI => match chr {
                'g' => Self::EIG,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::NI => match chr {
                'n' => Self::NIN,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::THR => match chr {
                'e' => Self::THRE,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::FOU => match chr {
                'r' => Self::Done(40),
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::FIV => match chr {
                'e' => Self::Done(50),
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::SEV => match chr {
                'e' => Self::SEVE,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::EIG => match chr {
                'h' => Self::EIGH,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::NIN => match chr {
                'e' => Self::Done(90),
                _ => Self::N.take(chr),
            },
            ForwardAutomaton::THRE => match chr {
                'e' => Self::Done(30),
                _ => Self::E.take(chr),
            },
            ForwardAutomaton::SEVE => match chr {
                'n' => Self::Done(70),
                _ => Self::E.take(chr),
            },
            ForwardAutomaton::EIGH => match chr {
                't' => Self::Done(80),
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::Done(_) => self,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ReverseAutomaton {
    Empty,
    E,
    O,
    R,
    X,
    N,
    T,
    NE,
    WO,
    EE,
    UR,
    VE,
    IX,
    EN,
    HT,
    REE,
    OUR,
    IVE,
    VEN,
    GHT,
    INE,
    HREE,
    EVEN,
    IGHT,
    Done(u32),
}

impl ReverseAutomaton {
    fn unwrap(self) -> u32 {
        if let Self::Done(num) = self {
            num
        } else {
            panic!("Unwrapped automaton state that was not done {:?}", self)
        }
    }

    fn take(self, chr: char) -> Self {
        match self {
            ReverseAutomaton::Empty => match chr {
                'e' => Self::E,
                'o' => Self::O,
                'r' => Self::R,
                'x' => Self::X,
                'n' => Self::N,
                't' => Self::T,
                '0' => Self::Done(0),
                '1' => Self::Done(1),
                '2' => Self::Done(2),
                '3' => Self::Done(3),
                '4' => Self::Done(4),
                '5' => Self::Done(5),
                '6' => Self::Done(6),
                '7' => Self::Done(7),
                '8' => Self::Done(8),
                '9' => Self::Done(9),
                _ => Self::Empty,
            },
            ReverseAutomaton::E => match chr {
                'n' => Self::NE,
                'e' => Self::EE,
                'v' => Self::VE,
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::O => match chr {
                'w' => Self::WO,
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::R => match chr {
                'u' => Self::UR,
                _ => Self::Empty.take(chr)
            }
            ReverseAutomaton::X => match chr {
                'i' => Self::IX,
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::N => match chr {
                'e' => Self::EN,
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::T => match chr {
                'h' => Self::HT,
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::NE => match chr {
                'o' => Self::Done(1),
                'i' => Self::INE,
                _ => Self::N.take(chr),
            },
            ReverseAutomaton::WO => match chr {
                't' => Self::Done(2),
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::EE => match chr {
                'r' => Self::REE,
                _ => Self::E.take(chr),
            },
            ReverseAutomaton::UR => match chr {
                'o' => Self::OUR,
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::VE => match chr {
                'i' => Self::IVE,
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::IX => match chr {
                's' => Self::Done(6),
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::EN => match chr {
                'v' => Self::VEN,
                _ => Self::E.take(chr),
            },
            ReverseAutomaton::HT => match chr {
                'g' => Self::GHT,
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::REE => match chr {
                'h' => Self::HREE,
                _ => Self::R.take(chr),
            },
            ReverseAutomaton::OUR => match chr {
                'f' => Self::Done(4),
                _ => Self::O.take(chr),
            },
            ReverseAutomaton::IVE => match chr {
                'f' => Self::Done(5),
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::VEN => match chr {
                'e' => Self::EVEN,
                _ => Self::VE.take(chr),
            },
            ReverseAutomaton::GHT => match chr {
                'i' => Self::IGHT,
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::INE => match chr {
                'n' => Self::Done(9),
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::HREE => match chr {
                't' => Self::Done(3),
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::EVEN => match chr {
                's' => Self::Done(7),
                _ => Self::E.take(chr),
            },
            ReverseAutomaton::IGHT => match chr {
                'e' => Self::Done(8),
                _ => Self::E.take(chr),
            },
            ReverseAutomaton::Done(_) => self,
        }
    }
}
