use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::num_traits::signum;

use crate::utils::AocError::*;

type Coords = (i32, i32);

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<char>>> {
//    let input = "029A
//980A
//179A
//456A
//379A";

    Ok(input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>())
}

fn number_to_coords(n: char) -> Coords {
    match n {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => unimplemented!(),
    }
}

fn type_number(from: char, to: char) -> String {
    let from = number_to_coords(from);
    let to = number_to_coords(to);
    let diff = (to.0 - from.0, to.1 - from.1);

    let left_or_right = if signum(diff.0) == -1 { '<' } else { '>' };
    let up_or_down = if signum(diff.1) == -1 { '^' } else { 'v' };

    if from.1 == 3 {
        (0..diff.1.abs())
            .map(|_| up_or_down)
            .chain((0..diff.0.abs()).map(|_| left_or_right))
            .chain(vec!['A'])
            .collect::<String>()
    } else {
        (0..diff.0.abs())
            .map(|_| left_or_right)
            .chain((0..diff.1.abs()).map(|_| up_or_down))
            .chain(vec!['A'])
            .collect::<String>()
    }
}

fn dir_to_coords(n: char) -> Coords {
    match n {
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => unimplemented!(),
    }
}

fn type_code(from: char, to: char) -> String {
    let from = dir_to_coords(from);
    let to = dir_to_coords(to);
    let diff = (to.0 - from.0, to.1 - from.1);

    let left_or_right = if signum(diff.0) == -1 { '<' } else { '>' };
    let up_or_down = if signum(diff.1) == -1 { '^' } else { 'v' };

    if from.1 == 0 {
        (0..diff.1.abs())
            .map(|_| up_or_down)
            .chain((0..diff.0.abs()).map(|_| left_or_right))
            .chain(vec!['A'])
            .collect::<String>()
    } else {
        (0..diff.0.abs())
            .map(|_| left_or_right)
            .chain((0..diff.1.abs()).map(|_| up_or_down))
            .chain(vec!['A'])
            .collect::<String>()
    }
}

fn control_robot(code: &[char]) -> String {
    let for_numbers = code.iter().fold(("".to_string(), 'A'), |acc, val| {
        let partial = type_number(acc.1, *val);
        (format!("{}{}", acc.0, partial), *val)
    });

    println!("{:?} -> {}", code, for_numbers.0);

    let dir_pad1 = for_numbers
        .0
        .chars()
        .fold(("".to_string(), 'A'), |acc, val| {
            let partial = type_code(acc.1, val);
            (format!("{}{}", acc.0, partial), val)
        });

    println!("{} -> {}", for_numbers.0, dir_pad1.0);

    let dir_pad2 = dir_pad1.0.chars().fold(("".to_string(), 'A'), |acc, val| {
        let partial = type_code(acc.1, val);
        (format!("{}{}", acc.0, partial), val)
    });

    println!("{} -> {}", dir_pad1.0, dir_pad2.0);

    dir_pad2.0
}

fn hash(code: &[char], punch: String) -> Result<usize> {
    let code = code
        .iter()
        .filter(|v| v.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()?;

    println!("{} * {}", punch.len(), code);
    Ok(punch.len() * code)
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> Result<usize> {
    // too high
    // 168492

    Ok(input
        .iter()
        .filter_map(|c| hash(c, control_robot(c)).ok())
        .inspect(|c| println!("{}\n", c))
        .sum())
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;
}
