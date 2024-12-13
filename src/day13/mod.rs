use std::str::FromStr;

use anyhow::{Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

use crate::utils::AocError::*;

type Coords = (i128, i128);

#[derive(Debug, Clone)]
pub struct Machine {
    a: Coords,
    b: Coords,
    prize: Coords,
}

impl Machine {
    pub fn new(a: Coords, b: Coords, prize: Coords) -> Self {
        Self { a, b, prize }
    }

    pub fn tokens(&self) -> i128 {
        let a = self.a;
        let b = self.b;
        let p = self.prize;

        let disc = a.0 * b.1 - b.0 * a.1;

        let fa = p.0 * b.1 - p.1 * b.0;
        let fb = p.1 * a.0 - p.0 * a.1;

        if fa % disc != 0 || fb % disc != 0 {
            return 0;
        }

        let ta = fa / disc;
        let tb = fb / disc;

        3 * ta + tb
    }
}

impl FromStr for Machine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(
                r"^Button A: X(.*), Y(.*)\nButton B: X(.*), Y(.*)\nPrize: X=(.*), Y=(.*)$"
            )
            .unwrap();
        }

        let matches = RE
            .captures_iter(s)
            .map(|c| -> Option<Machine> {
                let get = |idx| c.get(idx)?.as_str().parse::<i128>().ok();

                let ax = get(1)?;
                let ay = get(2)?;
                let bx = get(3)?;
                let by = get(4)?;
                let px = get(5)?;
                let py = get(6)?;

                Some(Machine::new((ax, ay), (bx, by), (px, py)))
            })
            .collect::<Option<Vec<_>>>()
            .ok_or(GenericError)?;

        Ok(matches
            .first()
            .ok_or(GenericError)
            .context("Could not parse")?
            .clone())
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Result<Vec<Machine>> {
    input
        .split("\n\n")
        .map(Machine::from_str)
        .collect::<Result<Vec<_>>>()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &[Machine]) -> Result<i128> {
    Ok(input.par_iter().map(|m| m.tokens()).sum())
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &[Machine]) -> Result<i128> {
    Ok(input
        .par_iter()
        .map(|m| {
            Machine::new(
                m.a,
                m.b,
                (m.prize.0 + 10000000000000, m.prize.1 + 10000000000000),
            )
        })
        .map(|m| m.tokens())
        .sum::<i128>())
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> &'static str {
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
    }

    #[test]
    fn part1() -> Result<()> {
        let data = input_generator(input())?;
        Ok(assert_eq!(480, solve_part1(&data)?))
    }

    #[test]
    fn part2() -> Result<()> {
        let data = input_generator(input())?;
        Ok(assert_eq!(875318608908, solve_part2(&data)?))
    }
}
