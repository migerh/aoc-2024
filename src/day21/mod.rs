use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use memoize::memoize;
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

#[memoize]
fn type_number(from: char, to: char) -> Vec<String> {
    let from = number_to_coords(from);
    let to = number_to_coords(to);
    let diff = (to.0 - from.0, to.1 - from.1);

    let left_or_right = if signum(diff.0) == -1 { '<' } else { '>' };
    let up_or_down = if signum(diff.1) == -1 { '^' } else { 'v' };

    if from.1 == 3 && to.0 == 0 {
        vec![(0..diff.1.abs())
            .map(|_| up_or_down)
            .chain((0..diff.0.abs()).map(|_| left_or_right))
            .chain(vec!['A'])
            .collect::<String>()]
    } else {
        vec![
            (0..diff.0.abs())
                .map(|_| left_or_right)
                .chain((0..diff.1.abs()).map(|_| up_or_down))
                .chain(vec!['A'])
                .collect::<String>(),
            (0..diff.1.abs())
                .map(|_| up_or_down)
                .chain((0..diff.0.abs()).map(|_| left_or_right))
                .chain(vec!['A'])
                .collect::<String>(),
        ]
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

#[memoize]
fn type_code(from: char, to: char) -> Vec<String> {
    let from = dir_to_coords(from);
    let to = dir_to_coords(to);
    let diff = (to.0 - from.0, to.1 - from.1);

    let left_or_right = if signum(diff.0) == -1 { '<' } else { '>' };
    let up_or_down = if signum(diff.1) == -1 { '^' } else { 'v' };

    if from.1 == 0 && to.0 == 0 {
        vec![(0..diff.1.abs())
            .map(|_| up_or_down)
            .chain((0..diff.0.abs()).map(|_| left_or_right))
            .chain(vec!['A'])
            .collect::<String>()]
    } else {
        vec![
            (0..diff.0.abs())
                .map(|_| left_or_right)
                .chain((0..diff.1.abs()).map(|_| up_or_down))
                .chain(vec!['A'])
                .collect::<String>(),
            (0..diff.1.abs())
                .map(|_| up_or_down)
                .chain((0..diff.0.abs()).map(|_| left_or_right))
                .chain(vec!['A'])
                .collect::<String>(),
        ]
    }
}

fn control_robot(code: &[char]) -> Result<String> {
    let for_numbers = code.iter().fold((vec!["".to_string()], 'A'), |acc, val| {
        let partial = type_number(acc.1, *val);
        (
            acc.0
                .into_iter()
                .flat_map(|prev| partial.iter().map(move |now| format!("{}{}", prev, now)))
                .collect::<Vec<_>>(),
            *val,
        )
    });

    let dir_pad1 = for_numbers
        .0
        .into_iter()
        .flat_map(|s| {
            s.chars()
                .fold((vec!["".to_string()], 'A'), |acc, val| {
                    let partial = type_code(acc.1, val);
                    (
                        acc.0
                            .into_iter()
                            .flat_map(|prev| {
                                partial.iter().map(move |now| format!("{}{}", prev, now))
                            })
                            .collect::<Vec<_>>(),
                        val,
                    )
                })
                .0
        })
        .collect::<Vec<_>>();

    let mut dir_pad2 = dir_pad1
        .into_iter()
        .flat_map(|s| {
            s.chars()
                .fold((vec!["".to_string()], 'A'), |acc, val| {
                    let partial = type_code(acc.1, val);
                    (
                        acc.0
                            .into_iter()
                            .flat_map(|prev| {
                                partial.iter().map(move |now| format!("{}{}", prev, now))
                            })
                            .collect::<Vec<_>>(),
                        val,
                    )
                })
                .0
        })
        .collect::<Vec<_>>();

    dir_pad2.sort_by_key(|a| a.len());

    Ok(dir_pad2
        .first()
        .ok_or(GenericError)
        .context("No path found")?
        .clone())
}

fn control_robot2(code: &[char]) -> Result<String> {
    let punchy = code.iter().fold(("".to_string(), 'A'), |acc, val| {
        let for_numbers = type_number(acc.1, *val);

        let dpad1 = for_numbers
            .into_iter()
            .flat_map(|s| {
                s.chars()
                    .fold((vec!["".to_string()], 'A'), |acc, val| {
                        let partial = type_code(acc.1, val);
                        (
                            acc.0
                                .into_iter()
                                .flat_map(|prev| {
                                    partial.iter().map(move |now| format!("{}{}", prev, now))
                                })
                                .collect::<Vec<_>>(),
                            val,
                        )
                    })
                    .0
            })
            .collect::<Vec<_>>();

        let mut dpad2 = dpad1
            .into_iter()
            .flat_map(|s| {
                s.chars()
                    .fold((vec!["".to_string()], 'A'), |acc, val| {
                        let partial = type_code(acc.1, val);
                        (
                            acc.0
                                .into_iter()
                                .flat_map(|prev| {
                                    partial.iter().map(move |now| format!("{}{}", prev, now))
                                })
                                .collect::<Vec<_>>(),
                            val,
                        )
                    })
                    .0
            })
            .collect::<Vec<_>>();

        dpad2.sort_by_key(|a| a.len());
        let add = dpad2.first().unwrap();
        (format!("{}{}", acc.0, add), *val)
    });

    Ok(punchy.0)
}

#[memoize]
fn type_number2(from: char, to: char) -> Vec<String> {
    let from = number_to_coords(from);
    let to = number_to_coords(to);

    let mut queue = vec![(from, vec![])];
    let mut result = vec![];
    while let Some(q) = queue.pop() {
        let (p, path) = q;

        // the forbidden place
        if p.0 == 0 && p.1 == 3 {
            continue;
        }

        if p.0 == to.0 && p.1 == to.1 {
            let mut fpath = path.clone();
            fpath.push('A');
            result.push(fpath);
            continue;
        }


        let diff = (to.0 - p.0, to.1 - p.1);

        let mut candidates = vec![];
        let sgnx = signum(diff.0);
        if sgnx == -1 {
            let mut np = path.clone();
            np.push('<');
            candidates.push(((p.0 - 1, p.1), np));
        } else if sgnx == 1 {
            let mut np = path.clone();
            np.push('>');
            candidates.push(((p.0 + 1, p.1), np));
        };

        let sgny = signum(diff.1);
        if sgny == -1 {
            let mut np = path.clone();
            np.push('^');
            candidates.push(((p.0, p.1 - 1), np));
        } else if sgny == 1 {
            let mut np = path.clone();
            np.push('v');
            candidates.push(((p.0, p.1 + 1), np));
        };

        queue.append(&mut candidates);
    }

    println!("{:?}", result);

    result.into_iter().map(|v| v.into_iter().collect::<String>()).collect::<Vec<_>>()
}

#[memoize]
fn type_code2(from: char, to: char) -> Vec<String> {
    let from = dir_to_coords(from);
    let to = dir_to_coords(to);

    let mut queue = vec![(from, vec![])];
    let mut result = vec![];
    while let Some(q) = queue.pop() {
        let (p, path) = q;

        // the forbidden place
        if p.0 == 0 && p.1 == 0 {
            continue;
        }

        if p.0 == to.0 && p.1 == to.1 {
            let mut fpath = path.clone();
            fpath.push('A');
            result.push(fpath);
            continue;
        }


        let diff = (to.0 - p.0, to.1 - p.1);

        let mut candidates = vec![];
        let sgnx = signum(diff.0);
        if sgnx == -1 {
            let mut np = path.clone();
            np.push('<');
            candidates.push(((p.0 - 1, p.1), np));
        } else if sgnx == 1 {
            let mut np = path.clone();
            np.push('>');
            candidates.push(((p.0 + 1, p.1), np));
        };

        let sgny = signum(diff.1);
        if sgny == -1 {
            let mut np = path.clone();
            np.push('^');
            candidates.push(((p.0, p.1 - 1), np));
        } else if sgny == 1 {
            let mut np = path.clone();
            np.push('v');
            candidates.push(((p.0, p.1 + 1), np));
        };

        queue.append(&mut candidates);
    }

    result.into_iter().map(|v| v.into_iter().collect::<String>()).collect::<Vec<_>>()
}

#[memoize]
fn control_robot_recursive(code: String, level: u32, max_level: u32) -> Option<usize> {
    if level >= max_level {
        return Some(code.len());
    }

    let to_type = code.chars().fold((vec!["".to_string()], 'A'), |acc, val| {
        let partial = if level == 0 { type_number(acc.1, val) } else { type_code(acc.1, val) };
        let candidates = acc.0
                .into_iter()
                .flat_map(|prev| partial.iter().map(move |now| format!("{}{}", prev, now)))
                .collect::<Vec<_>>();
        let shortest_length = candidates.iter().map(|v| v.len()).min().unwrap_or(0);
        (
            candidates.into_iter().filter(|c| c.len() == shortest_length).collect::<Vec<_>>(),
            val,
        )
    }).0;

    let mut result = to_type.into_iter().map(|c| control_robot_recursive(c, level + 1, max_level)).collect::<Option<Vec<_>>>()?;
    result.sort();

    Some(*result.first()?)
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

#[memoize]
fn control_robot_single_recursive(from: char, to: char, level: u32, max_level: u32) -> Option<usize> {
    let to_type = if level == 0 { type_number2(from, to) } else { type_code2(from, to ) };

    if level >= max_level {
        return to_type.into_iter().map(|s| s.len()).min();
    }

    to_type.into_iter().filter_map(|s| {
        vec!['A'].into_iter().chain(s.chars()).collect::<Vec<_>>().windows(2).map(|w| control_robot_single_recursive(w[0], w[1], level + 1, max_level)).sum::<Option<usize>>()
    }).min()
}

fn hash2(code: &[char], punch: usize) -> Result<usize> {
    let code = code
        .iter()
        .filter(|v| v.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()?;

    println!("{} * {}", punch, code);
    Ok(punch * code)
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> Result<usize> {
    let result = input.iter().filter_map(|s| {
        let len = vec!['A'].iter().chain(s).collect::<Vec<_>>().windows(2).filter_map(|w| {
            control_robot_single_recursive(*w[0], *w[1], 0, 2)
        })
        .sum();

        hash2(s, len).ok()
    })
    .sum();
    Ok(result)
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> Result<usize> {
    let result = input.iter().filter_map(|s| {
        let len = vec!['A'].iter().chain(s).collect::<Vec<_>>().windows(2).filter_map(|w| {
            control_robot_single_recursive(*w[0], *w[1], 0, 25)
        })
        .sum();

        hash2(s, len).ok()
    })
    .sum();
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
}
