//! # Reusable types
//!
//! `types` is a collection of types common to advent of code

/// Type that is either some literal value `T` or reference key `U` to another `RecValue`. This
/// type usually occurs along with `HashMap<U, _>` or some other kind of mapper. Useful for days
/// like [2022/21](https://adventofcode.com/2022/day/21) or
/// [2015/7](https://adventofcode.com/2015/day/7)
pub enum LiteralOther<T, U> {
    Literal(T),
    Other(U),
}

impl<T, U> From<&str> for LiteralOther<T, U>
where
    T: std::str::FromStr,
    U: std::str::FromStr,
    <U as std::str::FromStr>::Err: core::fmt::Debug,
{
    fn from(value: &str) -> Self {
        match value.parse::<T>() {
            Ok(x) => Self::Literal(x),
            Err(_) => Self::Other(value.parse::<U>().unwrap()),
        }
    }
}
