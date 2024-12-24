use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufWriter, Write},
    ops::{BitOr, BitXor},
};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::AocError::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Op {
    AND,
    XOR,
    OR,
}

#[derive(Debug, Clone)]
pub struct Expression {
    op: Op,
    left: String,
    right: String,
    result: String,
    value: Option<u8>,
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Result<(HashMap<String, u8>, Vec<Expression>)> {
    lazy_static! {
        static ref RE: regex::Regex =
            regex::Regex::new(r"(.*?) (AND|OR|XOR) (.*?) -> (.*)").unwrap();
    }

    let mut parts = input.split("\n\n");

    let initials = parts
        .next()
        .ok_or(GenericError)
        .context("No initials")?
        .lines()
        .filter_map(|l| {
            let mut split = l.split(": ");
            Some((split.next()?.to_string(), split.next()?.parse::<u8>().ok()?))
        })
        .collect::<HashMap<_, _>>();

    let instructions = parts
        .next()
        .ok_or(GenericError)
        .context("No instructions")?;

    let instructions = RE
        .captures_iter(instructions)
        .filter_map(|c| -> Option<Expression> {
            let left = c.get(1)?.as_str().to_string();
            let op = c.get(2)?.as_str();
            let right = c.get(3)?.as_str().to_string();
            let result = c.get(4)?.as_str().to_string();

            let op = match op {
                "AND" => Op::AND,
                "OR" => Op::OR,
                "XOR" => Op::XOR,
                _ => return None,
            };
            Some(Expression {
                left,
                op,
                right,
                result,
                value: None,
            })
        })
        .collect::<Vec<_>>();

    Ok((initials, instructions))
}

fn all_solved(exp: &[Expression]) -> bool {
    exp.iter().all(|e| e.value.is_some())
}

fn eval(values: &HashMap<String, u8>, exp: &Expression) -> Option<u8> {
    if exp.value.is_some() {
        return exp.value;
    }

    let left = values.get(&exp.left)?;
    let right = values.get(&exp.right)?;
    let value = match exp.op {
        Op::AND => left & right,
        Op::OR => left.bitor(right),
        Op::XOR => left.bitxor(right),
    };

    Some(value)
}

fn get_value(values: &HashMap<String, u8>, what: &str) -> Result<u128> {
    let mut result = values
        .iter()
        .filter(|v| v.0.starts_with(what))
        .map(|v| (v.0, v.1))
        .collect::<Vec<_>>();
    result.sort_by_key(|f| f.0);
    let result = result
        .into_iter()
        .map(|v| v.1.to_string())
        .rev()
        .collect::<Vec<String>>()
        .join("");

    Ok(u128::from_str_radix(&result, 2)?)
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &(HashMap<String, u8>, Vec<Expression>)) -> Result<u128> {
    let (mut values, mut expressions) = input.clone();

    while !all_solved(&expressions) {
        for i in 0..expressions.len() {
            if expressions[i].value.is_some() {
                continue;
            }

            if let Some(r) = eval(&values, &expressions[i]) {
                expressions[i].value = Some(r);
                values.entry(expressions[i].result.clone()).or_insert(r);
            }
        }
    }

    let result = get_value(&values, "z")?;

    Ok(result)
}

fn print_graph(
    expressions: &[Expression],
    involved_expressions: &HashSet<usize>,
    diff_z: &HashSet<String>,
    file_name: &str,
) {
    let f = File::create(file_name).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    writeln!(&mut f, "graph {{").expect("Write failed");
    for (i, e) in expressions.iter().enumerate() {
        if involved_expressions.contains(&i) {
            writeln!(
                &mut f,
                "{:?} -- {:?} [color=red, label=\"{:?}\"]",
                e.left, e.result, e.op
            )
            .unwrap();
            writeln!(
                &mut f,
                "{:?} -- {:?} [color=red, label=\"{:?}\"]",
                e.right, e.result, e.op
            )
            .unwrap();
        } else {
            writeln!(
                &mut f,
                "{:?} -- {:?} [label=\"{:?}\"]",
                e.left, e.result, e.op
            )
            .unwrap();
            writeln!(
                &mut f,
                "{:?} -- {:?} [label=\"{:?}\"]",
                e.right, e.result, e.op
            )
            .unwrap();
        }
    }

    for n in diff_z {
        writeln!(&mut f, "{:?} [style=filled, fillcolor=red]", n).unwrap();
    }
    writeln!(&mut f, "}}").unwrap();
}

fn build_subgraph(expressions: &Vec<Expression>, what: u32) -> Vec<Expression> {
    let mut vertices = HashSet::new();
    vertices.insert(format!("x{:02}", what));
    vertices.insert(format!("y{:02}", what));
    vertices.insert(format!("z{:02}", what));

    for e in expressions {
        let mut temp = HashSet::new();
        temp.insert(e.left.clone());
        temp.insert(e.right.clone());
        temp.insert(e.result.clone());

        if vertices.intersection(&temp).count() != 0 {
            vertices = vertices.union(&temp).cloned().collect::<HashSet<_>>();
        }
    }

    expressions
        .iter()
        .filter(|e| {
            vertices.contains(&e.left)
                || vertices.contains(&e.right)
                || vertices.contains(&e.result)
        })
        .cloned()
        .collect::<Vec<_>>()
}

#[aoc(day24, part2)]
pub fn solve_part2(input: &(HashMap<String, u8>, Vec<Expression>)) -> Result<String> {
    let (values, expressions) = input.clone();

    let x = get_value(&values, "x")?;
    let y = get_value(&values, "y")?;
    let z = solve_part1(input)?;

    // println!("{} + {} = {}", x, y, z);
    // println!("{:b}", x);
    // println!("{:b}", y);
    // println!("expected: {:b}", x + y);
    // println!("  actual: {:b}", z);

    let mut diff_bits = (x + y).bitxor(z);

    // println!("    diff: {:#046b}", diff_bits);

    let mut involved_values = vec![];
    let mut c = 0;
    while diff_bits != 0 {
        if diff_bits & 1 == 1 {
            involved_values.push(c);
        }

        diff_bits = diff_bits >> 1;
        c += 1;
    }

    let mut size = 0;
    let mut involved_expressions: HashSet<usize> = HashSet::new();
    let mut involved_values = involved_values
        .into_iter()
        .map(|v| format!("z{:02}", v))
        .collect::<HashSet<_>>();
    let differing_z_bits = involved_values.clone();

    loop {
        let next_expressions = expressions
            .iter()
            .enumerate()
            .filter(|(_, e)| involved_values.contains(&e.result))
            .map(|(i, _)| i)
            .collect::<HashSet<_>>();

        involved_expressions = involved_expressions
            .union(&next_expressions)
            .cloned()
            .collect::<HashSet<_>>();

        for e in involved_expressions.iter() {
            let e = &expressions[*e];
            involved_values.insert(e.left.clone());
            involved_values.insert(e.right.clone());
            involved_values.insert(e.result.clone());
        }

        if size == involved_expressions.len() {
            break;
        }
        size = involved_expressions.len();
    }

    print_graph(
        &expressions,
        &involved_expressions,
        &differing_z_bits,
        "../../../all.dot",
    );

    let sub = build_subgraph(&expressions, 43);
    print_graph(&sub, &HashSet::new(), &HashSet::new(), "../../../z43.dot");

    // these 8 are wrong; they were found by intensely looking at the graph output from above
    // x12 AND y12 -> z12
    // hnd XOR ggr -> kwb
    // x29 AND y29 -> jqn
    // x29 XOR y29 -> cph
    // cmc AND gkw -> z16
    // cmc XOR gkw -> qkf
    // vhm  OR wwd -> z24
    // ttg XOR stj -> tgr
    let mut result = ["z12", "kwb", "jqn", "cph", "z16", "qkf", "z24", "tgr"];
    result.sort();
    Ok(result.join(","))
}
