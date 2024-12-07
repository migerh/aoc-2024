use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

use crate::utils::AocError::*;

type Equation = (u128, Vec<u128>);

fn parse_line(s: &str) -> Result<(u128, Vec<u128>)> {
    let mut split = s.split(":");

    let lhs = split
        .next()
        .ok_or(GenericError)
        .context("Could not find lhs")?
        .parse::<u128>()?;
    let rhs = split
        .next()
        .ok_or(GenericError)
        .context("Could not find rhs")?;
    let values = rhs
        .split(" ")
        .filter(|v| !v.is_empty())
        .map(|v| -> Result<u128> { Ok(v.parse::<u128>()?) })
        .collect::<Result<Vec<_>>>()?;

    Ok((lhs, values))
}

#[aoc_generator(day07)]
pub fn input_generator(input: &str) -> Result<Vec<Equation>> {
    input.lines().map(parse_line).collect::<Result<Vec<_>>>()
}

fn plus(n: u128, r: &[u128]) -> Vec<u128> {
    let l = r.len();
    if l == 1 {
        return vec![n + r[0]];
    }

    plus(r[l - 1], &r[..l - 1])
        .into_iter()
        .map(|v| v + n)
        .chain(mul(r[l - 1], &r[..l - 1]).into_iter().map(|v| v + n))
        .collect::<Vec<_>>()
}

fn mul(n: u128, r: &[u128]) -> Vec<u128> {
    let l = r.len();
    if l == 1 {
        return vec![n * r[0]];
    }

    plus(r[l - 1], &r[..l - 1])
        .into_iter()
        .map(|v| v * n)
        .chain(mul(r[l - 1], &r[..l - 1]).into_iter().map(|v| v * n))
        .collect::<Vec<_>>()
}

fn filter(equations: &[Equation]) -> Vec<&Equation> {
    equations
        .par_iter()
        .filter(|e| {
            let collect = plus(0, &e.1);

            collect.contains(&e.0)
        })
        .collect()
}

#[aoc(day07, part1)]
pub fn solve_part1(input: &[Equation]) -> Result<u128> {
    Ok(filter(input).into_iter().map(|e| e.0).sum())
}

fn plus2(n: u128, r: &[u128]) -> Vec<u128> {
    let l = r.len();
    if l == 1 {
        return vec![n + r[0]];
    }

    plus2(r[l - 1], &r[..l - 1])
        .into_iter()
        .map(|v| v + n)
        .chain(mul2(r[l - 1], &r[..l - 1]).into_iter().map(|v| v + n))
        .chain(concat(r[l - 1], &r[..l - 1]).into_iter().map(|v| v + n))
        .collect::<Vec<_>>()
}

fn mul2(n: u128, r: &[u128]) -> Vec<u128> {
    let l = r.len();
    if l == 1 {
        return vec![n * r[0]];
    }

    plus2(r[l - 1], &r[..l - 1])
        .into_iter()
        .map(|v| v * n)
        .chain(mul2(r[l - 1], &r[..l - 1]).into_iter().map(|v| v * n))
        .chain(concat(r[l - 1], &r[..l - 1]).into_iter().map(|v| v * n))
        .collect::<Vec<_>>()
}

fn cc(a: u128, b: u128) -> u128 {
    (a.to_string() + b.to_string().as_str())
        .parse::<u128>()
        .unwrap()
}

fn concat(n: u128, r: &[u128]) -> Vec<u128> {
    let l = r.len();
    if l == 1 {
        return vec![cc(r[0], n)];
    }

    plus2(r[l - 1], &r[..l - 1])
        .into_iter()
        .map(|v| cc(v, n))
        .chain(mul2(r[l - 1], &r[..l - 1]).into_iter().map(|v| cc(v, n)))
        .chain(concat(r[l - 1], &r[..l - 1]).into_iter().map(|v| cc(v, n)))
        .collect::<Vec<_>>()
}

fn filter2(equations: &[Equation]) -> Vec<&Equation> {
    equations
        .par_iter()
        .filter(|e| {
            let collect = plus(0, &e.1);

            !collect.contains(&e.0)
        })
        .collect()
}

#[aoc(day07, part2)]
pub fn solve_part2(input: &[Equation]) -> Result<u128> {
    let sum1 = filter(input).into_iter().map(|e| e.0).sum::<u128>();
    Ok(filter2(input)
        .par_iter()
        .filter(|e| {
            let collect = plus2(0, &e.1);
            collect.contains(&e.0)
        })
        .map(|e| e.0)
        .sum::<u128>() + sum1)
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> &'static str {
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
    }

    #[test]
    fn cc1() {
        assert_eq!(156, cc(15, 6));
    }

    #[test]
    fn cc2() {
        assert_eq!(615, cc(6, 15));
    }
}
