use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    ops::{BitAnd, BitXor, Shl},
};

use crate::utils::AocError::*;

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Result<Vec<u128>> {
    input
        .lines()
        .map(|v| Ok(v.parse::<u128>()?))
        .collect::<Result<Vec<_>>>()
}

fn next(mut number: u128) -> u128 {
    number = number.bitxor(number.shl(6) as u128).bitand(0xFFFFFF);
    number = number.bitxor(number >> 5 as u128).bitand(0xFFFFFF);
    number = number.bitxor(number.shl(11) as u128).bitand(0xFFFFFF);
    number
}

fn hash(number: u128, rounds: usize) -> u128 {
    (0..rounds).fold(number, |acc, _| next(acc))
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &[u128]) -> Result<u128> {
    let result = input.par_iter().map(|v| hash(*v, 2000)).sum();
    Ok(result)
}

fn build_table(seeds: &[u128]) -> HashMap<(i8, i8, i8, i8), isize> {
    let mut table = HashMap::new();

    for seed in seeds {
        let mut v = vec![*seed; 2000];
        for i in 1..v.len() {
            v[i] = next(v[i - 1]);
        }

        let v = v.into_iter().map(|v| (v % 10) as i8).collect::<Vec<_>>();
        let mut buyer_table = HashMap::new();
        v.windows(5).for_each(|w| {
            let diff = (w[1] - w[0], w[2] - w[1], w[3] - w[2], w[4] - w[3]);
            buyer_table.entry(diff).or_insert(w[4] as isize);
        });

        for (k, v) in buyer_table {
            table.entry(k).and_modify(|w| *w += v).or_insert(v);
        }
    }

    table
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &[u128]) -> Result<isize> {
    let table = build_table(&input);

    let result = table
        .values()
        .max()
        .ok_or(GenericError)
        .context("Could not find max value")?;
    Ok(*result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let input = [1, 10, 100, 2024];
        Ok(assert_eq!(37327623, solve_part1(&input)?))
    }

    #[test]
    fn part2() -> Result<()> {
        let input = [1, 2, 3, 2024];
        Ok(assert_eq!(23, solve_part2(&input)?))
    }
}
