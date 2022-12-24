use std::collections::{BTreeSet, VecDeque};
use anyhow::Result;

use advent_2022::euclid3d::{Point, Vector, vector};

fn main() -> Result<()> {
    let points = parse_input(include_str!("input.txt"))?;
    println!("Exposed Surfaces: {}", count_all_exposed(&points));
    let (min, max) = wide_bounds(&points);
    println!("External Surfaces: {}", floodfill_surfaces(min, min, max,&points));
    Ok(())
}

fn count_exposed(pos: Point, points: &BTreeSet<Point>) -> usize {
    Vector::CARDINAL.iter().map(|v| pos + v).filter(|n| !points.contains(n)).count()
}

fn count_all_exposed(points: &BTreeSet<Point>) -> usize {
    points.iter().map(|p| count_exposed(*p, points)).sum()
}

fn wide_bounds<'a>(points: impl IntoIterator<Item = &'a Point>) -> (Point, Point) {
    let (min, max) = Point::bounding_box(points).expect("Not empty");
    (min + vector(-1,-1,-1), max + vector(1, 1, 1))
}

fn floodfill_surfaces(seed: Point, min: Point, max: Point, blocked: &BTreeSet<Point>) -> usize {
    let mut seen = BTreeSet::new();
    let mut pending = VecDeque::new();
    pending.push_back(seed);
    seen.insert(seed);
    let mut ret = 0;
    while let Some(pos) = pending.pop_front() {
        pending.extend(
            Vector::CARDINAL.iter().map(|v| pos + v)
                .filter(|p| !seen.contains(p))
                .filter(|p| p.in_bounds(min, max) && !blocked.contains(p))
        );
        seen.extend(Vector::CARDINAL.iter().map(|v| pos + v));
        ret += Vector::CARDINAL.iter().map(|v| pos + v).filter(|p| blocked.contains(p)).count();
    }
    ret
}

fn parse_input(input: &str) -> Result<BTreeSet<Point>> {
    input.lines().map(|l| l.parse()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_2022::euclid3d::point;

    #[test]
    fn check_input() { parse_input(include_str!("input.txt")).unwrap(); }

    #[test]
    fn minimal_example() {
        let points: BTreeSet<_> = [point(1, 1, 1), point(2, 1, 1)].into_iter().collect();
        assert_eq!(count_all_exposed(&points), 10);
    }

    #[test]
    fn example() {
        let points = parse_input(include_str!("example.txt")).unwrap();
        assert_eq!(count_all_exposed(&points), 64);
        let (min, max) = wide_bounds(&points);
        assert_eq!(floodfill_surfaces(min, min, max,&points), 58);
    }
}
