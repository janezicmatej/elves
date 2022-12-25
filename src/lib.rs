//! `elves` is a collection of common algorithms, parsers, types and traits for advent of code

pub mod parsers;
pub mod types;

// macros

/// minimum of multiple items by recursively calling `std::cmp::min`
#[macro_export]
macro_rules! many_min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::min($x, many_min!($($z),*)));
}

/// maximum of multiple items by recursively calling `std::cmp::max`
#[macro_export]
macro_rules! many_max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::max($x, many_max!($($z),*)));
}
