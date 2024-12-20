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

#[derive(Clone, Debug)]
pub struct Cheat {
    track_point: Coords,
    new_start: Coords,
    time_saved: u32,
}

impl Cheat {
    pub fn new(track_point: Coords, new_start: Coords, time_saved: u32) -> Self {
        Cheat {
            track_point,
            new_start,
            time_saved,
        }
    }
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Result<PlotMap> {
    let input = "###############
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

fn identify_possible_cheats(map: &PlotMap, race_track: &[Coords]) -> Result<Vec<Cheat>> {
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let end = find_node(map, &'E')?;
    let uncheated_result = race_track.len() as u32 - 1;

    let cheats = race_track
        .par_iter()
        .enumerate()
        .flat_map(|(current_len, p)| {
            dirs.iter()
                .filter_map(|dout| {
                    let neighbor = (p.0 + dout.0, p.1 + dout.1);
                    let nv = map.get(&neighbor)?;

                    if *nv != '#' {
                        None
                    } else {
                        Some(
                            dirs.iter()
                                .filter_map(|din| {
                                    let new_start = (neighbor.0 + din.0, neighbor.1 + din.1);
                                    let sn = map.get(&new_start)?;

                                    if (*sn == '#') || (new_start.0 == p.0 && new_start.1 == p.1) {
                                        None
                                    } else {
                                        let path = dijkstra(
                                            &new_start,
                                            |n| successors(map, n),
                                            |n| end.0 == n.0 && end.1 == n.1,
                                        )?;

                                        let new_race_length = current_len as u32 + path.1 + 2;
                                        if uncheated_result > new_race_length {
                                            Some(Cheat::new(
                                                *p,
                                                new_start,
                                                uncheated_result - new_race_length,
                                            ))
                                        } else {
                                            None
                                        }
                                    }
                                })
                                .collect::<Vec<_>>(),
                        )
                    }
                })
                .flatten()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(cheats)
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &PlotMap) -> Result<u32> {
    let start = find_node(input, &'S')?;
    let end = find_node(input, &'E')?;
    let path = dijkstra(
        &start,
        |n| successors(input, n),
        |n| end.0 == n.0 && end.1 == n.1,
    )
    .ok_or(GenericError)
    .context("No path found")?;

    let cheats = identify_possible_cheats(input, &path.0)?;
    let mut stats_map = HashMap::new();
    for cheat in &cheats {
        stats_map
            .entry(&cheat.time_saved)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    let result = stats_map
        .iter()
        .map(|(time_saved, number_of_cheats)| {
            if **time_saved >= 100 {
                *number_of_cheats
            } else {
                0
            }
        })
        .sum();

    Ok(result)
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &PlotMap) -> Result<i32> {
    let start = find_node(input, &'S')?;
    let end = find_node(input, &'E')?;
    let path = dijkstra(
        &start,
        |n| successors(input, n),
        |n| end.0 == n.0 && end.1 == n.1,
    )
    .ok_or(GenericError)
    .context("No path found")?;

    println!("{:?}", path);
    println!("len {}", path.0.len());

    let cheats = identify_possible_cheats(input, &path.0)?;
    println!("{:?}", cheats);

    let mut stats_map = HashMap::new();
    for cheat in &cheats {
        stats_map
            .entry(&cheat.time_saved)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    println!("{:?}", stats_map);

    let result = stats_map
        .iter()
        .map(|(time_saved, number_of_cheats)| {
            if **time_saved >= 100 {
                *number_of_cheats
            } else {
                0
            }
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
}
