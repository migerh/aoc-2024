use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::AocError::*;

type Base = i32;
type Coords = (i32, i32);
type Robot = (Coords, Coords);

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Result<Vec<Robot>> {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"p=(.*),(.*) v=(.*),(.*)").unwrap();
    }

    let matches = RE
        .captures_iter(input)
        .map(|c| -> Option<Robot> {
            let get = |idx| c.get(idx)?.as_str().parse::<Base>().ok();

            let ax = get(1)?;
            let ay = get(2)?;
            let bx = get(3)?;
            let by = get(4)?;

            Some(((ax, ay), (bx, by)))
        })
        .collect::<Option<Vec<_>>>()
        .ok_or(GenericError)?;

    Ok(matches)
}

fn simulate(grid: Coords, robot: &Robot, times: Base) -> Coords {
    (
        (robot.0 .0 + robot.1 .0 * times).rem_euclid(grid.0),
        (robot.0 .1 + robot.1 .1 * times).rem_euclid(grid.1),
    )
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &[Robot]) -> Result<Base> {
    let size = if input.len() == 12 {
        (11, 7)
    } else {
        (101, 103)
    };
    let hash = input
        .iter()
        .map(|m| simulate(size, m, 100))
        .fold((0, 0, 0, 0), |acc, m| match m {
            (x, y) if x < size.0 / 2 && y < size.1 / 2 => (acc.0 + 1, acc.1, acc.2, acc.3),
            (x, y) if x > size.0 / 2 && y < size.1 / 2 => (acc.0, acc.1 + 1, acc.2, acc.3),
            (x, y) if x < size.0 / 2 && y > size.1 / 2 => (acc.0, acc.1, acc.2 + 1, acc.3),
            (x, y) if x > size.0 / 2 && y > size.1 / 2 => (acc.0, acc.1, acc.2, acc.3 + 1),
            _ => acc,
        });
    Ok(hash.0 * hash.1 * hash.2 * hash.3)
}

fn score(size: Coords, m: &[Robot]) -> i32 {
    let mut score = 0;

    for x in 0..=size.0 {
        for y in 0..=size.1 {
            let on_this_tile = m.iter().filter(|m| m.0 .0 == x && m.0 .1 == y).count();
            score += if on_this_tile > 1 { 1 } else { 0 };
        }
    }

    score
}

fn print_robots(size: Coords, m: &[Robot]) {
    for x in 0..=size.0 {
        for y in 0..=size.1 {
            let on_this_tile = m.iter().filter(|m| m.0 .0 == x && m.0 .1 == y).count();
            let c = if on_this_tile > 0 { 'x' } else { ' ' };
            print!("{}", c);
        }
        println!();
    }
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &[Robot]) -> Result<Base> {
    let size = (101, 103);
    let machines = input
        .iter()
        .map(|m| (simulate(size, m, 8053), m.1))
        .collect::<Vec<_>>();
    println!("Score: {}", score(size, &machines));
    print_robots(size, &machines);

    Ok(8053)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simulate1() {
        assert_eq!((6, 5), simulate((11, 7), &((2, 4), (2, -3)), 2))
    }

    fn input() -> &'static str {
        "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
    }

    #[test]
    fn part1() -> Result<()> {
        let data = input_generator(input())?;
        Ok(assert_eq!(12, solve_part1(&data)?))
    }
}
