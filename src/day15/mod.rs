use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, MinMaxResult};

use crate::utils::AocError::*;

type Base = i32;
type Coords = (Base, Base);
type Map = HashMap<Coords, char>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn from_char(c: char) -> Result<Self> {
        Ok(match c {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => Err(GenericError).context("Invalid direction")?,
        })
    }

    pub fn to_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }

    pub fn to_velocity(&self) -> Option<Coords> {
        Some(match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        })
    }
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Result<(Map, Vec<Direction>)> {
//    let input = "##########
//#..O..O.O#
//#......O.#
//#.OO..O.O#
//#..O@..O.#
//#O#..O...#
//#O..O..O.#
//#.OO.O.OO#
//#....O...#
//##########
//
//<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
//vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
//><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
//<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
//^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
//^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
//>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
//<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
//^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
//v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    let mut input = input.split("\n\n");

    let map = input
        .next()
        .ok_or(GenericError)
        .context("Could not parse map")?
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c))
        })
        .collect::<HashMap<_, _>>();

    let directions = input
        .next()
        .ok_or(GenericError)
        .context("Could not parse directions")?
        .chars()
        .filter(|c| *c != '\n' && *c != '\r')
        .map(Direction::from_char)
        .collect::<Result<Vec<_>>>()?;

    Ok((map, directions))
}

fn find_start_pos(map: &Map) -> Option<Coords> {
    map.iter().find(|(_, e)| **e == '@').map(|(c, _)| *c)
}

fn tick(map: Map, pos: Coords, dir: &Direction) -> Option<(Map, Coords)> {
    let v = dir.to_velocity()?;
    let new_pos = (pos.0 + v.0, pos.1 + v.1);

    if map.get(&new_pos)? == &'#' {
        return Some((map, pos));
    }

    if map.get(&new_pos)? == &'O' {
        let mut new_map = map.clone();
        let mut target = (new_pos.0 + v.0, new_pos.1 + v.1);

        new_map.entry(new_pos).and_modify(|t| *t = '.');
        while let Some(t) = map.get(&target) {
            if *t == '#' {
                return Some((map, pos));
            }

            if *t == '.' || *t == '@' {
                new_map.entry(target).and_modify(|t| *t = 'O');
                return Some((new_map, new_pos));
            }

            target = (target.0 + v.0, target.1 + v.1);
        }
    }

    Some((map, new_pos))
}

fn size(map: &Map) -> Option<(i32, i32)> {
    let width = map.iter().map(|(c, _)| c.1).max()?;
    let height = map.iter().map(|(c, _)| c.0).max()?;

    Some((height, width))
}

fn format_map(map: &Map) -> Option<String> {
    let (height, width) = size(map)?;

    let mut out = vec![];
    for y in 0..=height {
        let mut line = vec![];
        for x in 0..=width {
            line.push(map.get(&(y, x))?);
        }
        line.push(&'\n');
        out.append(&mut line);
    }

    Some(out.into_iter().collect::<String>())
}

fn print_map(map: &Map) -> Option<()> {
    let m = format_map(map)?;
    print!("{}", m);

    Some(())
}

fn print_map_with_pos(map: &Map, pos: &Coords, dir: &Direction) -> Option<()> {
    let mut map = map.clone();
    let start = find_start_pos(&map)?;
    map.entry(start).and_modify(|v| {
        if *v == '@' {
            *v = '.'
        }
    });
    //map.entry(*pos).and_modify(|v| *v = dir.to_char());
    map.entry(*pos).and_modify(|v| *v = '@');

    print_map(&map)
}

fn hash_map(map: &Map) -> i32 {
    map.iter()
        .map(|((y, x), t)| match t {
            'O' => 100 * y + x,
            '[' => 100 * y + x,
            _ => 0,
        })
        .sum::<i32>()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &(Map, Vec<Direction>)) -> Result<i32> {
    let (map, directions) = input;

    let pos = map
        .iter()
        .find(|(_, e)| **e == '@')
        .map(|(c, _)| *c)
        .ok_or(GenericError)
        .context("Could not find starting position")?;

    let (map, _pos) = directions
        .iter()
        .try_fold((map.clone(), pos), |acc, dir| tick(acc.0, acc.1, dir))
        .ok_or(GenericError)
        .context("Folding failed")?;

    Ok(hash_map(&map))
}

fn scale(map: &Map) -> Option<Map> {
    let (width, height) = size(map)?;

    Some(
        (0..=height)
            .map(|y| {
                (0..=width).fold("".to_string(), |acc, x| match map.get(&(y, x)) {
                    Some('#') => format!("{}##", acc),
                    Some('O') => format!("{}[]", acc),
                    Some('.') => format!("{}..", acc),
                    Some('@') => format!("{}@.", acc),
                    _ => acc,
                })
            })
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(move |(x, c)| ((y as i32, x as i32), c))
                    .collect::<Vec<_>>()
            })
            .collect::<HashMap<_, _>>(),
    )
}

fn tick2(map: Map, pos: Coords, dir: &Direction) -> Option<(Map, Coords)> {
    if *dir == Direction::Up || *dir == Direction::Down {
        tick_vertical(map, pos, dir)
    } else {
        tick_horizontal(map, pos, dir)
    }
}

fn tick_horizontal(map: Map, pos: Coords, dir: &Direction) -> Option<(Map, Coords)> {
    let v = dir.to_velocity()?;
    let new_pos = (pos.0 + v.0, pos.1 + v.1);

    if map.get(&new_pos)? == &'#' {
        return Some((map, pos));
    }

    if map.get(&new_pos)? == &'[' || map.get(&new_pos)? == &']' {
        let mut target = (new_pos.0 + v.0, new_pos.1 + v.1);

        while let Some(t) = map.get(&target) {
            if *t == '#' {
                return Some((map, pos));
            }

            if *t == '.' || *t == '@' {
                break;
            }

            target = (target.0 + v.0, target.1 + v.1);
        }

        let mut new_map = map.clone();

        let start = if *dir == Direction::Left {
            target.1
        } else {
            new_pos.1 + 1
        };
        let end = if *dir == Direction::Left {
            new_pos.1 - 1
        } else {
            target.1
        };
        let mut box_chars = ['[', ']'].into_iter().cycle();

        new_map.entry(new_pos).and_modify(|t| *t = '.');
        for x in start..=end {
            let box_char = box_chars.next()?;
            new_map.entry((pos.0, x)).and_modify(|t| *t = box_char);
        }

        return Some((new_map, new_pos));
    }

    Some((map, new_pos))
}

fn tick_vertical(map: Map, pos: Coords, dir: &Direction) -> Option<(Map, Coords)> {
    let v = dir.to_velocity()?;
    let new_pos = (pos.0 + v.0, pos.1 + v.1);

    if map.get(&new_pos)? == &'#' {
        return Some((map, pos));
    }

    if map.get(&new_pos)? == &'[' || map.get(&new_pos)? == &']' {
        let mut new_map = map.clone();
        let mut seeds = [pos].into_iter().collect::<HashSet<_>>();
        let mut boxes_to_move = vec![];

        loop {
            let mut this_layer = HashSet::new();
            let mut all_empty = true;
            for p in seeds {
                let new = (p.0 + v.0, p.1);
                let c = map.get(&new);

                if c == Some(&'#') {
                    return Some((map, pos));
                }

                let is_empty = c == Some(&'.') || c == Some(&'@');
                all_empty &= is_empty;

                if is_empty {
                    continue;
                }

                this_layer.insert(new);
                let new2 = if map.get(&new) == Some(&'[') {
                    (new.0, new.1 + 1)
                } else {
                    (new.0, new.1 - 1)
                };
                this_layer.insert(new2);
            }

            if all_empty {
                break;
            }

            // println!("Collected: {:?}", this_layer);
            boxes_to_move.push(this_layer.clone());
            seeds = this_layer;
        }

        let mut y = pos.0 + (boxes_to_move.len() as i32 + 1) * v.0;
        for set in boxes_to_move.into_iter().rev() {
            let mut sorted = set.into_iter().collect::<Vec<_>>();
            sorted.sort_by(|a, b| a.1.cmp(&b.1));

            let mut dirs = ['[', ']'].into_iter().cycle();

            for p in sorted.iter() {
                let b = dirs.next()?;
                new_map.entry((y, p.1)).and_modify(|v| *v = b);
            }

            y -= v.0;
            for p in sorted {
                new_map.entry((y, p.1)).and_modify(|v| *v = '.');
            }
        }

        return Some((new_map, new_pos));
    }

    Some((map, new_pos))
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &(Map, Vec<Direction>)) -> Result<i32> {
    let (map, directions) = input;

    let map = scale(map)
        .ok_or(GenericError)
        .context("Unable to scale map")?;

    let pos = find_start_pos(&map)
        .ok_or(GenericError)
        .context("Could not find start pos")?;

    let (map, pos) = directions
        .iter()
        .try_fold((map.clone(), pos), |acc, dir| {
            let (map, pos) = tick2(acc.0.clone(), acc.1, dir)?;

            // if *dir == Direction::Up || *dir == Direction::Down {
            //     print_map_with_pos(&map, &pos, dir);
            // }

            Some((map, pos))
        })
        .ok_or(GenericError)
        .context("Folding failed")?;

    print_map_with_pos(&map, &pos, &Direction::Up);

    Ok(hash_map(&map))
}

#[cfg(test)]
mod test {
    use super::*;

    fn run(map: Map, dirs: Vec<Direction>) -> Result<Map> {
        let pos = find_start_pos(&map).ok_or(GenericError)?;
        let (new_map, _) = dirs
            .iter()
            .try_fold((map, pos), |acc, dir| tick2(acc.0, acc.1, dir))
            .ok_or(GenericError)?;
        print_map(&new_map)
            .ok_or(GenericError)
            .context("Could not print map")?;

        Ok(new_map)
    }

    #[test]
    fn tick_up_bug1() -> Result<()> {
        let input = "####################
##................##
##.....[].[]..[]..##
##......[][]......##
##.......[].......##
##........@.......##
####################

^";
        let expected = "####################
##.....[].[]......##
##......[][]..[]..##
##.......[].......##
##................##
##........@.......##
####################\n";
        let (map, dirs) = input_generator(input)?;
        let map = run(map, dirs)?;
        let out = format_map(&map)
            .ok_or(GenericError)
            .context("Could not format map")?;

        assert_eq!(expected, out);

        Ok(())
    }

    #[test]
    fn tick_down_weird_occurrence() -> Result<()> {
        let input = "####################
##[]..[]......[][]##
##[]...........[].##
##...........@[][]##
##..........[].[].##
##..##[]..[].[]...##
##...[]...[]..[]..##
##.....[]..[].[][]##
##........[]......##
####################

v";
        let expected = "####################
##[]..[]......[][]##
##[]...........[].##
##...........@[][]##
##.............[].##
##..##[]..[][]....##
##...[]...[].[]...##
##.....[]..[].[][]##
##........[]..[]..##
####################\n";
        let (map, dirs) = input_generator(input)?;
        let map = run(map, dirs)?;
        let out = format_map(&map)
            .ok_or(GenericError)
            .context("Could not format map")?;

        assert_eq!(expected, out);

        Ok(())
    }

    #[test]
    fn tick_up_once() -> Result<()> {
        let input = "###############
##...........##
##....[].....##
##....@......##
###############

^";
        let (map, dirs) = input_generator(input)?;
        let map = run(map, dirs)?;
        assert_eq!(Some(&'['), map.get(&(1, 6)));
        assert_eq!(Some(&']'), map.get(&(1, 7)));

        Ok(())
    }

    #[test]
    fn tick_up_multi_layered_single_column() -> Result<()> {
        let input = "###############
##...........##
##...........##
##....[].....##
##....[].....##
##....@......##
###############

^";
        let (map, dirs) = input_generator(input)?;
        let map = run(map, dirs)?;
        assert_eq!(Some(&'['), map.get(&(2, 6)));
        assert_eq!(Some(&']'), map.get(&(2, 7)));
        assert_eq!(Some(&'['), map.get(&(3, 6)));
        assert_eq!(Some(&']'), map.get(&(3, 7)));

        Ok(())
    }

    #[test]
    fn tick_up_multi_layered_multiple_columns() -> Result<()> {
        let input = "###############
##...........##
##...........##
##...[][]....##
##....[].....##
##....@......##
###############

^^";
        let (map, dirs) = input_generator(input)?;
        print_map(&map);
        let map = run(map, dirs)?;
        assert_eq!(Some(&'['), map.get(&(1, 5)));
        assert_eq!(Some(&']'), map.get(&(1, 6)));
        assert_eq!(Some(&'['), map.get(&(1, 7)));
        assert_eq!(Some(&']'), map.get(&(1, 8)));
        assert_eq!(Some(&'['), map.get(&(2, 6)));
        assert_eq!(Some(&']'), map.get(&(2, 7)));

        Ok(())
    }

    #[test]
    fn tick_down_with_weird_constellation() -> Result<()> {
        let input = "############
##.....@..##
##..##[]..##
##...[]...##
##.....[].##
##........##
############

v";
        let (map, dirs) = input_generator(input)?;
        let map = run(map, dirs)?;
        assert_eq!(Some(&'['), map.get(&(3, 6)));
        assert_eq!(Some(&']'), map.get(&(3, 7)));
        assert_eq!(Some(&'['), map.get(&(4, 5)));
        assert_eq!(Some(&']'), map.get(&(4, 6)));
        assert_eq!(Some(&'['), map.get(&(4, 7)));
        assert_eq!(Some(&']'), map.get(&(4, 8)));

        Ok(())
    }

    #[test]
    fn tick_left_twice() -> Result<()> {
        let input = "###############
##....[][]@..##
###############

<<";
        let (map, dirs) = input_generator(input)?;
        let map = run(map, dirs)?;
        assert_eq!(Some(&'['), map.get(&(1, 4)));
        assert_eq!(Some(&']'), map.get(&(1, 5)));
        assert_eq!(Some(&'['), map.get(&(1, 6)));
        assert_eq!(Some(&']'), map.get(&(1, 7)));

        Ok(())
    }

    #[test]
    fn tick_right_twice() -> Result<()> {
        let input = "###############
##...@[][]...##
###############

>>";
        let (map, dirs) = input_generator(input)?;
        let map = run(map, dirs)?;
        assert_eq!(Some(&'['), map.get(&(1, 8)));
        assert_eq!(Some(&']'), map.get(&(1, 9)));
        assert_eq!(Some(&'['), map.get(&(1, 10)));
        assert_eq!(Some(&']'), map.get(&(1, 11)));

        Ok(())
    }

    #[test]
    fn tick_into_wall_left() -> Result<()> {
        let input = "###############
##....[][]@..##
###############

<<<<<";
        let (map, dirs) = input_generator(input)?;
        let map = run(map, dirs)?;
        assert_eq!(Some(&'['), map.get(&(1, 2)));
        assert_eq!(Some(&']'), map.get(&(1, 3)));
        assert_eq!(Some(&'['), map.get(&(1, 4)));
        assert_eq!(Some(&']'), map.get(&(1, 5)));

        Ok(())
    }

    #[test]
    fn tick_into_wall_right() -> Result<()> {
        let input = "###############
##...@[][]...##
###############

>>>>>>";
        let (map, dirs) = input_generator(input)?;
        let map = run(map, dirs)?;
        assert_eq!(Some(&'['), map.get(&(1, 9)));
        assert_eq!(Some(&']'), map.get(&(1, 10)));
        assert_eq!(Some(&'['), map.get(&(1, 11)));
        assert_eq!(Some(&']'), map.get(&(1, 12)));

        Ok(())
    }
}
