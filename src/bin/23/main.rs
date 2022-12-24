use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use anyhow::{bail, Error, Result};

use advent_2022::euclid::{Point, point, Vector, vector};

fn main() -> Result<()> {
    let mut input: Map = include_str!("input.txt").parse()?;

    let mut round = 1;
    while input.round() {
        if round == 10 {
            println!("After 10 rounds, open ground: {}", input.open_ground());
        }
        round += 1;
    }
    println!("Rounds: {}", round);
    Ok(())
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    North, South, West, East,
}

impl Dir {
    fn check_from(&self, source: Point) -> Vec<Point> {
        match self {
            Dir::North => [vector(-1, -1), vector(0, -1), vector(1, -1)],
            Dir::South => [vector(-1, 1), vector(0, 1), vector(1, 1)],
            Dir::West => [vector(-1, -1), vector(-1, 0), vector(-1, 1)],
            Dir::East => [vector(1, -1), vector(1, 0), vector(1, 1)],
        }.iter().map(|v| source + v).collect()
    }

    fn move_from(&self, source: Point) -> Point {
        match self {
            Dir::North => source + vector(0, -1),
            Dir::South => source + vector(0, 1),
            Dir::West => source + vector(-1, 0),
            Dir::East => source + vector(1, 0),
        }
    }
}

struct Map {
    elves: HashSet<Point>,
    order: VecDeque<Dir>,
}

impl Map {
    fn round(&mut self) -> bool {
        let mut destinations = HashMap::new();
        for elf in &self.elves {
            if let Some(dest) = self.next_move(*elf) {
                match destinations.entry(dest) {
                    Entry::Vacant(entry) => { entry.insert(Some(*elf)); },
                    Entry::Occupied(mut entry) => { entry.insert(None); },
                }
            }
        }

        let ret = !destinations.is_empty();
        for (source, dest) in destinations.into_iter().filter_map(|(dest, source)| source.map(|s| (s, dest))) {
            assert!(self.elves.remove(&source), "Must be present");
            assert!(self.elves.insert(dest), "Must be absent");
        }

        self.order.rotate_left(1);
        ret
    }

    fn next_move(&self, elf: Point) -> Option<Point> {
        // Checking all the ordinals is redundant with the calls in check_from, could maybe be improved
        if Vector::ORDINAL.iter().map(|v| elf + v).any(|d| self.elves.contains(&d)) {
            for dir in &self.order {
                if !dir.check_from(elf).iter().any(|d| self.elves.contains(d)) {
                    return Some(dir.move_from(elf));
                }
            }
        }
        None
    }

    fn open_ground(&self) -> i32 {
        let (min, max) = Point::bounding_box(&self.elves).expect("Can't be empty");
        let area = (max.x - min.x + 1) * (max.y - min.y + 1);
        area - self.elves.len() as i32
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut elves = HashSet::new();
        for (row, line) in s.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                match char {
                    '#' => { elves.insert(point(col as i32, row as i32)); },
                    '.' => { /* noop */ },
                    _ => bail!("Unexpected char: {}", char),
                }
            }
        }
        let order = [Dir::North, Dir::South, Dir::West, Dir::East].into_iter().collect();
        Ok(Map { elves, order })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Point::display_point_set(&self.elves, '#', '.'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_input() { include_str!("input.txt").parse::<Map>().unwrap(); }

    #[test]
    fn small_example() {
        let mut example: Map = include_str!("example-1.txt").parse().unwrap();
        assert_eq!(format!("{}", example), "##\n#.\n..\n##\n");
        assert!(example.round());
        assert_eq!(format!("{}", example), "##\n..\n#.\n.#\n#.\n");
        assert!(example.round());
        assert_eq!(format!("{}", example), ".##.\n#...\n...#\n....\n.#..\n");
        assert!(example.round());
        assert_eq!(format!("{}", example), "..#..\n....#\n#....\n....#\n.....\n..#..\n");
        assert!(!example.round());
    }

    #[test]
    fn large_example() {
        let mut example: Map = include_str!("example-2.txt").parse().unwrap();
        for _ in 0..10 {
            assert!(example.round());
        }
        assert_eq!(example.open_ground(), 110);
        for _ in 10..19 {
            assert!(example.round());
        }
        assert!(!example.round());
    }
}
