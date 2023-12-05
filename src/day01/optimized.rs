use super::{ Day01Error, firstlast };

use aoc_runner_derive::aoc;
use bstr::{BString, ByteSlice};

/// # Examples
/// 
/// ```
/// use aoc_2023::day01::optimized::part1_specialized;
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
/// use aoc_2023::day01::optimized::part2_statem;
/// assert_eq!(281, part2_statem(b"two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen").unwrap())
/// ```
/// 
/// ```
/// use aoc_2023::day01::optimized::part2_statem;
/// assert_eq!(79, part2_statem(b"sevenine").unwrap())
/// ```
#[aoc(day1, part2, Statemachine)]
pub fn part2_statem(input: &[u8]) -> Result<u32, Day01Error> {
    BString::from(input).lines().map(|line| {
        let mut forward_state = ForwardAutomaton::Empty;
        let mut forward_iter = line.iter();

        while ! matches!(forward_state, ForwardAutomaton::Done(_)) {
            if let Some(chr) = forward_iter.next() {
                forward_state = forward_state.take(*chr);
            } else {
                return Err(Day01Error::NoMatchesFoundOnLine(unsafe { String::from_utf8_unchecked(Vec::from(line)) }));
            }
        }

        let mut reverse_state = ReverseAutomaton::Empty;
        let mut reverse_iter = line.iter().rev();

        while ! matches!(reverse_state, ReverseAutomaton::Done(_)) {
            if let Some(chr) = reverse_iter.next() {
                reverse_state = reverse_state.take(*chr);
            } else {
                return Err(Day01Error::NoMatchesFoundOnLine(unsafe { String::from_utf8_unchecked(Vec::from(line)) }));
            }
        }

        Ok(forward_state.unwrap() + reverse_state.unwrap())
    }).sum()
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
    Done(u8),
}

impl ForwardAutomaton {
    fn unwrap(self) -> u32 {
        if let Self::Done(num) = self {
            num as u32
        } else {
            panic!("Unwrapped automaton state that was not done {:?}", self)
        }
    }

    fn take(self, chr: u8) -> Self {
        match self {
            ForwardAutomaton::Empty => match chr {
                b'o' => Self::O,
                b't' => Self::T,
                b'f' => Self::F,
                b's' => Self::S,
                b'e' => Self::E,
                b'n' => Self::N,
                b'0' => Self::Done(0),
                b'1' => Self::Done(10),
                b'2' => Self::Done(20),
                b'3' => Self::Done(30),
                b'4' => Self::Done(40),
                b'5' => Self::Done(50),
                b'6' => Self::Done(60),
                b'7' => Self::Done(70),
                b'8' => Self::Done(80),
                b'9' => Self::Done(90),
                _ => Self::Empty,
            },
            ForwardAutomaton::O => match chr {
                b'n' => Self::ON,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::T => match chr {
                b'w' => Self::TW,
                b'h' => Self::TH,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::F => match chr {
                b'o' => Self::FO,
                b'i' => Self::FI,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::S => match chr {
                b'i' => Self::SI,
                b'e' => Self::SE,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::E => match chr {
                b'i' => Self::EI,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::N => match chr {
                b'i' => Self::NI,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::ON => match chr {
                b'e' => Self::Done(10),
                _ => Self::N.take(chr),
            },
            ForwardAutomaton::TW => match chr {
                b'o' => Self::Done(20),
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::TH => match chr {
                b'r' => Self::THR,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::FO => match chr {
                b'u' => Self::FOU,
                _ => Self::O.take(chr),
            },
            ForwardAutomaton::FI => match chr {
                b'v' => Self::FIV,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::SI => match chr {
                b'x' => Self::Done(60),
                _ => Self::Empty.take(chr),

            },
            ForwardAutomaton::SE => match chr {
                b'v' => Self::SEV,
                _ => Self::E.take(chr),
            },
            ForwardAutomaton::EI => match chr {
                b'g' => Self::EIG,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::NI => match chr {
                b'n' => Self::NIN,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::THR => match chr {
                b'e' => Self::THRE,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::FOU => match chr {
                b'r' => Self::Done(40),
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::FIV => match chr {
                b'e' => Self::Done(50),
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::SEV => match chr {
                b'e' => Self::SEVE,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::EIG => match chr {
                b'h' => Self::EIGH,
                _ => Self::Empty.take(chr),
            },
            ForwardAutomaton::NIN => match chr {
                b'e' => Self::Done(90),
                _ => Self::N.take(chr),
            },
            ForwardAutomaton::THRE => match chr {
                b'e' => Self::Done(30),
                _ => Self::E.take(chr),
            },
            ForwardAutomaton::SEVE => match chr {
                b'n' => Self::Done(70),
                _ => Self::E.take(chr),
            },
            ForwardAutomaton::EIGH => match chr {
                b't' => Self::Done(80),
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
    Done(u8),
}

impl ReverseAutomaton {
    fn unwrap(self) -> u32 {
        if let Self::Done(num) = self {
            num as u32
        } else {
            panic!("Unwrapped automaton state that was not done {:?}", self)
        }
    }

    fn take(self, chr: u8) -> Self {
        match self {
            ReverseAutomaton::Empty => match chr {
                b'e' => Self::E,
                b'o' => Self::O,
                b'r' => Self::R,
                b'x' => Self::X,
                b'n' => Self::N,
                b't' => Self::T,
                b'0' => Self::Done(0),
                b'1' => Self::Done(1),
                b'2' => Self::Done(2),
                b'3' => Self::Done(3),
                b'4' => Self::Done(4),
                b'5' => Self::Done(5),
                b'6' => Self::Done(6),
                b'7' => Self::Done(7),
                b'8' => Self::Done(8),
                b'9' => Self::Done(9),
                _ => Self::Empty,
            },
            ReverseAutomaton::E => match chr {
                b'n' => Self::NE,
                b'e' => Self::EE,
                b'v' => Self::VE,
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::O => match chr {
                b'w' => Self::WO,
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::R => match chr {
                b'u' => Self::UR,
                _ => Self::Empty.take(chr)
            }
            ReverseAutomaton::X => match chr {
                b'i' => Self::IX,
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::N => match chr {
                b'e' => Self::EN,
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::T => match chr {
                b'h' => Self::HT,
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::NE => match chr {
                b'o' => Self::Done(1),
                b'i' => Self::INE,
                _ => Self::N.take(chr),
            },
            ReverseAutomaton::WO => match chr {
                b't' => Self::Done(2),
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::EE => match chr {
                b'r' => Self::REE,
                _ => Self::E.take(chr),
            },
            ReverseAutomaton::UR => match chr {
                b'o' => Self::OUR,
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::VE => match chr {
                b'i' => Self::IVE,
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::IX => match chr {
                b's' => Self::Done(6),
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::EN => match chr {
                b'v' => Self::VEN,
                _ => Self::E.take(chr),
            },
            ReverseAutomaton::HT => match chr {
                b'g' => Self::GHT,
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::REE => match chr {
                b'h' => Self::HREE,
                _ => Self::R.take(chr),
            },
            ReverseAutomaton::OUR => match chr {
                b'f' => Self::Done(4),
                _ => Self::O.take(chr),
            },
            ReverseAutomaton::IVE => match chr {
                b'f' => Self::Done(5),
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::VEN => match chr {
                b'e' => Self::EVEN,
                _ => Self::VE.take(chr),
            },
            ReverseAutomaton::GHT => match chr {
                b'i' => Self::IGHT,
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::INE => match chr {
                b'n' => Self::Done(9),
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::HREE => match chr {
                b't' => Self::Done(3),
                _ => Self::Empty.take(chr),
            },
            ReverseAutomaton::EVEN => match chr {
                b's' => Self::Done(7),
                _ => Self::E.take(chr),
            },
            ReverseAutomaton::IGHT => match chr {
                b'e' => Self::Done(8),
                _ => Self::E.take(chr),
            },
            ReverseAutomaton::Done(_) => self,
        }
    }
}

