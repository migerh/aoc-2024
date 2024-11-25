use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::Result;

#[aoc_generator(dayXX)]
pub fn input_generator(input: &str) -> Result<Vec<i32>> {
    Ok(vec![])
}

#[aoc(dayXX, part1)]
pub fn solve_part1(input: &[i32]) -> Result<i32> {
    Ok(0)
}

#[aoc(dayXX, part2)]
pub fn solve_part2(input: &[i32]) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;
}