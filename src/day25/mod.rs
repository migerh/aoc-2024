use std::str::FromStr;

use anyhow::{Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::AocError::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum LockKey {
    Lock([usize; 5]),
    Key([usize; 5]),
}

impl FromStr for LockKey {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        if grid.len() != 7 || grid.iter().any(|l| l.len() != 5) {
            return Err(GenericError).context("Invalid shape");
        }

        let mut result = [0; 5];
        for x in 0..5 {
            let mut count = 0;
            for y in 0..7 {
                if grid[y][x] == '#' {
                    count += 1;
                }
            }
            result[x] = count - 1;
        }

        if grid[0].iter().all(|c| *c == '#') {
            Ok(LockKey::Lock(result))
        } else {
            Ok(LockKey::Key(result))
        }
    }
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Result<Vec<LockKey>> {
    let _input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    input
        .split("\n\n")
        .map(LockKey::from_str)
        .collect::<Result<Vec<_>>>()
}

#[aoc(day25, part1)]
pub fn solve_part1(input: &[LockKey]) -> Result<usize> {
    println!("{:?}", input);
    let locks = input
        .iter()
        .filter_map(|lk| {
            if let LockKey::Lock(l) = lk {
                Some(l)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let keys = input
        .iter()
        .filter_map(|lk| {
            if let LockKey::Key(k) = lk {
                Some(k)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut count = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if !lock
                .iter()
                .zip(key.iter())
                .map(|(l, k)| l + k)
                .any(|v| v > 5)
            {
                count += 1;
            }
        }
    }
    Ok(count)
}

#[aoc(day25, part2)]
pub fn solve_part2(input: &[LockKey]) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;
}
