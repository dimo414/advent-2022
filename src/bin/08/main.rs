use std::collections::{BTreeMap, BTreeSet};
use anyhow::{anyhow, Result};

use advent_2022::euclid::{Point, point, Vector, vector};

fn main() -> Result<()> {
    let forest = parse_input(include_str!("input.txt"))?;
    let visible = find_visible_trees(&forest)?;
    println!("Visible trees: {}", visible.len());

    let best_scenic_score =    forest.keys()
        .map(|&tree| (tree, scenic_score(&forest, tree)))
        .max_by_key(|(_,s)|*s)
        .expect("Non-empty");
    println!("Highest scenic score: {}", best_scenic_score.1);

    Ok(())
}

fn parse_input(input: &str) -> Result<BTreeMap<Point, i32>> {
    let mut ret = BTreeMap::new();
    for (pos_y, line) in input.lines().enumerate() {
        for (pos_x, height) in line.chars().enumerate() {
            let pos = point(pos_x as i32, pos_y as i32);
            ret.insert(pos, height.to_digit(10).ok_or_else(|| anyhow!("Invalid digit"))? as i32);
        }
    }
    Ok(ret)
}

fn find_visible_trees(forest: &BTreeMap<Point, i32>) -> Result<BTreeSet<Point>> {
    fn check_visibility(forest: &BTreeMap<Point, i32>, start: Point, dir: Vector) -> Vec<Point> {
        let mut max_height = -1;
        let mut visible = Vec::new();
        let mut tree = start;
        while let Some(&height) = forest.get(&tree) {
            if height > max_height {
                visible.push(tree);
                max_height = height;
            }
            tree += dir;
        }
        visible
    }

    let bounds = Point::bounding_box(forest.keys()).ok_or_else(|| anyhow!("Empty bounds"))?;

    Ok((bounds.0.x..=bounds.1.x).flat_map(|x| [(point(x, bounds.0.y), vector(0,1)), (point(x, bounds.1.y), vector(0,-1))]).chain(
        (bounds.0.y..=bounds.1.y).flat_map(|y| [(point(bounds.0.x, y), vector(1,0)), (point(bounds.1.x, y), vector(-1,0))])
    ).flat_map(|(p, v)| check_visibility(forest, p, v)).collect())
}

fn scenic_score(forest: &BTreeMap<Point, i32>, tree: Point) -> u32 {
    Vector::CARDINAL.iter().map(|dir| viewing_distance(forest, tree, *dir)).product()
}

fn viewing_distance(forest: &BTreeMap<Point, i32>, tree: Point, dir: Vector) -> u32 {
    let height = forest[&tree];
    let mut next_tree = tree;
    let mut dist = 0;
    loop {
        next_tree += dir;
        let next_height = forest.get(&next_tree);
        if next_height.is_none() {
            return dist;
        }
        dist += 1;
        if next_height.is_none() || *next_height.expect("not-none") >= height {
            return dist;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_trees() {
        let forest = parse_input(include_str!("example.txt")).unwrap();
        let visible = find_visible_trees(&forest).unwrap();
        assert_eq!(visible, [
            (0, 0), (0, 1), (0, 2), (0, 3), (0, 4),
            (1, 0), (1, 1), (1, 2),         (1, 4),
            (2, 0), (2, 1), (2, 3),         (2, 4),
            (3, 0),                 (3, 2), (3, 4),
            (4, 0), (4, 1), (4, 2), (4, 3), (4, 4)
        ].iter().map(|&(x,y)| point(x,y)).collect());
    }

    #[test]
    fn scenic_trees() {
        let forest = parse_input(include_str!("example.txt")).unwrap();
        assert_eq!(scenic_score(&forest, point(2,1)), 4);
        assert_eq!(scenic_score(&forest, point(2,3)), 8);
    }
}
