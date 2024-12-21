use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use memoize::memoize;
use pathfinding::num_traits::signum;

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
fn control_robot_single_recursive(from: char, to: char, level: u32, max_level: u32) -> Option<usize> {
    let to_type = if level == 0 { type_number2(from, to) } else { type_code2(from, to ) };

    if level >= max_level {
        return to_type.into_iter().map(|s| s.len()).min();
    }

    to_type.into_iter().filter_map(|s| {
        vec!['A'].into_iter().chain(s.chars()).collect::<Vec<_>>().windows(2).map(|w| control_robot_single_recursive(w[0], w[1], level + 1, max_level)).sum::<Option<usize>>()
    }).min()
}

fn hash(code: &[char], punch: usize) -> Result<usize> {
    let code = code
        .iter()
        .filter(|v| v.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()?;

    Ok(punch * code)
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> Result<usize> {
    Ok(input.iter().filter_map(|s| {
        let len = ['A'].iter().chain(s).collect::<Vec<_>>().windows(2).filter_map(|w| {
            control_robot_single_recursive(*w[0], *w[1], 0, 2)
        })
        .sum();

        hash(s, len).ok()
    })
    .sum())
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> Result<usize> {
    Ok(input.iter().filter_map(|s| {
        let len = ['A'].iter().chain(s).collect::<Vec<_>>().windows(2).filter_map(|w| {
            control_robot_single_recursive(*w[0], *w[1], 0, 25)
        })
        .sum();

        hash(s, len).ok()
    })
    .sum())
}

#[cfg(test)]
mod test {
    use super::*;
}
