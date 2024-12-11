use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use memoize::memoize;
use rayon::prelude::*;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Result<Vec<u128>> {
    input
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| Ok(s.parse::<u128>()?))
        .collect::<Result<Vec<_>>>()
}

fn ilen(v: u128) -> u32 {
    v.ilog10() + 1
}

fn split(v: u128) -> (u128, u128) {
    let len = ilen(v) / 2;
    let factor = 10u128.pow(len);
    let left = v / factor;
    let right = v % factor;

    (left, right)
}

fn blink(stones: &[u128]) -> Vec<u128> {
    stones
        .iter()
        .flat_map(|&v| match v {
            0 => vec![1],
            x if ilen(x) % 2 == 0 => {
                let (left, right) = split(x);
                vec![left, right]
            }
            x => vec![x * 2024],
        })
        .collect::<Vec<_>>()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[u128]) -> Result<usize> {
    let mut stones = input.to_vec();
    for _ in 0..25 {
        stones = blink(&stones);
    }
    Ok(stones.len())
}

#[memoize]
fn blink_fast(stone: u128, lvl: u32, max: u32) -> u128 {
    if lvl == max {
        return 1;
    }

    match stone {
        0 => blink_fast(1, lvl + 1, max),
        x if ilen(x) % 2 == 0 => {
            let (left, right) = split(x);
            blink_fast(left, lvl + 1, max) + blink_fast(right, lvl + 1, max)
        }
        x => blink_fast(x * 2024, lvl + 1, max),
    }
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[u128]) -> Result<u128> {
    let sum = input
        .par_iter()
        .map(|s| blink_fast(*s, 0, 75))
        .sum::<u128>();

    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> &'static str {
        "125 17"
    }

    #[test]
    fn part1() -> Result<()> {
        let data = input_generator(input())?;
        Ok(assert_eq!(55312, solve_part1(&data)?))
    }

    #[test]
    fn part2() -> Result<()> {
        let data = input_generator(input())?;
        Ok(assert_eq!(65601038650482, solve_part2(&data)?))
    }
}
