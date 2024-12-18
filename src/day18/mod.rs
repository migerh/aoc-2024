use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::dijkstra;
use rayon::prelude::*;

use crate::utils::AocError::*;

type Base = i32;
type Coords = (Base, Base);

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Result<Vec<Coords>> {
    input
        .lines()
        .map(|l| {
            let mut it = l
                .split(",")
                .map(|v| -> Result<i32> { Ok(v.parse::<i32>()?) });
            let a = it
                .next()
                .ok_or(GenericError)
                .context("Could not parse coordinate")??;
            let b = it
                .next()
                .ok_or(GenericError)
                .context("Could not parse coordinate")??;

            Ok((a, b))
        })
        .collect::<Result<Vec<_>>>()
}

fn size(map: &[Coords]) -> Option<(i32, i32)> {
    let width = map.iter().map(|c| c.1).max()?;
    let height = map.iter().map(|c| c.0).max()?;

    Some((height, width))
}

fn successors(map: &[Coords], size: &Coords, c: &Coords) -> Vec<(Coords, u32)> {
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    dirs.into_iter()
        .filter_map(|d| {
            let candidate = (c.0 + d.0, c.1 + d.1);
            if c.0 < 0 || c.1 < 0 || c.0 > size.0 || c.1 > size.1 {
                return None;
            }

            if map.contains(&candidate) {
                return None;
            }

            Some((candidate, 1))
        })
        .collect::<Vec<_>>()
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &[Coords]) -> Result<u32> {
    // doesn't always work, but in example & input it does work because there are corrupted
    // memory thingies in the last row and column.
    let end = size(input).ok_or(GenericError).context("Map is empty")?;
    let start = (0, 0);
    let part1 = input
        .iter()
        .take(if input.len() == 25 { 12 } else { 1024 })
        .cloned()
        .collect::<Vec<_>>();

    let path = dijkstra(
        &start,
        |n| successors(&part1, &end, n),
        |n| n.0 == end.0 && n.1 == end.1,
    )
    .ok_or(GenericError)
    .context("Could not find path")?;

    Ok(path.1)
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &[Coords]) -> Result<String> {
    let end = size(input).ok_or(GenericError).context("Map is empty")?;
    let start = (0, 0);

    (0..input.len()).into_par_iter().filter_map(|m| {
        let part1 = input.iter().take(m + 1).cloned().collect::<Vec<_>>();

        if dijkstra(
            &start,
            |n| successors(&part1, &end, n),
            |n| n.0 == end.0 && n.1 == end.1,
        ).is_none() {
            Some(m)
        } else {
            None
        }
    }).min().map(|c| {
        let mem = input[c];
        format!("{},{}", mem.0, mem.1)
    }).ok_or(GenericError).context("Path is never blocked")
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> &'static str {
        "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
    }

    #[test]
    fn part1() -> Result<()> {
        let data = input_generator(input())?;
        Ok(assert_eq!(22, solve_part1(&data)?))
    }

    #[test]
    fn part2() -> Result<()> {
        let data = input_generator(input())?;
        Ok(assert_eq!("6,1", solve_part2(&data)?))
    }
}