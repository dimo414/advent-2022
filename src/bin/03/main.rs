use std::collections::{HashMap, HashSet};
use anyhow::Result;

fn main() -> Result<()> {
    let mut misplaced_sum = 0;
    let mut badge_sum = 0;
    for group in include_str!("input.txt").lines().collect::<Vec<_>>().chunks(3) {
        for elf in group {
            misplaced_sum += score_set(&misplaced_item(elf));
        }
        badge_sum += score_set(&intersect_all_2(&group));
    }
    println!("Misplaced priority: {}", misplaced_sum);
    println!("Badges priority: {}", badge_sum);
    Ok(())
}

fn misplaced_item(elf: &str) -> HashSet<char> {
    let left = &elf[..elf.len()/2];
    let right = &elf[elf.len()/2..];
    intersect_all_2(&[left, right])
}

#[cfg(test)]
fn intersect_all_1(inputs: &[&str]) -> HashSet<char> {
    let packs: Vec<_> = inputs.iter().map(|p| p.chars().collect::<HashSet<_>>()).collect();
    let all_items: HashSet<_> = inputs.iter().flat_map(|p| p.chars()).collect();
    packs.iter().fold(all_items, |all, cur| &all & cur)
}

fn intersect_all_2(inputs: &[&str]) -> HashSet<char> {
    if inputs.is_empty() { return HashSet::new(); }
    let mut inputs = inputs.iter();
    let mut keep = true;
    let mut intersection: HashMap<_,_> = inputs.next().expect("non-empty").chars().map(|c| (c, !keep)).collect();
    for input in inputs {
        for c in input.chars() {
            if let Some(v) = intersection.get_mut(&c) {
                *v = keep;
            }
        }
        // could also set v=false here and not invert the keep variable, guessing it'd be slower
        intersection.retain(|_, &mut v| v == keep);
        keep = !keep;
    }
    intersection.into_keys().collect()
}

fn score_set(common: &HashSet<char>) -> u32 {
    assert_eq!(common.len(), 1);
    score(*common.iter().next().expect("non-empty"))
}

fn score(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 +1,
        'A'..='Z' => c as u32 - 'A' as u32 +27,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn misplaced() {
        let elves: Vec<_> = include_str!("example.txt").lines().collect();
        let misplaced: Vec<_> = elves.iter().map(|e| misplaced_item(e).into_iter().collect::<String>()).collect();
        assert_eq!(misplaced, &["p", "L", "P", "v", "t", "s"]);
        let scores: Vec<_> = misplaced.iter().map(|s| score(s.chars().next().unwrap())).collect();
        assert_eq!(scores, &[16, 38, 42, 22, 20, 19]);
    }

    #[test]
    fn groups() {
        let elves: Vec<_> = include_str!("example.txt").lines().collect();
        let groups: Vec<_> = elves.chunks(3).collect();
        let badges: Vec<_> = groups.iter().map(|g| intersect_all_2(g).into_iter().collect::<String>()).collect();
        assert_eq!(badges, &["r", "Z"]);
        let scores: Vec<_> = badges.iter().map(|s| score(s.chars().next().unwrap())).collect();
        assert_eq!(scores, &[18, 52]);
    }

    parameterized_test::create!{ intersect_impls, intersect, {
        let inputs = &["foobaaaar", "bar", "zab", "bang"];
        let expected: HashSet<_> = "ba".chars().collect();
        assert_eq!(intersect(inputs), expected);
    } }
    intersect_impls! {
       all_1: intersect_all_1,
       all_2: intersect_all_2,
    }
}
