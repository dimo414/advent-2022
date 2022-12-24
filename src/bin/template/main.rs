// FIXME delete this
// https://github.com/rust-lang/cargo/issues/3591#issuecomment-475701083
#![ allow( dead_code, unused_imports, unused_macros, unused_variables ) ]

use std::str::FromStr;
use anyhow::*;

use advent_2022::parsing::*;
use advent_2022::collect::{MoreIntoIterator,MoreItertools};

fn main() -> Result<()> {
    let input = parse_input(include_str!("input.txt"))?;
    input.iter().for_each(|o| println!("{:?}", o));

    Ok(())
}

#[derive(Debug)]
struct Obj {
    str: String,
}

impl FromStr for Obj {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let regex = static_regex!(r"Hello (.*)!");
        let caps = regex_captures(regex, s)?;
        Ok(Obj{ str: capture_group(&caps, 1).to_string() })
    }
}

fn parse_input(input: &str) -> Result<Vec<Obj>> {
    input.lines().map(|l| l.parse()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_input() { parse_input(include_str!("input.txt")).unwrap(); }

    parameterized_test::create!{ delete, n, { assert_eq!(n % 2, 0); } }
    delete! {
        me: 2,
    }
}
