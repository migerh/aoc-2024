use std::collections::HashMap;

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::AocError::*;

type Input = (Vec<String>, Vec<String>);

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Result<Input> {
    let mut split = input.split("\n\n");
    let patterns = split
        .next()
        .ok_or(GenericError)
        .context("Could not find patterns")?;
    let designs = split
        .next()
        .ok_or(GenericError)
        .context("Could not find designs")?;

    let patterns = patterns
        .split(", ")
        .map(|v| v.to_string())
        .collect::<Vec<_>>();
    let designs = designs.lines().map(|v| v.to_string()).collect::<Vec<_>>();

    Ok((patterns, designs))
}

fn find_next(patterns: &[String], design: &str, start: usize) -> Vec<String> {
    let design = design.chars().skip(start).collect::<Vec<_>>();

    patterns
        .iter()
        .filter_map(|p| {
            let len = p.len();
            let substr = design.iter().take(len).collect::<String>();

            if *p == substr {
                Some(p.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn find_validity(cache: &mut HashMap<String, bool>, patterns: Vec<String>, design: String) -> bool {
    if cache.contains_key(&design) {
        return *cache.get(&design).unwrap();
    }

    if design.is_empty() {
        return true;
    }

    let next = find_next(&patterns, &design, 0).into_iter().any(|n| {
        let offset = n.len();
        let next = design.chars().skip(offset).collect::<String>();
        let result = find_validity(cache, patterns.clone(), next.to_string());

        if !result && next.is_empty() {
            return true;
        }

        result
    });

    cache.entry(design).or_insert(next);

    next
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &Input) -> Result<usize> {
    let (patterns, designs) = input;
    let mut cache = HashMap::new();
    let result = designs
        .iter()
        .filter(|d| find_validity(&mut cache, patterns.clone(), d.to_string()))
        .collect::<Vec<_>>();

    Ok(result.len())
}

fn find_number(cache: &mut HashMap<String, usize>, patterns: Vec<String>, design: String) -> usize {
    if cache.contains_key(&design) {
        return *cache.get(&design).unwrap();
    }

    if design.is_empty() {
        return 0;
    }

    let next = find_next(&patterns, &design, 0)
        .into_iter()
        .map(|n| {
            let offset = n.len();
            let next = design.chars().skip(offset).collect::<String>();
            let result = find_number(cache, patterns.clone(), next.to_string());

            if result == 0 && next.is_empty() {
                return 1;
            }

            result
        })
        .sum();

    cache.entry(design).or_insert(next);

    next
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &Input) -> Result<usize> {
    let (patterns, designs) = input;
    let mut cache = HashMap::new();
    let result = designs
        .iter()
        .map(|d| find_number(&mut cache, patterns.clone(), d.to_string()))
        .sum::<usize>();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> &'static str {
        "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
    }

    #[test]
    fn part1() -> Result<()> {
        let data = input_generator(input())?;
        Ok(assert_eq!(6, solve_part1(&data)?))
    }

    #[test]
    fn part2() -> Result<()> {
        let data = input_generator(input())?;
        Ok(assert_eq!(16, solve_part2(&data)?))
    }
}
