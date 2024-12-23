use std::collections::{HashMap, HashSet};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

type Pair = (String, String);

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Result<Vec<Pair>> {
    // let input = "kh-tc
    // qp-kh
    // de-cg
    // ka-co
    // yn-aq
    // qp-ub
    // cg-tb
    // vc-aq
    // tb-ka
    // wh-tc
    // yn-cg
    // kh-ub
    // ta-co
    // de-co
    // tc-td
    // tb-wq
    // wh-td
    // ta-ka
    // td-qp
    // aq-cg
    // wq-ub
    // ub-vc
    // de-ta
    // wq-aq
    // wq-vc
    // wh-yn
    // ka-de
    // kh-ta
    // co-tc
    // wh-qp
    // tb-vc
    // td-yn";

    Ok(input
        .lines()
        .filter_map(|l| {
            let mut s = l.split("-");
            Some((s.next()?.to_string(), s.next()?.to_string()))
        })
        .collect::<Vec<_>>())
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &[Pair]) -> Result<usize> {
    let mut connections = HashMap::new();
    for c in input {
        connections
            .entry(c.0.clone())
            .and_modify(|v: &mut Vec<String>| v.push(c.1.clone()))
            .or_insert(vec![c.1.clone()]);

        connections
            .entry(c.1.clone())
            .and_modify(|v: &mut Vec<String>| v.push(c.0.clone()))
            .or_insert(vec![c.0.clone()]);
    }

    //let mut keys = connections.keys().collect::<Vec<_>>();
    //keys.sort();
    //for k in keys.into_iter() {
    //    println!("{}", k);
    //    let v = connections.get(k).clone().unwrap();
    //    println!("{}", format!("{:?}", v).replace("\"", "'"));
    //}

    let mut thriples = vec![];
    for c in connections.iter() {
        for neighbor in c.1.iter() {
            for second in connections.get(neighbor).unwrap().iter() {
                if second != c.0 && connections.get(second).unwrap().contains(c.0) {
                    thriples.push([c.0.clone(), neighbor.clone(), second.clone()]);
                }
            }
        }
        //let is_triple = if let Some(w) = connections.get(&val[0]) {
        //    w.contains(&val[1]) && w.contains(c.0)
        //} else {
        //    false
        //};

        //if is_triple {
        //    acc.into_iter()
        //        .chain([vec![c.0.clone(), val[0].clone(), val[1].clone()]])
        //        .collect::<Vec<_>>()
        //} else {
        //    acc
        //}
        //});
        //thriples.append(&mut concat);
    }

    let mut condensed = thriples
        .iter()
        .filter(|t| t[0] != t[1] && t[1] != t[2] && t[0] != t[2])
        .map(|t| {
            let mut t = (*t).clone();
            t.sort();
            t.join(",")
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    condensed.sort();
    println!("total thriples: {}", condensed.len());
    for c in condensed {
        println!("{}", c);
    }

    let thriples_with_t = thriples
        .iter()
        .filter(|t| t.iter().any(|l| l.starts_with("t")))
        .collect::<Vec<_>>();

    println!("{} thriples with t", thriples_with_t.len());

    let mut condensed = thriples_with_t
        .iter()
        .filter(|t| t[0] != t[1] && t[1] != t[2] && t[0] != t[2])
        .map(|t| {
            let mut t = (*t).clone();
            t.sort();
            t.join(",")
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    condensed.sort();

    println!("{} condensed list of thriples with t", condensed.len());
    //for t in condensed.iter() {
    //    println!("{}", t);
    //}

    // stuck
    // 958
    // 487

    Ok(condensed.len())
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &[Pair]) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;
}
