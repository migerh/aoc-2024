use std::collections::HashMap;

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::dijkstra;
use rayon::prelude::*;

use crate::utils::AocError::*;

type Base = i32;
type Coords = (Base, Base);
type Map<T> = HashMap<Coords, T>;
type PlotMap = Map<char>;

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Result<PlotMap> {
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

fn successors(map: &PlotMap, pos: &Coords) -> Vec<(Coords, u32)> {
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    dirs.into_iter()
        .filter_map(|d| {
            let c = (pos.0 + d.0, pos.1 + d.1);
            let v = map.get(&c)?;

            if *v != '#' {
                Some(((c.0, c.1), 1))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn find_node(map: &PlotMap, what: &char) -> Result<Coords> {
    Ok(*map
        .iter()
        .find(|(_, v)| *v == what)
        .ok_or(GenericError)
        .context("No start found")?
        .0)
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &PlotMap) -> Result<usize> {
    let start = find_node(input, &'S')?;
    let end = find_node(input, &'E')?;
    let path = dijkstra(
        &start,
        |n| successors(input, n),
        |n| end.0 == n.0 && end.1 == n.1,
    )
    .ok_or(GenericError)
    .context("No path found")?;

    let result = identify_cheats(input, &path.0, 2)?
        .into_iter()
        .filter(|t| *t >= 100)
        .count();

    Ok(result)
}

fn distance(start: &Coords, end: &Coords) -> u32 {
    ((end.0 - start.0).abs() + (end.1 - start.1).abs()) as u32
}

fn identify_cheats(
    map: &PlotMap,
    race_track: &[Coords],
    max_cheat_time: u32,
) -> Result<Vec<u32>> {
    let uncheated_result = race_track.len() as u32 - 1;
    let remaining_paths = race_track
        .iter()
        .enumerate()
        .map(|(i, &p)| (p, uncheated_result - i as u32))
        .collect::<HashMap<_, _>>();

    let cheats = race_track
        .par_iter()
        .enumerate()
        .flat_map(|(current_len, p)| {
            let new_starts = map
                .iter()
                .filter_map(|(k, c)| {
                    let d = distance(p, k);
                    if *c != '#' && d <= max_cheat_time {
                        Some((k, d))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            new_starts
                .iter()
                .filter_map(|new_start| {
                    let cheated_path_rest = remaining_paths.get(new_start.0)?;

                    let new_race_length = current_len as u32 + cheated_path_rest + new_start.1;
                    if uncheated_result > new_race_length {
                        Some(uncheated_result - new_race_length)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(cheats)
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &PlotMap) -> Result<usize> {
    let start = find_node(input, &'S')?;
    let end = find_node(input, &'E')?;
    let path = dijkstra(
        &start,
        |n| successors(input, n),
        |n| end.0 == n.0 && end.1 == n.1,
    )
    .ok_or(GenericError)
    .context("No path found")?;

    let result = identify_cheats(input, &path.0, 20)?
        .into_iter()
        .filter(|t| *t >= 100)
        .count();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> &'static str {
        "###############
    #...#...#.....#
    #.#.#.#.#.###.#
    #S#...#.#.#...#
    #######.#.#.###
    #######.#.#...#
    #######.#.###.#
    ###..E#...#...#
    ###.#######.###
    #...###...#...#
    #.#####.#.###.#
    #.#...#.#.#...#
    #.#.#.#.#.#.###
    #...#...#...###
    ###############"
    }

    fn real_input() -> &'static str {
        include_str!("../../input/2024/day20.txt")
    }

    #[test]
    fn part2() -> Result<()> {
        let data = input_generator(input())?;
        Ok(assert_eq!(0, solve_part2(&data)?))
    }

    #[test]
    fn part2_real() -> Result<()> {
        let data = input_generator(real_input())?;
        Ok(assert_eq!(1005476, solve_part2(&data)?))
    }
}
