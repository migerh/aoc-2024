use std::collections::{HashMap, HashSet};

use crate::utils::AocError::*;
use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

type Coords = (i32, i32);

#[aoc_generator(day06)]
pub fn input_generator(input: &str) -> Result<HashMap<Coords, char>> {
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

fn find_start(map: &HashMap<Coords, char>) -> Result<Coords> {
    map.iter()
        .find_map(|v| match v.1 {
            '^' => Some(*v.0),
            _ => None,
        })
        .ok_or(GenericError)
        .context("could not find starting position")
}

fn pos_is_on_map(map: &HashMap<Coords, char>, pos: &Coords) -> bool {
    map.get(pos).is_some()
}

fn generate_dirs() -> Vec<Coords> {
    vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
}

fn find_path(map: &HashMap<Coords, char>) -> Result<Vec<Coords>> {
    let mut dirs = generate_dirs().into_iter().cycle();
    let mut pos = find_start(map)?;
    let mut dir = dirs
        .next()
        .ok_or(GenericError)
        .context("Ran out of directions")?;
    let mut path = Vec::new();

    while pos_is_on_map(map, &pos) {
        path.push(pos);

        let x = pos.0 + dir.0;
        let y = pos.1 + dir.1;

        if let Some(c) = map.get(&(x, y)) {
            if *c == '#' {
                dir = dirs
                    .next()
                    .ok_or(GenericError)
                    .context("Ran out of directions")?;
            }
        }

        pos.0 += dir.0;
        pos.1 += dir.1;
    }

    Ok(path)
}

fn find_visited(map: &HashMap<Coords, char>) -> Result<HashSet<Coords>> {
    Ok(find_path(map)?.into_iter().collect::<HashSet<_>>())
}

#[aoc(day06, part1)]
pub fn solve_part1(input: &HashMap<Coords, char>) -> Result<usize> {
    Ok(find_visited(input)?.len())
}

fn has_loop(map: &HashMap<Coords, char>, start: Coords) -> Result<bool> {
    let mut visited = HashSet::new();
    let mut pos = start;
    let mut dirs = generate_dirs().into_iter().cycle();
    let mut dir = dirs.next().ok_or(GenericError).context("No more dirs")?;

    loop {
        visited.insert((pos, dir));

        // dude...
        loop {
            let x = pos.0 + dir.0;
            let y = pos.1 + dir.1;

            if let Some(c) = map.get(&(x, y)) {
                if *c == '#' {
                    dir = dirs
                        .next()
                        .ok_or(GenericError)
                        .context("Ran our of directions")?;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        pos.0 += dir.0;
        pos.1 += dir.1;

        if visited.contains(&(pos, dir)) {
            return Ok(true);
        }

        if pos_is_on_map(map, &pos) {
            visited.insert((pos, dir));
        } else {
            return Ok(false);
        }
    }
}

#[aoc(day06, part2)]
pub fn solve_part2(input: &HashMap<Coords, char>) -> Result<i32> {
    let start = find_start(input)?;

    let count = find_visited(input)?
        .par_iter()
        .map(|&(i, j)| -> Result<i32> {
            let mut map = input.clone();
            map.entry((i, j)).and_modify(|v| *v = '#');
            let looping_louie = has_loop(&map, start)?;
            Ok(if looping_louie { 1 } else { 0 })
        })
        .sum::<Result<i32>>()?;

    Ok(count)
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> &'static str {
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
    }

    #[test]
    fn part1() -> Result<()> {
        let data = input_generator(input())?;
        Ok(assert_eq!(41, solve_part1(&data)?))
    }

    #[test]
    fn part2() -> Result<()> {
        let data = input_generator(input())?;
        Ok(assert_eq!(6, solve_part2(&data)?))
    }
}
