use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

use crate::utils::AocError::*;

type Num = u64;
type Equation = (Num, Vec<Num>);

fn parse_line(s: &str) -> Result<(Num, Vec<Num>)> {
    let mut split = s.split(":");

    let lhs = split
        .next()
        .ok_or(GenericError)
        .context("Could not find lhs")?
        .parse::<Num>()?;
    let rhs = split
        .next()
        .ok_or(GenericError)
        .context("Could not find rhs")?;
    let values = rhs
        .split(" ")
        .filter(|v| !v.is_empty())
        .map(|v| -> Result<Num> { Ok(v.parse::<Num>()?) })
        .collect::<Result<Vec<_>>>()?;

    Ok((lhs, values))
}

#[aoc_generator(day07)]
pub fn input_generator(input: &str) -> Result<Vec<Equation>> {
    input.lines().map(parse_line).collect::<Result<Vec<_>>>()
}

fn plus(n: Num, r: &[Num]) -> Vec<Num> {
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

fn mul(n: Num, r: &[Num]) -> Vec<Num> {
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
pub fn solve_part1(input: &[Equation]) -> Result<Num> {
    Ok(filter(input).into_iter().map(|e| e.0).sum())
}

fn plus2(n: Num, r: &[Num]) -> Vec<Num> {
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

fn mul2(n: Num, r: &[Num]) -> Vec<Num> {
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

fn cc(a: Num, b: Num) -> Num {
    let digits = b.ilog10() + 1;
    let factor = 10u64.pow(digits);
    a * factor + b
}

fn concat(n: Num, r: &[Num]) -> Vec<Num> {
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
pub fn solve_part2(input: &[Equation]) -> Result<Num> {
    let sum1 = filter(input).into_iter().map(|e| e.0).sum::<Num>();
    Ok(filter2(input)
        .par_iter()
        .filter(|e| {
            let collect = plus2(0, &e.1);
            collect.contains(&e.0)
        })
        .map(|e| e.0)
        .sum::<Num>() + sum1)
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

    #[test]
    fn cc_many() {
        for i in 0..1000 {
            for j in 1..1000 {
                let correct = i * 10u64.pow(j.to_string().len() as u32) + j;
                assert_eq!(correct, cc(i, j));
            }
        }
    }

    #[test]
    fn part1() -> Result<()> {
        let data = input_generator(input())?;
        Ok(assert_eq!(3749, solve_part1(&data)?))
    }

    #[test]
    fn part2() -> Result<()> {
        let data = input_generator(input())?;
        Ok(assert_eq!(11387, solve_part2(&data)?))
    }
}
