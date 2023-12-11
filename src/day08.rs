use std::{rc::{Weak, Rc}, collections::HashMap, hash::Hash, cell::RefCell};

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{IResult, Err, error::Error, character::complete::{space0, char, newline, alphanumeric0}, sequence::{terminated, tuple}, combinator::map_res, multi::{separated_list0, many_till, many1}, branch::alt, Parser};
use thiserror::Error;

#[derive(Clone)]
pub struct Graph<Name> {
    nodes: HashMap<Name, Rc<RefCell<Node<Name>>>>,
}

#[derive(Clone)]
struct Node<Name> {
    pub name: Name,
    pub left: RefCell<Weak<RefCell<Self>>>,
    pub right: RefCell<Weak<RefCell<Self>>>,
}

#[derive(Debug, Error)]
pub enum Day08Error {
    #[error("Failed to parse due to remainder: {0}")]
    Rest(String),
    #[error("Failed to parse due to nom error: {0}")]
    ParseError(#[from] Err<Error<String>>),
}

impl From<Err<Error<&str>>> for Day08Error {
    fn from(value: Err<Error<&str>>) -> Self {
        Self::from(value.to_owned())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Step {
    Left,
    Right,
}

impl Step {
    fn parse_many(input: &str) -> IResult<&str, Vec<Self>> {
        map_res(
            many_till(alt((
                map_res(char('L'), |_| Result::<_, Error<&str>>::Ok(Self::Left)),
                map_res(char('R'), |_| Result::<_, Error<&str>>::Ok(Self::Right)))), many1(newline)),
            |(a, _)| Result::<_, Error<&str>>::Ok(a)
        )(input)
    }
}

impl<Name> Node<Name> {
    fn new(name: Name) -> Self {
        Node {
            name,
            left: Default::default(),
            right: Default::default(),
        }
    }
    fn set_left(&mut self, other: Weak<RefCell<Self>>) {
        self.left.replace(other);
    }

    fn set_right(&mut self, other: Weak<RefCell<Self>>) {
        self.right.replace(other);
    }
}

impl<Name> Graph<Name> {
    fn new() -> Self {
        Self { nodes: HashMap::new() }
    }
}

impl<Name> Default for Graph<Name> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Name: Hash + Eq + Clone> Graph<Name> {
    fn from_vec<N: Into<Name> + Sized, L: Into<Name> + Sized, R: Into<Name> + Sized>(tuples: Vec<(N, L, R)>) -> Self {
        let mut g = Graph::new();
        g.add_from_vec(tuples);
        g
    }

    fn add_from_vec<N: Into<Name> + Sized, L: Into<Name> + Sized, R: Into<Name> + Sized>(&mut self, tuples: Vec<(N, L, R)>) {
        self.nodes.reserve(tuples.len());
        for (a, b, c) in tuples {
            self.add(a.into(), b.into(), c.into());
        }
    }

    fn add(&mut self, name: Name, left: Name, right: Name) {
        fn insert_empty<N: Eq + Hash + Clone>(s: &mut Graph<N>, name: N) -> Weak<RefCell<Node<N>>> {
            let n = Rc::new(RefCell::new(Node::new(name.clone())));

            let weak_node = Rc::downgrade(&n);
            s.nodes.insert(name, n);

            weak_node
        }

        let node = if let Some(n) = self.nodes.get(&name) {
            Rc::downgrade(n)
        } else {
            insert_empty(self, name)
        };

        let left_node = if let Some(n) = self.nodes.get(&left) {
            Rc::downgrade(n)
        } else {
            insert_empty(self, left)
        };

        let right_node = if let Some(n) = self.nodes.get(&right) {
            Rc::downgrade(n)
        } else {
            insert_empty(self, right)
        };

        // let mut node = Weak::upgrade(&node).unwrap();
        // let node = Rc::get_mut(&mut node).unwrap();
        Weak::upgrade(&node).map(|v| {
            v.borrow_mut().set_left(left_node);
            v.borrow_mut().set_right(right_node);
        });
    }

    fn solve(&self, walk: &[Step], start_node: Rc<RefCell<Node<Name>>>, end_predicate: impl Fn(&Name) -> bool) -> u32 {
        let mut current = start_node;
        let mut steps = 0u32;

        for step in walk.iter().clone().cycle() {
            if end_predicate(&current.borrow().name) {
                break;
            }
            steps += 1;
            current = match step {
                Step::Left => {
                    current.borrow().left.borrow().upgrade().unwrap()
                },
                Step::Right => {
                    current.borrow().right.borrow().upgrade().unwrap()
                }
            }
        }

        steps
    }
}

impl Graph<String> {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list0(newline, tuple((
            terminated(alphanumeric0::<&str, _>, tuple((space0, char('='), space0, char('(')))),
            terminated(alphanumeric0, tuple((space0, char(','), space0))),
            terminated(alphanumeric0, char(')')),
        ))).map(Graph::from_vec).parse(input)
    }
}

#[aoc_generator(day8)]
pub fn parse(input: &str) -> Result<(Vec<Step>, Graph<String>), Day08Error> {
    let (input, steps) = Step::parse_many(input)?;

    let graph = match Graph::parse(input)? {
        ("", result) => Ok(result),
        (rest, _) => Err(Day08Error::Rest(rest.to_owned())),
    }?;

    Ok((steps, graph))
}

/// # Examples
/// 
/// ```
/// use aoc_2023::day08::{ parse, part1 };
/// 
/// assert_eq!(2, part1(&parse("RL
///
/// AAA = (BBB, CCC)
/// BBB = (DDD, EEE)
/// CCC = (ZZZ, GGG)
/// DDD = (DDD, DDD)
/// EEE = (EEE, EEE)
/// GGG = (GGG, GGG)
/// ZZZ = (ZZZ, ZZZ)").unwrap()));
/// 
/// assert_eq!(6, part1(&parse("LLR
///
/// AAA = (BBB, BBB)
/// BBB = (AAA, ZZZ)
/// ZZZ = (ZZZ, ZZZ)").unwrap()));
/// ```
#[aoc(day8, part1)]
pub fn part1((walk, graph): &(Vec<Step>, Graph<String>)) -> u32 {
    let current = graph.nodes.get("AAA").unwrap().to_owned();

    graph.solve(&walk, current, |name| name == "ZZZ")
}

/// # Examples
/// 
/// ```
/// use aoc_2023::day08::{ parse, part2 };
/// 
/// assert_eq!(6, part2(&parse("LR
///
/// 11A = (11B, XXX)
/// 11B = (XXX, 11Z)
/// 11Z = (11B, XXX)
/// 22A = (22B, XXX)
/// 22B = (22C, 22C)
/// 22C = (22Z, 22Z)
/// 22Z = (22B, 22B)
/// XXX = (XXX, XXX)").unwrap()))
/// ```
#[aoc(day8, part2)]
pub fn part2((walk, graph): &(Vec<Step>, Graph<String>)) -> usize {
    graph.nodes.iter()
        .filter(|n| n.0.ends_with("A"))
        .map(|node|
            graph.solve(walk, node.1.to_owned(), |cur| cur.ends_with("Z")) as usize)
        .reduce(num::integer::lcm).unwrap()
}
