/*
 * Copyright 2023 Golden_Water
 * All rights reserved
 */

use itertools::Itertools;
use num::Integer;

use crate::utils::Then;

#[derive(Debug, Copy, Clone)]
enum Direction {
  Left,
  Right,
}

impl From<char> for Direction {
  fn from(value: char) -> Self {
    match value {
      'L' => Self::Left,
      'R' => Self::Right,
      _ => unreachable!(),
    }
  }
}

pub fn part_1(input: String) {
  let (instructions, map) = parse(&input);
  let (map_arr, map_ends_with_z) = gen_map_arr(&map);

  map
    .iter()
    .enumerate()
    .find(|it| it.1 .0 == "AAA")
    .map(|it| it.0 as u16)
    .map(|it| find(&instructions, &map_arr, &map_ends_with_z, it))
    .unwrap()
    .println();
}

pub fn part_2(input: String) {
  let (instructions, map) = parse(&input);
  let (map_arr, map_ends_with_z) = gen_map_arr(&map);

  map
    .iter()
    .enumerate()
    .filter(|(_, it)| it.0.ends_with('A'))
    .map(|it| it.0 as u16)
    .map(|it| find(&instructions, &map_arr, &map_ends_with_z, it))
    .reduce(|acc, cur| acc.lcm(&cur))
    .unwrap()
    .println();
}

fn gen_map_arr(map: &[(&str, (&str, &str))]) -> (Vec<(u16, u16)>, Vec<bool>) {
  let mut map_arr = vec![];
  let mut map_ends_with_z = vec![];
  for (cur, (left, right)) in map {
    let left = map
      .iter()
      .enumerate()
      .find(|(_, it)| it.0 == *left)
      .unwrap()
      .0 as u16;
    let right = map
      .iter()
      .enumerate()
      .find(|(_, it)| it.0 == *right)
      .unwrap()
      .0 as u16;
    let is_endswith_z = cur.ends_with('Z');
    map_arr.push((left, right));
    map_ends_with_z.push(is_endswith_z);
  }
  (map_arr, map_ends_with_z)
}

fn find(
  instructions: &[Direction],
  map_arr: &[(u16, u16)],
  map_ends_with_z: &[bool],
  mut cur: u16,
) -> usize {
  for (count, ins) in instructions.iter().cycle().enumerate() {
    if map_ends_with_z[cur as usize] {
      return count;
    }

    let dirs = map_arr[cur as usize];
    let next = match ins {
      Direction::Left => dirs.0,
      Direction::Right => dirs.1,
    };

    cur = next;
  }
  0
}

type ParseResult<'a> = (Vec<Direction>, Vec<(&'a str, (&'a str, &'a str))>);

fn parse(input: &str) -> ParseResult<'_> {
  let mut lines = input.lines();
  let instructions = lines
    .next()
    .unwrap()
    .chars()
    .map(Direction::from)
    .collect_vec();

  lines.next();

  let map = lines
    .map(|it| {
      it.split_once('=').unwrap().then(|(from, to)| {
        (
          from.trim(),
          to.split_once(',').unwrap().then(|(left, right)| {
            (
              left.trim_start_matches(['(', ' ']),
              right.trim_end_matches([')', ' ']).trim(),
            )
          }),
        )
      })
    })
    .collect_vec();
  (instructions, map)
}
