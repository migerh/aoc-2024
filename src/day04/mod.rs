use std::collections::HashMap;

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day04)]
pub fn input_generator(input: &str) -> String {
    input.to_string()
}

fn parse_part1(input: &str) -> Result<Vec<String>> {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = map.len();
    let width = map[0].len();

    let mut lines = input.lines().map(|l| l.to_owned()).collect::<Vec<_>>();
    let mut columns = vec![];
    for j in 0..width {
        let mut col = vec![];
        (0..height).for_each(|i| {
            col.push(map[i][j]);
        });
        columns.push(col.into_iter().collect::<String>());
    }

    let mut diagonal1 = vec![];
    for i in 0..height {
        let mut d1 = vec![];
        let mut d2 = vec![];
        for x in 0..=i {
            d1.push(map[i - x][x]);
            if i != height - 1 {
                d2.push(map[height - 1 - x][width - 1 - i + x]);
            }
        }
        diagonal1.push(d1.into_iter().collect::<String>());
        if !d2.is_empty() {
            diagonal1.push(d2.into_iter().collect::<String>());
        }
    }

    let mut diagonal2 = vec![];
    for i in 0..height {
        let mut d1 = vec![];
        let mut d2 = vec![];
        for x in 0..=i {
            d1.push(map[height - 1 - i + x][x]);

            if i != height - 1 {
                d2.push(map[x][width - 1 - i + x]);
            }
        }
        diagonal2.push(d1.into_iter().collect::<String>());
        if !d2.is_empty() {
            diagonal2.push(d2.into_iter().collect::<String>());
        }
    }

    lines.append(&mut columns);
    lines.append(&mut diagonal1);
    lines.append(&mut diagonal2);

    Ok(lines)
}

#[aoc(day04, part1)]
pub fn solve_part1(input: &str) -> Result<usize> {
    let input = parse_part1(input)?;

    Ok(input
        .iter()
        .map(|l| {
            l.chars()
                .collect::<Vec<_>>()
                .windows(4)
                .filter(|w| {
                    w.iter().copied().collect::<String>() == "XMAS"
                        || w.iter().copied().collect::<String>() == "SAMX"
                })
                .count()
        })
        .sum())
}

#[aoc(day04, part2)]
pub fn solve_part2(input: &str) -> Result<i32> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| l.chars().enumerate().map(move |(j, c)| ((i, j), c)))
        .collect::<HashMap<_, _>>();

    Ok(map
        .iter()
        .map(|(p, c)| {
            if *c != 'A' {
                return 0;
            }

            let top_left = map.get(&(p.0.wrapping_sub(1), p.1.wrapping_sub(1)));
            let top_right = map.get(&(p.0.wrapping_sub(1), p.1 + 1));
            let bottom_left = map.get(&(p.0 + 1, p.1.wrapping_sub(1)));
            let bottom_right = map.get(&(p.0 + 1, p.1 + 1));

            match (top_left, bottom_right, top_right, bottom_left) {
                (Some('M'), Some('S'), Some('M'), Some('S')) => 1,
                (Some('M'), Some('S'), Some('S'), Some('M')) => 1,
                (Some('S'), Some('M'), Some('M'), Some('S')) => 1,
                (Some('S'), Some('M'), Some('S'), Some('M')) => 1,
                _ => 0,
            }
        })
        .sum())
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> &'static str {
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
    }

    #[test]
    fn part1() -> Result<()> {
        Ok(assert_eq!(18, solve_part1(input())?))
    }

    #[test]
    fn part2() -> Result<()> {
        Ok(assert_eq!(9, solve_part2(input())?))
    }
}
