use std::collections::{HashMap, HashSet};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

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

fn build_graph_str(edges: &[Pair]) -> HashMap<&str, HashSet<&str>> {
    let mut connections = HashMap::new();
    for c in edges {
        connections
            .entry(c.0.as_str())
            .and_modify(|v: &mut HashSet<&str>| {
                v.insert(&c.1);
            })
            .or_insert(HashSet::from([c.1.as_str()]));

        connections
            .entry(c.1.as_str())
            .and_modify(|v: &mut HashSet<&str>| {
                v.insert(&c.0);
            })
            .or_insert(HashSet::from([c.0.as_str()]));
    }
    connections
}

fn bron_kerbosch<'a>(
    r: HashSet<&'a str>,
    p: HashSet<&'a str>,
    x: HashSet<&'a str>,
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    report: &mut Vec<HashSet<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        report.push(r.clone());
    }

    let mut p = p.clone();
    let mut x = x.clone();
    for v in p.clone() {
        let ns = if let Some(n) = graph.get(&v) {
            n
        } else {
            &HashSet::new()
        };
        let mut nr = r.clone();
        nr.insert(v);
        let np = p.intersection(&ns).cloned().collect::<HashSet<_>>();
        let nx = x.intersection(&ns).cloned().collect::<HashSet<_>>();
        bron_kerbosch(nr, np, nx, graph, report);
        p.remove(&v);
        x.insert(v);
    }
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &[Pair]) -> Result<String> {
    let connections = build_graph_str(input);
    let r = HashSet::new();
    let x = HashSet::new();
    let p = connections.keys().cloned().collect::<HashSet<_>>();

    let mut cliques = vec![];
    bron_kerbosch(r, p, x, &connections, &mut cliques);

    let mut c = cliques.clone();
    c.sort_by_key(|v| v.len());
    let mut vertices = c.last().unwrap().iter().cloned().collect::<Vec<_>>();
    vertices.sort();
    let solution = vertices.join(",");

    Ok(solution.clone())
}
