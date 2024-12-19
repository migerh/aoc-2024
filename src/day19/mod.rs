use std::{collections::HashMap, sync::{Arc, Mutex}};

use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Result, Context};
use rayon::prelude::*;
use memoize::memoize;

use crate::utils::AocError::*;

type Input = (Vec<String>, Vec<String>);

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Result<Input> {
//    let input = "r, wr, b, g, bwu, rb, gb, br
//
//brwrr
//bggr
//gbbr
//rrbgbr
//ubwu
//bwurrg
//brgr
//bbrgwb";

    let mut split = input.split("\n\n");
    let patterns = split.next().ok_or(GenericError).context("Could not find patterns")?;
    let designs = split.next().ok_or(GenericError).context("Could not find designs")?;

    let patterns = patterns.split(", ").map(|v| v.to_string()).collect::<Vec<_>>();
    let designs = designs.lines().map(|v| v.to_string()).collect::<Vec<_>>();

    Ok((patterns, designs))
}

fn find_next(patterns: &[String], design: &str, start: usize) -> Vec<String> {
    let design = design.chars().skip(start).collect::<Vec<_>>();

    patterns.iter().filter_map(|p| {
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

fn total_length(s: &[String]) -> usize {
    s.iter().map(|v| v.len()).sum::<usize>()
}

fn is_valid(cache: &mut HashMap<String, Vec<String>>, patterns: &[String], design: &str) -> bool {
    let mut queue = find_next(patterns, design, 0).into_iter().map(|v| vec![v]).collect::<Vec<_>>();
    let mut solutions = vec![];

    while let Some(q) = queue.pop() {
        let start = total_length(&q);

        if start > design.len() {
            continue;
        }

        if start == design.len() {
            solutions.push(q);
            return true;
        }

        let mut next = find_next(patterns, design, start).into_iter().map(|v| {
            let mut n = q.clone();
            n.push(v);
            n
        })
        .filter(|v| total_length(v) <= design.len())
        .collect::<Vec<_>>();

        queue.append(&mut next);
    }

    !solutions.is_empty()
}

fn find_solutions(cache: &mut HashMap<String, Vec<String>>, patterns: Vec<String>, design: String) -> Vec<String> {
    // println!("design {}", design);

    if cache.contains_key(&design) {
        return cache.get(&design).unwrap().clone();
    }

    if design.is_empty() {
        return vec![];
    }

    let result = find_next(&patterns, &design, 0).into_iter().filter(|prev| prev.len() <= design.len()).flat_map(|prev| {
        // println!("intermediate {}", prev);
        let offset = prev.len();
        let next = design.chars().skip(offset).collect::<String>();
        let result = find_solutions(cache, patterns.clone(), next.to_string()).iter().map(|post| {
            prev.clone() + post
        }).collect::<Vec<_>>();

        if result.is_empty() && next.is_empty() {
            return vec![prev];
        }

        result
    }).collect::<Vec<_>>();

    cache.entry(design).or_insert(result.clone());

    // println!("found {:?}", result);

    result
}

fn find_validity(cache: &mut HashMap<String, bool>, patterns: Vec<String>, design: String) -> bool {
    if cache.contains_key(&design) {
        return cache.get(&design).unwrap().clone();
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
    println!("# patterns {}, # designs: {}", patterns.len(), designs.len());

    let mut cache = HashMap::new();

    // let result = designs.iter().filter(|d| is_valid(&mut cache, patterns, d)).count();
    let result = designs.iter().enumerate().filter(|d| {
        let result = find_validity(&mut cache, patterns.clone(), d.1.to_string());

        println!("# {}, result {}", d.0, result);

        result
    }).collect::<Vec<_>>();
    println!("{:?}", result);
    Ok(result.len())
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &Input) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;
}