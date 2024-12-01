use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use crate::utils::AocError::*;

fn parse_line(s: &str) -> Result<(i32, i32)> {
    let mut cs = s
        .split(" ")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .into_iter();

    match (cs.next(), cs.next()) {
        (Some(i), Some(j)) => Ok((i.parse::<i32>()?, j.parse::<i32>()?)),
        _ => Err(GenericError).context("No numbers"),
    }
}

#[aoc_generator(day01)]
pub fn input_generator(input: &str) -> Result<Vec<(i32, i32)>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(parse_line)
        .collect::<Result<Vec<(i32, i32)>>>()
        .context("Error while parsing input")
}

#[aoc(day01, part1)]
pub fn solve_part1(input: &[(i32, i32)]) -> Result<i32> {
    let mut first = input.iter().map(|tup| tup.0).collect::<Vec<_>>();
    let mut second = input.iter().map(|tup| tup.1).collect::<Vec<_>>();
    first.sort();
    second.sort();

    Ok(first.into_iter().zip(second).map(|(i, j)| (j-i).abs()).sum())
}

#[aoc(day01, part2)]
pub fn solve_part2(input: &[(i32, i32)]) -> Result<i32> {
    let second = input.iter().map(|tup| tup.1).collect::<Vec<_>>();

    Ok(input.iter().map(|tup| tup.0).fold(0, |sum, v| {
        let needle = v;
        let factor = second.iter().filter(|s| **s == needle).count() as i32;

        sum + needle * factor
    }))
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "3   4
4   3
2   5
1   3
3   9
3   3"
    }

    fn input() -> Result<Vec<(i32, i32)>> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(11, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(31, solve_part2(&data)?))
    }
}
