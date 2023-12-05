use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Vec<(u32, Option<(char, (usize, usize))>)> {
    let mut nums: HashMap<(usize, usize), usize> = HashMap::new();
    let mut nums_shortlist: Vec<(u32, Option<(char, (usize, usize))>)> = Vec::new();

    let mut num_lines = 0;
    input.lines().enumerate().for_each(|l| {
        let (y, line) = l;

        let mut current_number: u32 = 0;
        let mut current_number_coords: Vec<(usize, usize)> = Vec::new();

        line.chars().enumerate().for_each(|c| {
            let (x, chr) = c;

            if chr.is_ascii_digit() {
                current_number = current_number * 10 + chr.to_digit(10).unwrap();
                current_number_coords.push((x, y));
            } else {
                if ! current_number_coords.is_empty() {
                    nums_shortlist.push((current_number, None));
                    let idx = nums_shortlist.len() - 1;
                    current_number_coords.iter().for_each(|coord| {
                        nums.insert(coord.to_owned(), idx);
                    });
                    current_number_coords.clear();
                    current_number = 0;
                }
            }
        });

        if ! current_number_coords.is_empty() {
            nums_shortlist.push((current_number, None));
            let idx = nums_shortlist.len() - 1;
            current_number_coords.iter().for_each(|coord| {
                nums.insert(coord.to_owned(), idx);
            });
        }

        num_lines = y + 1;
    });

    input.lines().enumerate().for_each(|l| {
        let (y, line) = l;
        line.chars().enumerate().for_each(|c| {
            let (x, chr) = c;

            if ! (chr.is_ascii_digit() || chr == '.') {
                for (rel_x, rel_y) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
                    let actual_x = x as isize - rel_x;
                    let actual_y = y as isize - rel_y;

                    if ! (actual_x < 0 || actual_x == line.len() as isize || actual_y < 0 || actual_y == num_lines as isize) {
                        if let Some(idx) = nums.get(&(actual_x.try_into().unwrap(), actual_y.try_into().unwrap())) {
                            let entry = nums_shortlist.get_mut(*idx).unwrap();
                            entry.1 = Some((chr, (x, y)));
                        }
                    }
                }
            }
        })
    });

    nums_shortlist
}


/// # Example
/// 
/// ```rust
/// use aoc_2023::day03::{ part1, parse };
/// assert_eq!(4361, part1(&parse("467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..")));
/// ```
#[aoc(day3, part1)]
pub fn part1(nums_shortlist: &[(u32, Option<(char, (usize, usize))>)]) -> u32 {
    nums_shortlist.iter().map(|v| {
        if v.1.is_some() {
            v.0
        } else {
            0
        }
    }).sum()
}

/// # Example
/// 
/// ```rust
/// use aoc_2023::day03::{ part2, parse };
/// assert_eq!(467835, part2(&parse("467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..")));
/// ```
#[aoc(day3, part2)]
pub fn part2(nums_shortlist: &[(u32, Option<(char, (usize, usize))>)]) -> u32 {
    let mut gears: HashMap<(usize, usize), u32> = HashMap::new();
    let mut actual_gears: Vec<u32> = Vec::new();

    for (num, part) in nums_shortlist {
        if let Some(('*', (x, y))) = part {
            if let Some(ratio) = gears.get_mut(&(*x, *y)) {
                actual_gears.push((*ratio) * (*num));
                gears.remove(&(*x, *y));
            } else {
                gears.insert((*x, *y), *num);
            }
        }
    }

    actual_gears.iter().sum()
}
