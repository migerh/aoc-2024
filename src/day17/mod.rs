use std::ops::BitXor;

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::utils::AocError::*;

type Base = u64;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub enum Operand {
    Literal(Base),
    A,
    B,
    C,
}

use Operand::*;

impl Operand {
    fn from_char(c: char) -> Result<Operand> {
        Ok(match c {
            '0' => Literal(0),
            '1' => Literal(1),
            '2' => Literal(2),
            '3' => Literal(3),
            '4' => A,
            '5' => B,
            '6' => C,
            _ => Err(GenericError).context("Could not parse operand")?,
        })
    }

    fn value(&self, intcode: &IntCode) -> Base {
        match self {
            Literal(a) => *a,
            A => intcode.a,
            B => intcode.b,
            C => intcode.c,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntCode {
    a: Base,
    b: Base,
    c: Base,
    output: Vec<Base>,
    ip: usize,
    program: Vec<(char, char)>,
}

impl IntCode {
    pub fn new(a: Base, b: Base, c: Base, program: &str) -> Result<Self> {
        let program = program
            .split(",")
            .filter_map(|c| c.chars().next())
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|v| {
                if v.len() != 2 {
                    Err(GenericError).context("Could not parse program: too short")
                } else {
                    Ok((v[0], v[1]))
                }
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(IntCode {
            a,
            b,
            c,
            output: vec![],
            ip: 0,
            program,
        })
    }

    pub fn run(&mut self) -> Result<Vec<Base>> {
        let len = self.program.len();
        while self.ip < len {
            let (instruction, operand) = self.program[self.ip];

            match instruction {
                // Adv
                '0' => {
                    let op_value = Operand::from_char(operand)?.value(self);
                    self.a /= 2_u64.pow(op_value as u32);
                }
                '1' => {
                    self.b = self
                        .b
                        .bitxor(operand.to_digit(10).ok_or(GenericError)? as u64);
                }
                '2' => {
                    let op_value = Operand::from_char(operand)?.value(self);
                    self.b = op_value % 8;
                }
                '3' => {
                    let op_value = operand.to_digit(10).ok_or(GenericError)?;
                    if self.a != 0 {
                        self.ip = op_value as usize;
                        continue;
                    }
                }
                '4' => {
                    self.b = self.b.bitxor(self.c);
                }
                '5' => {
                    let op_value = Operand::from_char(operand)?.value(self);
                    self.output.push(op_value % 8);
                }
                '6' => {
                    let op_value = Operand::from_char(operand)?.value(self);
                    self.b = self.a / 2_u64.pow(op_value as u32);
                }
                '7' => {
                    let op_value = Operand::from_char(operand)?.value(self);
                    self.c = self.a / 2_u64.pow(op_value as u32);
                }
                _ => unimplemented!(),
            }

            self.ip += 1;
        }

        Ok(self.output.clone())
    }
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Option<IntCode> {
    let mut lines = input.lines();
    let a = lines.next()?.split(":").nth(1)?.trim().parse::<u64>().ok()?;
    let b = lines.next()?.split(":").nth(1)?.trim().parse::<u64>().ok()?;
    let c = lines.next()?.split(":").nth(1)?.trim().parse::<u64>().ok()?;

    let program = lines.nth(1)?.split(":").nth(1)?.trim().to_string();

    IntCode::new(a, b, c, &program).ok()
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &IntCode) -> Result<String> {
    let mut intcode = input.clone();
    let out = intcode.run()?;
    Ok(out.iter().map(|v| v.to_string()).join(","))
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &IntCode) -> Result<u64> {
    let needle = input.program.iter().flat_map(|v| vec![v.0, v.1]).join(",");
    let needle_split = needle
        .split(",")
        .map(|v| Ok(v.parse::<u64>()?))
        .collect::<Result<Vec<_>>>()?;
    let mut queue: Vec<u64> = vec![0];
    let mut solutions = vec![];
    let search = [0, 1, 2, 3, 4, 5, 6, 7];

    while let Some(q) = queue.pop() {
        for s in search {
            let a = q * 8 + s;
            let mut intcode = IntCode::new(a, 0, 0, &needle)?;
            let out = intcode.run()?;

            if out.len() > needle_split.len() {
                continue;
            }

            let result = out.iter().map(|v| v.to_string()).join(",");
            if result == needle {
                solutions.push(a);
            }

            let mut matches = true;
            for i in 0..out.len() {
                if out[out.len() - 1 - i] != needle_split[needle_split.len() - i - 1] {
                    matches = false;
                    break;
                }
            }

            if matches {
                queue.push(q * 8 + s);
            }
        }
    }

    let min = solutions
        .into_iter()
        .min()
        .ok_or(GenericError)
        .context("Could not find min value")?;

    Ok(min)
}

#[cfg(test)]
mod test {
    use super::*;

    fn input1() -> &'static str {
        "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
    }

    #[test]
    fn part1() -> Result<()> {
        let data = input_generator(input1()).ok_or(GenericError)?;
        Ok(assert_eq!("4,6,3,5,6,3,5,2,1,0", solve_part1(&data)?))
    }
}
