use std::collections::BTreeMap;
use std::str::FromStr;
use anyhow::{anyhow, bail, Error, Result};

use advent_2022::euclid::{Point, point, Vector};
use advent_2022::pathfinding::{Edge, Graph};

fn main() -> Result<()> {
    let landscape: Landscape = include_str!("input.txt").parse()?;
    let path = landscape.traverse()?;
    println!("Distance to destination: {}", path.len());

    let best_path = landscape.traverse_backwards()?;
    println!("Distance from best starting point: {}", best_path.len());

    Ok(())
}

#[derive(Debug)]
struct Landscape {
    heights: BTreeMap<Point, u32>,
    start: Point,
    dest: Point,
}

impl Landscape {
    fn traverse(&self) -> Result<Vec<Edge<Point>>> {
        self.dijkstras(&self.start, |&p| p == self.dest).ok_or_else(||anyhow!("No such path"))
    }

    fn traverse_backwards(&self) -> Result<Vec<Edge<Point>>> {
        let view = LandscapeBackwardsView{ landscape:self };
        let start_height = self.heights[&self.start];
        view.dijkstras(&self.dest, |p| self.heights[p] == start_height).ok_or_else(||anyhow!("No such path"))
    }

    // Returns all neighbors and their _relative_ heights
    fn all_neighbors(&self, source: Point) -> impl Iterator<Item=(Point, i32)> + '_ {
        let cur_height = self.heights[&source];
        Vector::CARDINAL.iter()
            .map(move |v| source + v)
            .filter_map(move |d| self.heights.get(&d).map(|&h|(d, h as i32 - cur_height as i32)))
    }
}

impl Graph for Landscape {
    type Node = Point;

    fn neighbors(&self, source: &Self::Node) -> Vec<Edge<Self::Node>> {
        self.all_neighbors(*source)
            .filter_map(|(d, h)| if h <= 1 { Some((d, h)) } else { None })
            .map(|(d, _)| Edge::new(1, *source, d))
            .collect()
    }
}

impl FromStr for Landscape {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        fn to_height(c: char) -> Result<u32> {
            Ok(match c {
                'S' => 1,
                'E' => 26,
                'a'..='z' => c as u32 - 'a' as u32 + 1,
                _ => bail!("Unknown: {}", c),
            })
        }

        let mut heights = BTreeMap::new();
        let mut start = None;
        let mut dest = None;
        for (x, row) in s.lines().enumerate() {
            for (y, c) in row.chars().enumerate() {
                let pos = point(x as i32, y as i32);
                if c == 'S' { start = Some(pos); }
                if c == 'E' { dest = Some(pos); }
                heights.insert(pos, to_height(c)?);
            }
        }
        Ok(Landscape{ heights, start: start.ok_or_else(||anyhow!("Start missing"))?, dest: dest.ok_or_else(||anyhow!("End missing"))?, })
    }
}

struct LandscapeBackwardsView<'a> {
    landscape: &'a Landscape,
}

impl<'a> Graph for LandscapeBackwardsView<'a> {
    type Node = Point;

    fn neighbors(&self, source: &Self::Node) -> Vec<Edge<Self::Node>> {
        self.landscape.all_neighbors(*source)
            .filter_map(|(d, h)| if h >= -1 { Some((d, h)) } else { None })
            .map(|(d, _)| Edge::new(1, *source, d))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_input() { include_str!("input.txt").parse::<Landscape>().unwrap(); }

    #[test]
    fn traverse() {
        let landscape: Landscape = include_str!("example.txt").parse().unwrap();
        let path = landscape.traverse().unwrap();
        assert_eq!(path.len(), 31);
    }

    #[test]
    fn traverse_backwards() {
        let landscape: Landscape = include_str!("example.txt").parse().unwrap();
        let path = landscape.traverse_backwards().unwrap();
        assert_eq!(path.len(), 29);
    }
}
