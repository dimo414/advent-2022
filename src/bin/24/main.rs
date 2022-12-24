use std::fmt::{Display, Formatter};
use std::str::FromStr;
use anyhow::{anyhow, Error, Result};

use advent_2022::euclid::{Point, point, Vector, vector};
use advent_2022::pathfinding::{Edge, Graph};

fn main() -> Result<()> {
    let valley: Valley = include_str!("input.txt").parse()?;
    let traverse = valley.traverse()?;
    println!("First traversal: {}", traverse[0]);
    println!("Back and forth: {}", traverse.iter().sum::<usize>());

    Ok(())
}

struct Direction {
    symbol: char,
    shift: Vector,
}

impl Direction {
    fn contains(&self, grid: &Vec<Vec<char>>, time: i32, pos: Point) -> bool {
        fn safe_index(idx: i32, len: usize) -> usize {
            let len: i32 = len.try_into().expect("Unsupported");
            let safe_idx = ((idx % len) + len) % len;
            safe_idx.try_into().expect("Invalid index")
        }

        let offset = self.shift * time;
        let target = pos + offset;
        let row = &grid[safe_index(target.y, grid.len())];
        row[safe_index(target.x, row.len())] == self.symbol
    }
}

struct Valley {
    grid: Vec<Vec<char>>,
    dirs: [Direction; 4],
    source: Point,
    dest: Point,
}

impl Valley {
    fn traverse(&self) -> Result<[usize; 3]> {
        let path1 = self.bfs(&(self.source, 0), |&(pos, _)| pos == self.dest).ok_or_else(|| anyhow!("No path found"))?;
        let path2 = self.bfs(path1.last().expect("Path"), |&(pos, _)| pos == self.source).ok_or_else(|| anyhow!("No path found"))?;
        let path3 = self.bfs(path2.last().expect("Path"), |&(pos, _)| pos == self.dest).ok_or_else(|| anyhow!("No path found"))?;
        Ok([path1.len() - 1, path2.len() - 1, path3.len() - 1])
    }

    fn open(&self, time: i32, pos: Point) -> bool {
        if let Some(row) = self.grid.get(pos.y as usize) {
            if row.get(pos.x as usize).is_some() {
                return !self.dirs.iter().any(|d| d.contains(&self.grid, time, pos));
            }
        }
        false
    }

    #[cfg(test)]
    fn display(&self, time: i32) -> ValleyDisplay {
        ValleyDisplay{ valley: self, time }
    }
}

impl Graph for Valley {
    type Node = (Point, i32);

    fn neighbors(&self, source: &Self::Node) -> Vec<Edge<Self::Node>> {
        let (cur, time) = *source;
        let next = time + 1;
        let mut dests: Vec<_> = Vector::CARDINAL.iter().map(|v| cur + v).filter(|&p| self.open(next, p)).collect();
        if self.open(next, cur) {
            dests.push(cur);
        }
        if cur == self.dest + vector(0, -1) {
            dests.push(self.dest);
        }
        if cur == self.dest {
            dests.push(self.dest + vector(0, -1));
        }
        if cur == point(0, 0) {
            dests.push(point(0, -1));
        }
        if cur == point(0, -1) {
            dests.push(point(0, 0));
        }
        dests.into_iter().map(|d| Edge::new(1, *source, (d, next))).collect()
    }
}

impl FromStr for Valley {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let grid: Vec<Vec<char>> = s.lines()
            // skip first and last row
            .skip_while(|l| l.starts_with("#.###")).take_while(|l| !l.ends_with("###.#"))
            // skip first and last column
            .map(|l| l.chars().skip_while(|&c| c == '#').take_while(|&c| c != '#').collect()).collect();

        assert!(!grid.is_empty());
        let width = grid[0].len();
        assert!(grid.iter().all(|r| r.len() == width));
        let dest = point(grid[0].len() as i32 - 1, grid.len() as i32);
        Ok(Valley{
            grid,
            dirs: [
                Direction{ symbol: '^', shift: vector(0, 1) },
                Direction{ symbol: 'v', shift: vector(0, -1) },
                Direction{ symbol: '>', shift: vector(-1, 0) },
                Direction{ symbol: '<', shift: vector(1, 0) },
            ],
            source: point(0, -1),
            dest,
        })
    }
}

struct ValleyDisplay<'a> {
    valley: &'a Valley,
    time: i32,
}

impl<'a> Display for ValleyDisplay<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        out.push_str("#.");
        out.push_str(&"#".repeat(self.valley.grid[0].len()));
        out.push('\n');
        for y in 0..self.valley.grid.len() {
            out.push('#');
            for x in 0..self.valley.grid[y].len() {
                let pos = point(x as i32, y as i32);
                let mut c = '.';
                let mut count = 0;
                for dir in &self.valley.dirs {
                    if dir.contains(&self.valley.grid, self.time, pos) {
                        c = dir.symbol;
                        count += 1;
                    }
                }
                if count > 1 {
                    c = char::from_digit(count, 10).expect("Not a digit");
                }
                out.push(c);
            }
            out.push('#');
            out.push('\n');
        }
        out.push_str(&"#".repeat(self.valley.grid[0].len()));
        out.push_str(".#");
        out.push('\n');
        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_input() { include_str!("input.txt").parse::<Valley>().unwrap(); }

    fn check_movement(input: &str, expected: &str) {
        let valley: Valley = input.parse().unwrap();
        let height = valley.grid.len();
        let width = valley.grid[0].len();
        let cycle = num::integer::lcm(width,height);
        assert_eq!(format!("{}", valley.display(cycle as i32)), format!("{}", valley.display(0)), "Cycled");

        let steps: Vec<_> = expected.split("\n\n").collect();
        for i in 0..steps.len() {
            assert_eq!(format!("{}", valley.display(i as i32)).trim(), steps[i], "After {} steps", i);
        }
    }

    parameterized_test::create!{ movement, (input, expected), {
        check_movement(input, expected);
    } }
    movement! {
        example_1: (include_str!("example-1.txt"), include_str!("expected-1.txt")),
        example_2: (include_str!("example-2.txt"), include_str!("expected-2.txt")),
    }

    #[test]
    fn traverse() {
        let valley: Valley = include_str!("example-2.txt").parse().unwrap();
        assert_eq!(valley.traverse().unwrap(), [18, 23, 13]);
    }
}
