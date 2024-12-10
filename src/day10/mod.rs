use std::collections::HashMap;

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::dijkstra_all;

type Base = i32;
type Coords = (Base, Base);
type Map = HashMap<Coords, char>;

#[aoc_generator(day10)]
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

fn successors(map: &Map, node: &Coords) -> Vec<(Coords, usize)> {
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let node_value = map.get(node);

    dirs.into_iter()
        .filter_map(|d| {
            let h = node_value?.to_digit(10)?;
            let c = (node.0 + d.0, node.1 + d.1);
            let v = map.get(&c)?;

            if v.to_digit(10)? == 1 + h {
                Some((c, 1))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Map) -> Result<u32> {
    let map = input;
    let sum = map
        .iter()
        .filter(|(_, v)| **v == '0')
        .map(|(c, _)| c)
        .map(|head| dijkstra_all(head, |n| successors(map, n)))
        .map(|p| {
            p.iter()
                .filter_map(|(k, _)| if map.get(k)? == &'9' { Some(1) } else { None })
                .sum::<u32>()
        })
        .sum::<u32>();

    Ok(sum)
}

fn find_all_paths(map: &Map, start: &Coords) -> usize {
    let mut queue = vec![(*start, vec![*start])];
    let mut all_paths = vec![];

    while let Some(n) = queue.pop() {
        let (point, path) = n;
        let next = successors(map, &point)
            .into_iter()
            .map(|v| v.0)
            .collect::<Vec<_>>();

        let mut collect = next
            .into_iter()
            .map(|p| {
                let mut foo = path.clone();
                foo.push(p);
                (p, foo)
            })
            .collect::<Vec<_>>();

        let mut new_paths = collect.iter().map(|p| p.1.clone()).collect::<Vec<_>>();
        all_paths.append(&mut new_paths);

        queue.append(&mut collect);
    }

    let number_unique_paths = all_paths
        .iter()
        .filter_map(|p| {
            let f = p.first()?;
            let l = p.last()?;

            let fv = map.get(f)?;
            let lv = map.get(l)?;

            if *fv == '0' && *lv == '9' {
                Some(1)
            } else {
                None
            }
        })
        .sum();

    number_unique_paths
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Map) -> Result<usize> {
    let paths = input
        .iter()
        .filter(|(_, v)| **v == '0')
        .map(|(c, _)| c)
        .map(|head| find_all_paths(input, head))
        .collect::<Vec<_>>();

    Ok(paths.iter().sum::<usize>())
}

#[cfg(test)]
mod test {
    use super::*;

    fn input1() -> &'static str {
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
    }

    fn input2() -> &'static str {
        "0123
1234
8765
9876"
    }

    #[test]
    fn part1_input1() -> Result<()> {
        let data = input_generator(input1())?;
        Ok(assert_eq!(36, solve_part1(&data)?))
    }

    #[test]
    fn part1_input2() -> Result<()> {
        let data = input_generator(input2())?;
        Ok(assert_eq!(1, solve_part1(&data)?))
    }

    #[test]
    fn part2_input1() -> Result<()> {
        let data = input_generator(input1())?;
        Ok(assert_eq!(81, solve_part2(&data)?))
    }

    #[test]
    fn part2_input2() -> Result<()> {
        let data = input_generator(input2())?;
        Ok(assert_eq!(16, solve_part2(&data)?))
    }
}
