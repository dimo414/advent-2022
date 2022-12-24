use anyhow::{anyhow, Result};
use advent_2022::terminal::elapsed;

fn main() -> Result<()> {
    let data = include_str!("input.txt");
    println!("First Packet: {}", elapsed!(find_marker(data, 4))?);
    println!("First Message: {}", elapsed!(find_marker(data, 14))?);

    Ok(())
}

fn find_marker(data: &str, length: usize) -> Result<usize> {
    data.as_bytes() // collecting to a Vec<char> would be more correct, but who has the time?
        .windows(length).enumerate()
        .filter(|(_,w)| all_different(w))
        .map(|(i,_)| i+length).next()
        .ok_or_else(|| anyhow!("No segments found with length {}", length))
}

fn all_different(s: &[u8]) -> bool {
    // Even if it's a little redundant, this is much faster than
    // s.iter().collect::<std::collections::HashSet<_>>().len() == s.len()
    for i in 0..s.len() {
        for j in i+1..s.len() {
            if s[i] == s[j] { return false; }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_input() { assert!(!include_str!("input.txt").is_empty()); }

    parameterized_test::create!{ examples, (data, idx4, idx14), {
        assert_eq!(find_marker(data, 4).unwrap(), idx4);
        assert_eq!(find_marker(data, 14).unwrap(), idx14);
    } }
    examples! {
        a: ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        b: ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        c: ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        d: ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        e: ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    }
}
