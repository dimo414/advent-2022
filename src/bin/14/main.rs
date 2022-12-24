use std::collections::BTreeSet;
use std::str::FromStr;
use std::time::Duration;
use anyhow::{ensure, Error, Result};
use itertools::Itertools;

use advent_2022::euclid::{Point, point, vector};
use advent_2022::terminal::{Color, Terminal, TerminalImage, TerminalRender};

fn main() -> Result<()> {
    let _drop = Terminal::init();
    let mut cave: Cave = include_str!("input.txt").parse()?;

    advent_2022::elapsed!("Initial",
        while cave.pour() == Outcome::Stuck {
            Terminal::interactive_render(&cave.view(None), Duration::from_millis(1));
        });
    Terminal::end_interactive();
    println!("Sand accumulated (initial): {}", cave.count_sand() - 1);
    advent_2022::elapsed!("Filled",
    while cave.pour() != Outcome::Blocked {
        Terminal::interactive_render(&cave.view(None), Duration::from_millis(1));
    });

    Terminal::end_interactive();
    println!("Sand accumulated (filled): {}", cave.count_sand());

    Ok(())
}

#[derive(Eq, PartialEq, Debug)]
enum Outcome {
    Stuck,
    Floor,
    Blocked,
}

#[derive(Debug)]
struct Cave {
    walls: BTreeSet<Point>,
    abyss: i32,
    sand: BTreeSet<Point>,
}

impl Cave {
    fn new(walls: BTreeSet<Point>) -> Cave {
        let abyss = walls.iter().map(|p| p.y).max().expect("Must have a wall") + 2;
        Cave{ walls, abyss, sand: BTreeSet::new(), }
    }

    fn count_sand(&self) -> usize { self.sand.len() }

    fn blocked(&self, pos: Point) -> bool {
        pos.y >= self.abyss || self.walls.contains(&pos) || self.sand.contains(&pos)
    }

    // TODO it may be possible to speed this up by bulk-filling sections that overflow left or right
    fn pour(&mut self) -> Outcome {
        let source = point(500, 0);
        if self.sand.contains(&source) { return Outcome::Blocked; }
        let mut cur_sand = source;
        loop {
            debug_assert!(!self.sand.contains(&cur_sand));
            debug_assert!(!self.walls.contains(&cur_sand));
            Terminal::interactive_render(&self.view(Some(cur_sand)), Duration::from_millis(0));
            let below = cur_sand + vector(0, 1);
            if !self.blocked(below) {
                cur_sand = below;
            } else {
                let below_left = cur_sand + vector(-1, 1);
                if !self.blocked(below_left) {
                    cur_sand = below_left;
                } else {
                    let below_right = cur_sand + vector(1, 1);
                    if !self.blocked(below_right) {
                        cur_sand = below_right;
                    } else {
                        self.sand.insert(cur_sand);
                        return if cur_sand.y+1 == self.abyss { Outcome::Floor } else { Outcome::Stuck };
                    }
                }
            }
        }
    }

    fn view(&self, cur_sand: Option<Point>) -> CaveView {
        CaveView{ cave: self, cur_sand, }
    }
}

impl FromStr for Cave {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        fn to_points(line: &str) -> Result<Vec<Point>> {
            let corners = line.split(" -> ").map(|p| p.parse()).collect::<Result<Vec<Point>>>()?;
            ensure!(corners.len() >= 2);
            let mut points = Vec::new();
            for i in 0..corners.len()-1 {
                let source = corners[i];
                let dest = corners[i+1];
                let dir = (dest - source).signum();
                let mut cur = source;
                while cur != dest {
                    points.push(cur);
                    cur += dir;
                }
                points.push(dest);
            }
            Ok(points)
        }

        let walls = s.lines().map(to_points).flatten_ok().collect::<Result<BTreeSet<Point>>>()?;
        Ok(Cave::new(walls))
    }
}

struct CaveView<'a> {
    cave: &'a Cave,
    cur_sand: Option<Point>,
}

impl<'a> TerminalRender for CaveView<'a> {
    fn render(&self, width: usize, height: usize) -> TerminalImage {
        // Determine the viewport
        let (min, max) = Point::bounding_box(self.cave.walls.iter().chain([point(500,0)].iter())).unwrap();
        let (mut min, mut max) = (min + vector(-5, -1), max + vector(5, 1));
        if (width as i32) < max.x - min.x {
            let offset = ((max.x - min.x) - width as i32) / 2;
            min.x += offset;
            max.x -= offset;
        }
        if (height as i32) < max.y - min.y {
            let offset = ((max.y - min.y) - height as i32) / 2;
            min.y += offset;
            max.y -= offset;
        }

        let mut pixels = Vec::new();
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let p = point(x, y);
                if Some(p) == self.cur_sand {
                    pixels.push(Color::BROWN);
                } else if p == point(500, 0) {
                    pixels.push(Color::BLUE);
                } else if self.cave.walls.contains(&p) {
                    pixels.push(Color::GREYSCALE(0.2));
                } else if self.cave.sand.contains(&p) {
                    pixels.push(Color::YELLOW);
                } else {
                    pixels.push(Color::GREYSCALE(0.6));
                }
            }
        }

        TerminalImage{ pixels, width: (max.x - min.x + 1) as usize, }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_input() { include_str!("input.txt").parse::<Cave>().unwrap(); }

    #[test]
    fn example() {
        let mut cave: Cave = include_str!("example.txt").parse().unwrap();
        while cave.pour() == Outcome::Stuck {}
        assert_eq!(cave.count_sand()-1, 24); // last sand placed is Floor, not Stuck

        while cave.pour() != Outcome::Blocked {}
        assert_eq!(cave.count_sand(), 93);
    }
}
