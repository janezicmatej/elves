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

/// Nested array, e.g.: [1,[2,3],4,[[5]]]. Useful for days like
/// [2022/13](https://adventofcode.com/2022/day/13) or
/// [2022/18](https://adventofcode.com/2021/day/18)
pub enum NestedArray<T> {
    Literal(T),
    List(Vec<NestedArray<T>>),
}

impl<T> From<&str> for NestedArray<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: core::fmt::Debug,
{
    fn from(value: &str) -> Self {
        let mut stack: Vec<Vec<NestedArray<T>>> = Vec::new();
        let mut chars = value.chars();
        let mut buf = None;
        while let Some(c) = buf.take().or_else(|| chars.next()) {
            match c {
                ',' => (),
                '0'..='9' => {
                    let literal = std::iter::once(c)
                        .chain(std::iter::from_fn(|| {
                            buf = chars.next();
                            buf.filter(|cc| cc.is_ascii_digit())
                        }))
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    stack
                        .last_mut()
                        .unwrap()
                        .push(NestedArray::Literal(literal));
                }
                '[' => stack.push(Vec::new()),
                ']' => {
                    let list = NestedArray::List(stack.pop().unwrap());
                    match stack.last_mut() {
                        None => return list,
                        Some(x) => x.push(list),
                    }
                }
                _ => unreachable!(),
            }
        }
        unreachable!()
    }
}
