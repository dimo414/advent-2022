// Initially solved in Bash:
//
// $ paste -s -d + src/bin/01/input.txt | sed 's/++/\n/g' | bc | sort -n -r | head -n 1
// 75501
//
// $ paste -s -d + src/bin/01/input.txt | sed 's/++/\n/g' | bc | sort -n -r | head -n 3 | paste -s -d + | bc
// 215594

use anyhow::{Result, Context};

fn main() -> Result<()> {
    let mut input = parse_input(include_str!("input.txt"))?;
    input.sort_by_key(|w| std::cmp::Reverse(*w));
    println!("Most calories: {}", input[0]);
    println!("Three most calories: {:?}", input[0..3].iter().sum::<u32>());

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<u32>> {
    fn parse_elf(input: &str) -> Result<u32> {
        input.lines()
            .map(|n| n.parse::<u32>().with_context(|| n.to_string()))
            .collect::<Result<Vec<_>, _>>()
            .map(|ns| ns.iter().sum::<u32>())
    }

    input.trim()
        .split("\n\n")
        .map(parse_elf)
        .collect::<Result<Vec<_>, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let example = parse_input(include_str!("example.txt")).unwrap();
        assert_eq!(example, &[6000, 4000, 11000, 24000, 10000]);
    }
}
