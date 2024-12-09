use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::utils::AocError::*;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum FileSystemEntry {
    Empty(u32),
    File(u32, u32),
}

impl FileSystemEntry {
    pub fn is_empty(&self) -> bool {
        matches!(self, Empty(_))
    }

    pub fn is_file(&self) -> bool {
        matches!(self, File(_, _))
    }
}

use FileSystemEntry::*;

#[aoc_generator(day09)]
pub fn input_generator(input: &str) -> Result<Vec<FileSystemEntry>> {
    // let input = "2333133121414131402";

    input
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .map(|(i, c)| {
            let size = c
                .to_digit(10)
                .ok_or(GenericError)
                .context("Could not parse size")?;
            Ok(match i % 2 {
                0 => File(size, i as u32 / 2),
                1 => Empty(size),
                _ => unimplemented!(),
            })
        })
        .collect::<Result<Vec<_>>>()
}

fn can_be_compacted(hdd: &[FileSystemEntry]) -> bool {
    let first_empty = hdd
        .iter()
        .find_position(|v| v.is_empty())
        .map(|v| v.0)
        .unwrap_or(hdd.len());
    let last_file = hdd.len()
        - 1
        - hdd
            .iter()
            .rev()
            .find_position(|v| v.is_file())
            .map(|v| v.0)
            .unwrap_or(0);

    first_empty < last_file
}

fn hash_hdd(hdd: &[FileSystemEntry]) -> usize {
    hdd.iter()
        .fold((0_usize, 0_usize), |(index, sum), v| {
            if let File(size, id) = v {
                let size = *size as usize;
                let plus = (index..index + size)
                    .map(|i| i * *id as usize)
                    .sum::<usize>();
                (index + size, sum + plus)
            } else if let Empty(size) = v {
                (index + *size as usize, sum)
            } else {
                (index, sum)
            }
        })
        .1
}

#[aoc(day09, part1)]
pub fn solve_part1(input: &[FileSystemEntry]) -> Result<usize> {
    let mut hdd = input.to_vec();

    while can_be_compacted(&hdd) {
        let first_empty = hdd
            .clone()
            .into_iter()
            .find_position(|v| v.is_empty())
            .ok_or(GenericError)
            .context("No free space left on device")?;
        let mut last_file = hdd
            .clone()
            .into_iter()
            .rev()
            .find_position(|v| v.is_file())
            .ok_or(GenericError)
            .context("No file found")?;
        last_file.0 = hdd.len() - last_file.0 - 1;

        match (first_empty.1, last_file.1) {
            (Empty(free_space), File(file_size, id)) => {
                if free_space < file_size {
                    hdd[last_file.0] = File(file_size - free_space, id);
                    hdd[first_empty.0] = File(free_space, id);
                } else if free_space == file_size {
                    hdd[last_file.0] = Empty(free_space);
                    hdd[first_empty.0] = File(free_space, id);
                } else if free_space > file_size {
                    hdd.remove(last_file.0);
                    hdd[first_empty.0] = Empty(free_space - file_size);
                    hdd.insert(first_empty.0, File(file_size, id));
                }
            }
            _ => unimplemented!(),
        }
    }

    Ok(hash_hdd(&hdd))
}

#[aoc(day09, part2)]
pub fn solve_part2(input: &[FileSystemEntry]) -> Result<usize> {
    let mut hdd = input.to_vec();
    let max_file_id = *input
        .iter()
        .filter_map(|v| if let File(_, id) = v { Some(id) } else { None })
        .max()
        .ok_or(GenericError)
        .context("Empty disk")?;

    for id in (0..=max_file_id).rev() {
        let file = hdd.clone().into_iter().find_position(|f| if let File(_, i) = f {
            *i == id
        } else { false }).ok_or(GenericError).context("Could not find specific file")?;

        match file.1 {
            File(file_size, id) => {
                let first_empty = hdd
                    .clone()
                    .into_iter()
                    .find_position(|v| if let Empty(u) = v {
                        *u >= file_size
                    } else {
                        false
                    });

                match first_empty {
                    None => continue,
                    Some((idx, Empty(size))) => {
                        if idx > file.0 {
                            continue;
                        }
                        if size > file_size {
                            hdd[idx] = Empty(size - file_size);
                            hdd[file.0] = Empty(file_size);
                            hdd.insert(idx, File(file_size, id));
                        } else {
                            hdd[idx] = File(file_size, id);
                            hdd[file.0] = Empty(file_size);
                        }
                    },
                    _ => unimplemented!(),
                }
            },
            _ => unimplemented!(),
        }
    }

    Ok(hash_hdd(&hdd))
}

#[cfg(test)]
mod test {
    use super::*;
}
