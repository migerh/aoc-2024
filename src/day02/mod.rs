use crate::utils::{input::read_by_line, AocError::*};
use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Safety {
    Safe,
    Unsafe,
}

fn parse_line(line: &str) -> Result<Vec<i32>> {
    line.split(" ")
        .map(|v| Ok(v.parse::<i32>()?))
        .collect::<Result<Vec<_>>>()
}

#[aoc_generator(day02)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<i32>>> {
    read_by_line(input, parse_line)
}

fn determine_report_safety(report: &[i32]) -> Result<Safety> {
    let gradient = report.windows(2).map(|w| w[0] - w[1]).collect::<Vec<_>>();
    let first = gradient.first().ok_or(GenericError)?.signum();
    let monotonous = gradient.iter().all(|v| v.signum() == first);

    if !monotonous {
        return Ok(Safety::Unsafe);
    }

    let within_margins = gradient.iter().all(|v| v.abs() > 0 && v.abs() < 4);

    if within_margins {
        return Ok(Safety::Safe);
    }

    Ok(Safety::Unsafe)
}

#[aoc(day02, part1)]
pub fn solve_part1(input: &[Vec<i32>]) -> Result<usize> {
    Ok(input
        .iter()
        .filter_map(|r| determine_report_safety(r).ok())
        .filter(|s| *s == Safety::Safe)
        .count())
}

fn determine_report_safety_with_dampener(report: &[i32]) -> Result<Safety> {
    for i in 0..report.len() {
        let mut modified = report.to_vec();
        modified.remove(i);
        let safety = determine_report_safety(&modified)?;

        if safety == Safety::Safe {
            return Ok(Safety::Safe);
        }
    }

    Ok(Safety::Unsafe)
}

#[aoc(day02, part2)]
pub fn solve_part2(input: &[Vec<i32>]) -> Result<usize> {
    Ok(input
        .iter()
        .filter_map(|r| determine_report_safety_with_dampener(r).ok())
        .filter(|s| *s == Safety::Safe)
        .count())
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> &'static str {
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
    }

    fn parse() -> Result<Vec<Vec<i32>>> {
        input_generator(input())
    }

    #[test]
    fn part1() -> Result<()> {
        let data = parse()?;
        Ok(assert_eq!(2, solve_part1(&data)?))
    }

    #[test]
    fn part2() -> Result<()> {
        let data = parse()?;
        Ok(assert_eq!(4, solve_part2(&data)?))
    }
}
