use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::Result;
use crate::utils::AocError::*;

pub enum Instruction {
    Do,
    Dont,
    Mul((i32, i32)),
}

#[aoc_generator(day03)]
pub fn input_generator(input: &str) -> Result<Vec<Instruction>> {
    lazy_static! {
        static ref RE: regex::Regex =
            regex::Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap();
    }

    let matches = RE
        .captures_iter(input)
        .map(|c| -> Option<Instruction> {
            let mat = c.get(1)?.as_str();
            Some(if mat.contains("do()") {
                Instruction::Do
            } else if mat.contains("don't()") {
                Instruction::Dont
            } else {
                Instruction::Mul((c.get(2)?.as_str().parse::<i32>().ok()?, c.get(3)?.as_str().parse::<i32>().ok()?))
            })
        })
        .collect::<Option<Vec<_>>>().ok_or(GenericError)?;

    Ok(matches)
}

#[aoc(day03, part1)]
pub fn solve_part1(input: &[Instruction]) -> Result<i32> {
    Ok(input.iter().fold(0, |acc, v| {
        match v {
            Instruction::Mul((x, y)) => acc + x * y,
            _ => acc
        }
    }))
}

#[aoc(day03, part2)]
pub fn solve_part2(input: &[Instruction]) -> Result<i32> {
    Ok(input.iter().fold((0, 1), |acc, v| 
        match v {
            Instruction::Do => (acc.0, 1),
            Instruction::Dont => (acc.0, 0),
            Instruction::Mul((x, y)) => (acc.0 + acc.1 * x * y, acc.1),
        }
    ).0)
}

#[cfg(test)]
mod test {
    use super::*;

    fn input1() -> &'static str {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    }

    fn input2() -> &'static str {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    }

    fn parse(input: &str) -> Result<Vec<Instruction>> {
        input_generator(input)
    }

    #[test]
    fn part1() -> Result<()> {
        let data = parse(input1())?;
        Ok(assert_eq!(161, solve_part1(&data)?))
    }

    #[test]
    fn part2() -> Result<()> {
        let data = parse(input2())?;
        Ok(assert_eq!(48, solve_part2(&data)?))
    }
}