use crate::utils::AocError::*;
use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

type PrintOrder = Vec<u32>;

type PagePair = (u32, u32);
type PageOrders = Vec<PagePair>;

#[aoc_generator(day05)]
pub fn input_generator(input: &str) -> Result<(PageOrders, Vec<PrintOrder>)> {
//     let input = "47|53
// 97|13
// 97|61
// 97|47
// 75|29
// 61|13
// 75|53
// 29|13
// 97|29
// 53|29
// 61|53
// 97|53
// 61|29
// 47|13
// 75|47
// 97|75
// 47|61
// 75|61
// 47|29
// 75|13
// 53|13
// 
// 75,47,61,53,29
// 97,61,53,29,13
// 75,29,13
// 75,97,47,61,53
// 61,13,29
// 97,13,75,29,47";
    let mut basic_split = input.split("\n\n");
    let page_orders = basic_split
        .next()
        .ok_or(GenericError)
        .context("No page orders")?;
    let print_orders = basic_split
        .next()
        .ok_or(GenericError)
        .context("No print orders")?;

    let page_orders = page_orders
        .lines()
        .map(|l| {
            let mut s = l.split("|");
            let lhs = s
                .next()
                .ok_or(GenericError)
                .context("No lhs")?
                .parse::<u32>()?;
            let rhs = s
                .next()
                .ok_or(GenericError)
                .context("No lhs")?
                .parse::<u32>()?;

            Ok((lhs, rhs))
        })
        .collect::<Result<Vec<_>>>()?;

    let print_orders = print_orders
        .lines()
        .map(|l| {
            l.split(",")
                .map(|v| Ok(v.parse::<u32>()?))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    Ok((page_orders, print_orders))
}

fn check(orders: &PageOrders, pages: PagePair) -> bool {
    orders
        .iter()
        .filter(|o| o.1 == pages.0 && o.0 == pages.1)
        .count()
        == 0
}

#[aoc(day05, part1)]
pub fn solve_part1(input: &(PageOrders, Vec<PrintOrder>)) -> Result<u32> {
    let (pages, prints) = input;

    let sum = prints
        .iter()
        .filter(|p| {
            for i in 0..(p.len() - 1) {
                let lhs = p[i];
                for j in (i + 1)..p.len() {
                    let rhs = p[j];

                    let c = check(pages, (lhs, rhs));
                    if !c {
                        return false;
                    }
                }
            }

            true
        })
        .map(|v| v[v.len() / 2])
        .sum::<u32>();

    Ok(sum)
}

fn is_correct(pages: &PageOrders, p: &PrintOrder) -> bool {
    for i in 0..(p.len() - 1) {
        let lhs = p[i];
        for j in (i + 1)..p.len() {
            let rhs = p[j];

            let c = check(pages, (lhs, rhs));
            if !c {
                return false;
            }
        }
    }

    true
}

#[aoc(day05, part2)]
pub fn solve_part2(input: &(PageOrders, Vec<PrintOrder>)) -> Result<u32> {
    let (pages, prints) = input;

    let to_fix = prints
        .iter()
        .filter(|p| {
            for i in 0..(p.len() - 1) {
                let lhs = p[i];
                for j in (i + 1)..p.len() {
                    let rhs = p[j];

                    let c = check(pages, (lhs, rhs));
                    if !c {
                        return true;
                    }
                }
            }

            false
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for prints in to_fix {
        let mut p = prints.clone();
        let mut is_correct = false;
        'outer: while !is_correct {
            for i in 0..(p.len() - 1) {
                let lhs = p[i];
                for j in (i + 1)..p.len() {
                    let rhs = p[j];

                    let c = check(pages, (lhs, rhs));
                    if !c {
                        p.swap(i, j);
                        continue 'outer;
                    }
                }
            }

            is_correct = true;
        }

        sum += p[p.len() / 2];
    }

    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;
}
