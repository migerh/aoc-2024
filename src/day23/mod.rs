use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::AocError::*;

type Pair = (String, String);

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Result<Vec<Pair>> {
    Ok(input
        .lines()
        .filter_map(|l| {
            let mut s = l.split("-");
            Some((s.next()?.to_string(), s.next()?.to_string()))
        })
        .collect::<Vec<_>>())
}

fn build_graph(edges: &[Pair]) -> HashMap<String, Vec<String>> {
    let mut connections = HashMap::new();
    for c in edges {
        connections
            .entry(c.0.clone())
            .and_modify(|v: &mut Vec<String>| v.push(c.1.clone()))
            .or_insert(vec![c.1.clone()]);

        connections
            .entry(c.1.clone())
            .and_modify(|v: &mut Vec<String>| v.push(c.0.clone()))
            .or_insert(vec![c.0.clone()]);
    }
    connections
}
#[aoc(day23, part1)]
pub fn solve_part1(input: &[Pair]) -> Result<usize> {
    let connections = build_graph(input);

    let mut triples = vec![];
    for c in connections.iter() {
        for neighbor in c.1.iter() {
            if let Some(second_list) = connections.get(neighbor) {
                for second in second_list.iter() {
                    if let Some(third_list) = connections.get(second) {
                        if second != c.0 && third_list.contains(c.0) {
                            triples.push([c.0.clone(), neighbor.clone(), second.clone()]);
                        }
                    }
                }
            }
        }
    }

    let triples_with_t = triples
        .iter()
        .filter(|t| t.iter().any(|l| l.starts_with("t")))
        .collect::<Vec<_>>();

    let condensed = triples_with_t
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

    Ok(condensed.len())
}

fn bron_kerbosch(
    r: HashSet<String>,
    p: HashSet<String>,
    x: HashSet<String>,
    neighbors: &impl Fn(String) -> HashSet<String>,
    report: &impl Fn(HashSet<String>) -> (),
) {
    if p.is_empty() && x.is_empty() {
        report(r.clone());
    }

    let mut p = p.clone();
    let mut x = x.clone();
    for v in p.clone() {
        let ns = neighbors(v.clone());
        let mut nr = r.clone();
        nr.insert(v.clone());
        let np = p.intersection(&ns).cloned().collect::<HashSet<_>>();
        let nx = x.intersection(&ns).cloned().collect::<HashSet<_>>();
        bron_kerbosch(nr, np, nx, neighbors, report);
        p.remove(&v);
        x.insert(v);
    }
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &[Pair]) -> Result<String> {
    let connections = build_graph(input);
    let r = HashSet::new();
    let x = HashSet::new();
    let p = connections.keys().cloned().collect::<HashSet<_>>();

    let cliques = Mutex::new(vec![]);
    bron_kerbosch(
        r,
        p,
        x,
        &|v: String| {
            if let Some(n) = connections.get(&v) {
                n.iter().cloned().collect::<HashSet<_>>()
            } else {
                HashSet::new()
            }
        },
        &|c: HashSet<String>| cliques.lock().unwrap().push(c),
    );

    if let Ok(c) = cliques.lock() {
        let mut c = c.clone();
        c.sort_by_key(|v| v.len());
        let mut vertices = c.last().unwrap().iter().cloned().collect::<Vec<_>>();
        vertices.sort();
        let solution = vertices.join(",");

        return Ok(solution.clone());
    }

    Err(GenericError).context("No solution found")
}
