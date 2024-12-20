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
    //    let input = "###############
    //#...#...#.....#
    //#.#.#.#.#.###.#
    //#S#...#.#.#...#
    //#######.#.#.###
    //#######.#.#...#
    //#######.#.###.#
    //###..E#...#...#
    //###.#######.###
    //#...###...#...#
    //#.#####.#.###.#
    //#.#...#.#.#...#
    //#.#.#.#.#.#.###
    //#...#...#...###
    //###############";

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

fn identify_possible_cheats(map: &PlotMap, race_track: &[Coords]) -> Result<Vec<u32>> {
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
                                            Some(uncheated_result - new_race_length)
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

    let result = identify_possible_cheats(input, &path.0)?
    // Using part 2 identifier algorithm works, too, but takes 3 times as long
    // let result = identify_cheats_part2(input, &path.0, 2)?
        .into_iter()
        .filter(|t| *t >= 100)
        .count();

    Ok(result)
}

fn distance(start: &Coords, end: &Coords) -> u32 {
    ((end.0 - start.0).abs() + (end.1 - start.1).abs()) as u32
}

fn get_shortest_path_length_cached(
    cache: &mut HashMap<Coords, u32>,
    map: &PlotMap,
    start: &Coords,
    end: &Coords,
) -> Option<u32> {
    if cache.contains_key(start) {
        return cache.get(start).copied();
    }

    let path = dijkstra(
        start,
        |n| successors(map, n),
        |n| end.0 == n.0 && end.1 == n.1,
    )?;
    cache.entry(*start).or_insert(path.1);

    Some(path.1)
}

fn identify_cheats_part2(
    map: &PlotMap,
    race_track: &[Coords],
    max_cheat_time: u32,
) -> Result<Vec<u32>> {
    let end = find_node(map, &'E')?;
    let uncheated_result = race_track.len() as u32 - 1;
    let mut cache = HashMap::new();

    let cheats = race_track
        .iter()
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
                    let cheated_path_rest =
                        get_shortest_path_length_cached(&mut cache, map, new_start.0, &end)?;

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

    let result = identify_cheats_part2(input, &path.0, 20)?
        .into_iter()
        .filter(|t| *t >= 100)
        .count();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
}
