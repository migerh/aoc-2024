use std::collections::{HashMap, HashSet};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Base = i32;
type Coords = (Base, Base);
type Map = HashMap<Coords, char>;

#[aoc_generator(day08)]
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

fn is_on_map(map: &Map, coords: &Coords) -> bool {
    map.get(coords).is_some()
}

#[aoc(day08, part1)]
pub fn solve_part1(input: &Map) -> Result<usize> {
    let frequencies = input
        .iter()
        .filter(|&v| *v.1 != '.')
        .map(|v| v.1)
        .collect::<HashSet<_>>();

    let sum = frequencies
        .into_iter()
        .flat_map(|freq| {
            let positions = input.iter().filter(|&a| a.1 == freq).map(|p| p.0);

            positions
                .combinations(2)
                .flat_map(|c| {
                    if c.len() < 2 {
                        return vec![];
                    }

                    // a = (3, 4)
                    // b = (1, 2)

                    // x = (5, 6)
                    // y = (-1, 0)

                    let a = c[0];
                    let b = c[1];

                    let diff = (a.0 - b.0, a.1 - b.1);
                    let x = (a.0 + diff.0, a.1 + diff.1);
                    let y = (b.0 - diff.0, b.1 - diff.1);

                    vec![x, y]
                        .into_iter()
                        .filter(|e| is_on_map(input, e))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>()
        .len();

    Ok(sum)
}

#[aoc(day08, part2)]
pub fn solve_part2(input: &Map) -> Result<usize> {
    let antennas = input.iter().filter(|&v| *v.1 != '.').collect::<Vec<_>>();
    let frequencies = antennas.iter().map(|v| v.1).collect::<HashSet<_>>();
    let antenna_positions = antennas.into_iter().map(|v| *v.0);

    let sum = frequencies
        .into_iter()
        .flat_map(|freq| {
            let positions = input.iter().filter(|&a| a.1 == freq).map(|p| p.0);

            positions
                .combinations(2)
                .flat_map(|c| {
                    if c.len() < 2 {
                        return vec![];
                    }

                    let a = c[0];
                    let b = c[1];

                    let diff = (a.0 - b.0, a.1 - b.1);

                    let mut antinodes = vec![];
                    let mut p = (a.0 + diff.0, a.1 + diff.1);
                    while is_on_map(input, &p) {
                        antinodes.push(p);
                        p = (p.0 + diff.0, p.1 + diff.1);
                    }

                    let mut q = (b.0 - diff.0, b.1 - diff.1);
                    while is_on_map(input, &q) {
                        antinodes.push(q);
                        q = (q.0 - diff.0, q.1 - diff.1);
                    }

                    antinodes
                })
                .collect::<Vec<_>>()
        })
        .chain(antenna_positions)
        .collect::<HashSet<_>>()
        .len();

    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    fn example() -> &'static str {
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
    }

    fn example1() -> &'static str {
        "..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
.........."
    }

    fn example2() -> &'static str {
        "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."
    }

    #[test]
    fn part1_example() -> Result<()> {
        let data = input_generator(example())?;
        Ok(assert_eq!(14, solve_part1(&data)?))
    }

    #[test]
    fn part1_example1() -> Result<()> {
        let data = input_generator(example1())?;
        Ok(assert_eq!(4, solve_part1(&data)?))
    }

    #[test]
    fn part2_example() -> Result<()> {
        let data = input_generator(example())?;
        Ok(assert_eq!(34, solve_part2(&data)?))
    }

    #[test]
    fn part1_example2() -> Result<()> {
        let data = input_generator(example2())?;
        Ok(assert_eq!(9, solve_part2(&data)?))
    }
}
