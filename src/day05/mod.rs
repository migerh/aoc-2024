use crate::utils::AocError::*;
use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

type PrintOrder = Vec<u32>;

type PagePair = (u32, u32);
type PageOrders = Vec<PagePair>;

#[aoc_generator(day05)]
pub fn input_generator(input: &str) -> Result<(PageOrders, Vec<PrintOrder>)> {
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
                .context("No rhs")?
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

fn find_first_page_with_wrong_order(pages: &PageOrders, p: &PrintOrder) -> Option<(usize, usize)> {
    for i in 0..(p.len() - 1) {
        let lhs = p[i];
        for j in (i + 1)..p.len() {
            let rhs = p[j];

            let c = check(pages, (lhs, rhs));
            if !c {
                return Some((i, j));
            }
        }
    }

    None
}

fn is_correct(pages: &PageOrders, p: &PrintOrder) -> bool {
    find_first_page_with_wrong_order(pages, p).is_none()
}

#[aoc(day05, part1)]
pub fn solve_part1(input: &(PageOrders, Vec<PrintOrder>)) -> Result<u32> {
    let (pages, prints) = input;

    let sum = prints
        .iter()
        .filter(|p| is_correct(pages, p))
        .map(|v| v[v.len() / 2])
        .sum::<u32>();

    Ok(sum)
}

#[aoc(day05, part2)]
pub fn solve_part2(input: &(PageOrders, Vec<PrintOrder>)) -> Result<u32> {
    let (pages, prints) = input;

    let to_fix = prints
        .iter()
        .filter(|p| !is_correct(pages, p))
        .collect::<Vec<_>>();

    let sum = to_fix.into_iter().map(|prints| {
        let mut p = prints.clone();
        loop {
            let swap = find_first_page_with_wrong_order(pages, &p);
            if let Some((i, j)) = swap {
                p.swap(i, j);
            } else {
                break;
            }
        }

        p[p.len() / 2]
    }).sum();

    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> &'static str {
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
    }

    fn parse() -> Result<(PageOrders, Vec<PrintOrder>)> {
        input_generator(input())
    }

    #[test]
    fn par1() -> Result<()> {
        let data = parse()?;
        Ok(assert_eq!(143, solve_part1(&data)?))
    }

    #[test]
    fn par2() -> Result<()> {
        let data = parse()?;
        Ok(assert_eq!(123, solve_part2(&data)?))
    }
}
