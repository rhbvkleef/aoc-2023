// pub mod naive;
pub mod optimized;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day01Error {
    #[error("Could not convert {0} to number.")]
    NumberUnrecognized(String),
    #[error("Could not find numbers on line {0}")]
    NoMatchesFoundOnLine(String),
}

fn firstlast<T: Clone>(mut it: impl Iterator<Item = T>) -> Option<(T, T)> {
    if let Some(first) = it.next() {
        let last = it.last().unwrap_or(first.clone());

        Some((first, last))
    } else {
        None
    }
}
