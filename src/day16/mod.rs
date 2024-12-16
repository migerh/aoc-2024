use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::{dijkstra, dijkstra_partial};

use crate::utils::AocError::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West
}

type Base = i32;
type Coords = (Base, Base);
type Coords3 = (Base, Base, Direction);
type Map = HashMap<Coords, char>;

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Result<Map> {
    let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

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

fn succ(map: &Map, dir: &Coords, pos: &Coords) -> Vec<((Coords, Coords), u32)> {
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    dirs.into_iter()
        .filter_map(|p| {
            let c = (pos.0 + p.0, pos.1 + p.1);
            let v = map.get(&c)?;

            if *v != '#' {
                let m = cost(dir, &p);
                Some(((c, p), m))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn size(map: &Map) -> Option<(i32, i32)> {
    let width = map.iter().map(|(c, _)| c.1).max()?;
    let height = map.iter().map(|(c, _)| c.0).max()?;

    Some((height, width))
}

fn print_map_and_path(map: &Map, path: &Vec<(Coords, Coords)>) -> Option<()> {
    let (height, width) = size(map)?;
    let points = path.iter().map(|v| v.0).collect::<HashSet<_>>();

    for y in 0..=height {
        for x in 0..=width {
            let c = if points.contains(&(y, x)) {
                'O'
            } else if let Some(c) = map.get(&(y, x)) {
                *c
            } else {
                '.'
            };

            print!("{}", c);
        }
        println!();
    }

    Some(())
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Map) -> Result<u32> {
    let start = input
        .iter()
        .find(|(_, v)| **v == 'S')
        .ok_or(GenericError)
        .context("No start found")?
        .0;
    let path = dijkstra(&(*start, (0, 1)), |&n: &((i32, i32), (i32, i32))| {
        succ(input, &n.1, &n.0)
    }, |&n| input.get(&n.0) == Some(&'E'))
    .ok_or(GenericError)
    .context("No path found")?;

    println!("{:?}", path);
    print_map_and_path(input, &path.0).ok_or(GenericError).context("Could not print map")?;

    Ok(path.1)
}

fn min_score(input: &Map) -> Result<u32> {
    let start = input
        .iter()
        .find(|(_, v)| **v == 'S')
        .ok_or(GenericError)
        .context("No start found")?
        .0;
    let path = dijkstra(&(*start, (0, 1)), |&n: &((i32, i32), (i32, i32))| {
        succ(input, &n.1, &n.0)
    }, |&n| input.get(&n.0) == Some(&'E'))
    .ok_or(GenericError)
    .context("No path found")?;

    Ok(path.1)
}

fn distance(a: &Coords, b: &Coords) -> u32 {
    ((b.0 - a.0).abs() + (b.1 - a.1).abs()) as u32
}

fn find_all_paths(map: &Map, start: &Coords, dir: &Coords) -> Result<Vec<Vec<Coords>>> {
    let min = min_score(map)?;
    println!("min score {}", min);
    let end = map.iter().find(|p| p.1 == &'E').ok_or(GenericError).context("Could not find end")?.0;

    let mut queue = vec![(((*start, *dir), 0_u32), vec![(*start, 0_u32)])];
    let mut all_paths = vec![];

    while let Some(n) = queue.pop() {
        let (((point, dir), score), path) = n;
        let current_score = path.last().ok_or(GenericError).context("Empty path")?.1;
        if current_score + distance(&point, end) > min {
            continue;
        }

        let next = succ(map, &dir, &point);
            //.into_iter()
            //.map(|v| v.0)
            //.collect::<Vec<_>>();

        let mut collect = next
            .into_iter()
            .map(|p| {
                let mut foo = path.clone();
                foo.push((p.0.0, p.1 + current_score));
                (p, foo)
            })
            .collect::<Vec<_>>();

        let mut new_paths = collect.iter().map(|p| p.1.clone()).collect::<Vec<_>>();
        all_paths.append(&mut new_paths);

        queue.append(&mut collect);
    }

    println!("{:?}", all_paths.len());

    Ok(all_paths
        .iter()
        .filter_map(|p| {
            let f = p.first()?;
            let l = p.last()?;

            let fv = map.get(&f.0)?;
            let lv = map.get(&l.0)?;
            let s = l.1;

            if *fv == 'S' && *lv == 'E' && s == min {
                let path = p.iter().map(|v| v.0).collect::<Vec<_>>();
                Some(path)
            } else {
                None
            }
        }).collect::<Vec<_>>())
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Map) -> Result<usize> {
    let start = input
        .iter()
        .find(|(_, v)| **v == 'S')
        .ok_or(GenericError)
        .context("No start found")?
        .0;

    let all_paths = find_all_paths(input, start, &(0, 1))?;
    let all_points = all_paths.into_iter().flat_map(|path| path.into_iter().collect::<Vec<_>>()).collect::<HashSet<_>>();

    println!("{:?}", all_points.len());

    Ok(all_points.len())
}

#[cfg(test)]
mod test {
    use super::*;
}
