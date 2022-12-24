use std::str::FromStr;
use anyhow::{Context, ensure, Error, Result};

fn main() -> Result<()> {
    let input = parse_input(include_str!("input.txt"))?;
    println!("Contains: {}", count_contains(&input));
    println!("Overlaps: {}", count_overlaps(&input));
    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct Range(i32, i32);

impl Range {
    // 'other' is fully within 'self' (asymmetric - a.contains(b) !⇒ b.contains(a) )
    fn contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    // 'other' and 'self' overlap (symmetric - a.overlaps(b) ⇒ b.overlaps(a) )
    fn overlaps(&self, other: &Range) -> bool {
        self.1 >= other.0 && self.0 <= other.1
    }
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split('-').map(|n| n.parse::<i32>().context("")).collect::<Result<Vec<_>>>()?;
        ensure!(parts[0] <= parts[1]);
        Ok(Range(parts[0], parts[1]))
    }
}

fn parse_input(input: &str) -> Result<Vec<(Range, Range)>> {
    fn parse_pair(line: &str) -> Result<(Range, Range)> {
        let parts = line.split(',').map(|r| r.parse::<Range>()).collect::<Result<Vec<_>>>()?;
        ensure!(parts.len() == 2);
        Ok((parts[0], parts[1]))
    }
    input.lines().map(parse_pair).collect()
}

fn count_contains(pairs: &[(Range, Range)]) -> u32 {
    pairs.iter()
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count() as u32
}

fn count_overlaps(pairs: &[(Range, Range)]) -> u32 {
    pairs.iter().filter(|(a, b)| a.overlaps(b)).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_2022::collect::MoreIntoIterator;

    #[test]
    fn check_input() { parse_input(include_str!("input.txt")).unwrap(); }

    parameterized_test::create!{ example, (line, contains, contained, overlaps), {
        let e = parse_input(line)?.take_only()?;
        assert_eq!(e.0.contains(&e.1), contains);
        assert_eq!(e.1.contains(&e.0), contained);
        assert_eq!(e.0.overlaps(&e.1), e.1.overlaps(&e.0));
        assert_eq!(e.0.overlaps(&e.1), overlaps);
    } }
    example! {
        a: ("2-4,6-8", false, false, false),
        b: ("2-3,4-5", false, false, false),
        c: ("5-7,7-9", false, false, true),
        d: ("2-8,3-7", true, false, true),
        e: ("6-6,4-6", false, true, true),
        f: ("2-6,4-8", false, false, true),
    }
}
