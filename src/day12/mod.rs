use std::collections::{HashMap, HashSet};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

type Base = i32;
type Coords = (Base, Base);
type Map<T> = HashMap<Coords, T>;
type PlotMap = Map<char>;
type LabelMap = Map<u32>;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Result<PlotMap> {
    //    let input = "AAAA
    //BBCD
    //BBCC
    //EEEC
    //";

    let map = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c))
        })
        .collect::<HashMap<_, _>>();
    Ok(map)
}

fn has_label(map: &LabelMap, c: &Coords) -> bool {
    map.get(c).is_some()
}

fn next_plot_without_label(plots: &PlotMap, labels: &LabelMap) -> Option<(Coords, char)> {
    plots
        .iter()
        .find(|p| !has_label(labels, p.0))
        .map(|c| (*c.0, *c.1))
}

fn label_map(plots: &PlotMap) -> LabelMap {
    let mut labels: LabelMap = HashMap::new();
    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut current_label = 0;

    while let Some(p) = next_plot_without_label(plots, &labels) {
        let mut queue = vec![p];

        while let Some((c, t)) = queue.pop() {
            labels.entry(c).or_insert(current_label);
            let mut same_neighbors = dirs
                .iter()
                .map(|d| (d.0 + c.0, d.1 + c.1))
                .filter_map(|c| Some((c, *plots.get(&c)?)))
                .filter(|pt| pt.1 == t)
                .filter(|pt| !has_label(&labels, &pt.0))
                .collect::<Vec<_>>();
            queue.append(&mut same_neighbors);
        }

        current_label += 1;
    }

    labels
}

fn count_other_neighbors(plots: &PlotMap, c: &Coords) -> usize {
    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    if let Some(t) = plots.get(c) {
        let no_plots = dirs
            .iter()
            .map(|d| (d.0 + c.0, d.1 + c.1))
            .filter(|c| plots.get(c).is_none())
            .count();

        let other_plots = dirs
            .iter()
            .map(|d| (d.0 + c.0, d.1 + c.1))
            .filter_map(|c| Some((c, *plots.get(&c)?)))
            .filter(|pt| pt.1 != *t)
            .count();
        no_plots + other_plots
    } else {
        0
    }
}

fn fencing_costs(label: u32, plots: &PlotMap, labels: &LabelMap) -> usize {
    let points = labels
        .iter()
        .filter(|p| *p.1 == label)
        .map(|p| *p.0)
        .collect::<Vec<_>>();
    let area = points.len();
    let perimeter = points
        .iter()
        .map(|p| count_other_neighbors(plots, p))
        .sum::<usize>();

    area * perimeter
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &PlotMap) -> Result<usize> {
    let plots = input;
    let labels = label_map(plots);

    let label_list = labels.iter().map(|p| p.1).collect::<HashSet<_>>();
    let costs = label_list
        .iter()
        .map(|l| fencing_costs(**l, plots, &labels))
        .sum::<usize>();

    Ok(costs)
}

fn new_perimeter(label: u32, labels: &LabelMap) -> usize {
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let polygon = labels
        .iter()
        .filter(|l| *l.1 == label)
        .map(|l| *l.0)
        .collect::<HashSet<_>>();

    let sum = polygon
        .iter()
        .map(|c| {
            let mut total = 0;
            let outside = dirs
                .iter()
                .map(|d| (d.0 + c.0, d.1 + c.1))
                .filter(|v| !polygon.contains(v))
                .count();

            // The center element is what we're looking at below's mini samples

            // single cell -> four corners
            // BBB
            // BAB
            // BBB
            if outside == 4 {
                total += 4;
            }

            // single row/column start/end -> 2 corners
            // BBB
            // BAB
            // BAB
            if outside == 3 {
                total += 2;
            }

            // outside == 2 could be inside a tube or a classic corner
            // BAB
            // BAB
            // BAB
            let is_inside_tube = (polygon.contains(&(c.0 - 1, c.1))
                && polygon.contains(&(c.0 + 1, c.1)))
                || (polygon.contains(&(c.0, c.1 - 1)) && polygon.contains(&(c.0, c.1 + 1)));

            // classic corner
            // AAB
            // AAB
            // BBB
            if outside == 2 && !is_inside_tube {
                total += 1;
            }


            // BAA
            // AAA
            // AAA
            if !polygon.contains(&(c.0 - 1, c.1 - 1))
                && polygon.contains(&(c.0 - 1, c.1))
                && polygon.contains(&(c.0, c.1 - 1))
            {
                total += 1;
            }

            // AAB
            // AAA
            // AAA
            if !polygon.contains(&(c.0 - 1, c.1 + 1))
                && polygon.contains(&(c.0 - 1, c.1))
                && polygon.contains(&(c.0, c.1 + 1))
            {
                total += 1;
            }

            // AAA
            // AAA
            // AAB
            if !polygon.contains(&(c.0 + 1, c.1 + 1))
                && polygon.contains(&(c.0 + 1, c.1))
                && polygon.contains(&(c.0, c.1 + 1))
            {
                total += 1;
            }

            // AAA
            // AAA
            // BAA
            if !polygon.contains(&(c.0 + 1, c.1 - 1))
                && polygon.contains(&(c.0 + 1, c.1))
                && polygon.contains(&(c.0, c.1 - 1))
            {
                total += 1;
            }

            total
        })
        .sum::<usize>();

    sum
}

fn fencing_costs_with_discount(label: u32, labels: &LabelMap) -> usize {
    let points = labels
        .iter()
        .filter(|p| *p.1 == label)
        .map(|p| *p.0)
        .collect::<Vec<_>>();
    let area = points.len();
    let perimeter = new_perimeter(label, labels);

    area * perimeter
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &PlotMap) -> Result<usize> {
    let plots = input;
    let labels = label_map(plots);

    let label_list = labels.iter().map(|p| p.1).collect::<HashSet<_>>();
    let costs = label_list
        .iter()
        .map(|l| fencing_costs_with_discount(**l, &labels))
        .sum::<usize>();

    Ok(costs)
}

#[cfg(test)]
mod test {
    use super::*;
}
