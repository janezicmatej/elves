//! # Input parsers
//!
//! `parsers` is a collection of input parsers to get inputs in correct format quickly and start
//! solving the daily problem ASAP

use std::{fmt::Debug, str::FromStr};

/// separate input with `split`, parse each element to `T` and collect as `Vec<T>`
pub fn as_vec<T>(input: &str, split: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .split(split)
        .map(|x| x.parse::<T>().unwrap())
        .collect()
}

/// shortcut fuction that splits lines with `\n` as that's the most common occurance
pub fn vec_lines<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    as_vec(input, "\n")
}

/// separate input with `split_rows`, each row with `split_row`, parse each element to `T` and collect
/// as `Vec<T>`
pub fn as_vec_vec<T>(input: &str, split_rows: char, split_row: char) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .split(split_rows)
        .map(|x| {
            x.split(split_row)
                .map(|y| y.parse::<T>().unwrap())
                .collect()
        })
        .collect()
}
