use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::{astar_bag, dijkstra};

use crate::utils::AocError::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn vec(&self) -> Coords {
        match self {
            North => (-1, 0),
            East => (0, 1),
            South => (1, 0),
            West => (0, -1),
        }
    }
}

use Direction::*;

type Base = i32;
type Coords = (Base, Base);
type Coords3 = (Base, Base, Direction);
type Map = HashMap<Coords, char>;

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Result<Map> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c))
        })
        .collect::<HashMap<_, _>>();

    Ok(map)
}

fn cost(dir1: &Coords, dir2: &Coords) -> u32 {
    if dir1.0 == dir2.0 && dir1.1 == dir2.1 {
        1
    } else {
        1001
    }
}

fn successors(map: &Map, pos: &Coords3) -> Vec<(Coords3, u32)> {
    let dirs = [North, East, South, West];

    dirs.into_iter()
        .filter_map(|p| {
            let delta = p.vec();
            let c = (pos.0 + delta.0, pos.1 + delta.1);
            let v = map.get(&c)?;

            if *v != '#' {
                let m = cost(&pos.2.vec(), &delta);
                Some(((c.0, c.1, p), m))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Map) -> Result<u32> {
    let start = find_node(input, &'S')?;
    let start = (start.0, start.1, East);
    let path = dijkstra(
        &start,
        |n| successors(input, n),
        |n| input.get(&to_coords(n)) == Some(&'E'),
    )
    .ok_or(GenericError)
    .context("No path found")?;

    Ok(path.1)
}

fn distance(a: &Coords, b: &Coords) -> u32 {
    ((b.0 - a.0).abs() + (b.1 - a.1).abs()) as u32
}

fn to_coords(c: &Coords3) -> Coords {
    (c.0, c.1)
}

fn find_node(map: &Map, what: &char) -> Result<Coords> {
    Ok(*map
        .iter()
        .find(|(_, v)| *v == what)
        .ok_or(GenericError)
        .context("No start found")?
        .0)
}

fn find_all_paths(map: &Map) -> Result<usize> {
    let start = find_node(map, &'S')?;
    let end = find_node(map, &'E')?;

    let start = (start.0, start.1, East);
    let all = astar_bag(
        &start,
        |n| successors(map, n),
        |n| distance(&to_coords(n), &end),
        |n| distance(&to_coords(n), &end) == 0,
    )
    .ok_or(GenericError)
    .context("Could not find any paths")?;

    let points = all
        .0
        .flat_map(|v| v.into_iter().map(|p| to_coords(&p)))
        .collect::<HashSet<_>>();

    Ok(points.len())
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Map) -> Result<usize> {
    let all_points = find_all_paths(input)?;

    Ok(all_points)
}

#[cfg(test)]
mod test {
    use super::*;
}
