use std::collections::BTreeSet;
use std::time::Duration;
use anyhow::{bail, Result};

use advent_2022::euclid::{Point, point, Vector, vector};
use advent_2022::terminal::{Color, Terminal, TerminalImage, TerminalRender};

fn main() -> Result<()> {
    let _drop = Terminal::init();
    let input = parse_input(include_str!("input.txt"))?;

    let mut rope = Rope::new(2);
    for dir in &input {
        rope.motion(*dir);
    }
    println!("Length 2: {}", rope.tail_visits.len());
    let mut rope = Rope::new(10);
    for dir in &input {
        rope.motion(*dir);
        Terminal::interactive_render(&rope, Duration::from_millis(0));
    }
    Terminal::end_interactive();
    println!("Length 10: {}", rope.tail_visits.len());

    Ok(())
}

#[derive(Debug)]
struct Rope {
    parts: Vec<Point>,
    tail_visits: BTreeSet<Point>,
}

impl Rope {
    fn new(len: usize) -> Rope {
        assert!(len >= 2);
        Rope { parts: vec![point(0,0); len], tail_visits: BTreeSet::new(), }
    }

    fn motion(&mut self, dir: Vector) {
        // kinda sloppy, maybe pass Vector and a u32 magnitude?
        let dir_sign = dir.signum();
        let dest = self.parts[0] + dir;
        while self.parts[0] != dest {
            self.parts[0] += dir_sign;
            self.resolve_parts();
        }
    }

    fn resolve_parts(&mut self) {
        for i in 1..self.parts.len() {
            let ht_vec = self.parts[i-1] - self.parts[i];
            let dist = ht_vec.grid_len();
            if dist > 4 {
                panic!("Too far @ {}: {:?}", i, self);
            } else if dist == 4 {
                // moved away diagonally, so now two away in each direction, move diagonally
                assert!(ht_vec.x.abs() == 2 && ht_vec.y.abs() == 2);
                self.parts[i] += ht_vec.signum();
            } else if dist == 3 {
                // if not touching and not in the same row/column, move diagonally towards head
                // signum_unchecked() changes the vector to be a proper diagonal, which we want
                assert!(ht_vec.x != 0 && ht_vec.y != 0);
                self.parts[i] += ht_vec.signum_unchecked();
            } else if dist == 2 {
                if ht_vec.x == 0 || ht_vec.y == 0 {
                    // if not touching but in the same row/column, move towards head
                    // otherwise we're "touching" diagonally and don't need to do anything
                    self.parts[i] += ht_vec.signum();
                }
            } else { debug_assert!(dist == 0 || dist == 1); }
        }
        self.tail_visits.insert(*self.parts.last().expect("Must be present"));
    }
}

impl TerminalRender for Rope {
    fn render(&self, width: usize, height: usize) -> TerminalImage {
        // center the viewport over the tail of the rope
        let min = self.parts.last().unwrap() + vector(-(width as i32-1)/2, -(height as i32-1)/2);
        let max = self.parts.last().unwrap() + vector((width as i32)/2, (height as i32)/2);

        let parts: BTreeSet<_> = self.parts.iter().cloned().collect();
        let mut pixels = Vec::new();

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let p = point(x, y);
                if parts.contains(&p) {
                    pixels.push(Color::YELLOW);
                } else if self.tail_visits.contains(&p) {
                    pixels.push(Color::MAGENTA);
                } else {
                    pixels.push(Color::BLUE);
                }
            }
        }

        TerminalImage{ pixels, width, }
    }
}

fn parse_input(input: &str) -> Result<Vec<Vector>> {
    fn to_vector(line: &str) -> Result<Vector> {
        let parts: Vec<_> = line.split(' ').collect();
        let vec = match parts[0] {
            "U" => vector(0, -1),
            "D" => vector(0, 1),
            "L" => vector(-1, 0),
            "R" => vector(1, 0),
            _ => { bail!("Invalid {}", line); },
        };
        Ok(vec * parts[1].parse()?)
    }
    input.lines().map(to_vector).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_input() { parse_input(include_str!("input.txt")).unwrap(); }

    const EXAMPLE_1: &str = include_str!("example-1.txt");
    const EXAMPLE_2: &str = include_str!("example-2.txt");

    parameterized_test::create!{ ropes, (input, len, tail_visits), {
        let input = parse_input(input)?;
    let mut rope = Rope::new(len);
    for dir in input {
        rope.motion(dir);
    }
    assert_eq!(rope.tail_visits.len(), tail_visits);
    } }
    ropes! {
        part1: (EXAMPLE_1, 2, 13),
        part2_example1: (EXAMPLE_1, 10, 1),
        part2_example2: (EXAMPLE_2, 10, 36),
    }
}
