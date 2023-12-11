/*
 * Copyright 2023 Golden_Water
 * All rights reserved
 */

use itertools::Itertools;

use crate::utils::Then;

pub fn part_1(input: String) {
  let mut map = parse(input);

  let mut i = 0;
  while i < map.len() {
    let empty = !map[i].iter().any(|it| *it);
    if empty {
      map.insert(i, map[i].clone());
      i += 1;
    }
    i += 1;
  }

  let mut i = 0;
  while i < map[0].len() {
    let empty = !map.iter().map(|it| it[i]).any(|it| it);
    if empty {
      map.iter_mut().for_each(|it| it.insert(i, false));
      i += 1;
    }
    i += 1;
  }

  let galaxies = extract_galaxies(&map);

  calc_distance(galaxies).println();

  // for line in map {
  //   line.iter().map(|&it| it as usize).join("").println();
  // }
}

fn extract_galaxies(map: &[Vec<bool>]) -> Vec<(usize, usize)> {
  map
    .iter()
    .enumerate()
    .flat_map(|(y, line)| {
      line
        .iter()
        .enumerate()
        .filter_map(|(x, it)| it.then_some((x, y)))
        .collect_vec()
    })
    .collect_vec()
}

fn parse(input: String) -> Vec<Vec<bool>> {
  input
    .lines()
    .map(|it| it.chars().map(|it| matches!(it, '#')).collect_vec())
    .collect_vec()
}

fn calc_distance(galaxies: Vec<(usize, usize)>) -> usize {
  galaxies
    .iter()
    .combinations(2)
    .map(|it| it[0].0.abs_diff(it[1].0) + it[0].1.abs_diff(it[1].1))
    .sum::<usize>()
}

pub fn part_2(input: String) {
  const EXPAND_BY: usize = 1_000_000 - 1;

  let map = parse(input);
  let mut galaxies = extract_galaxies(&map);

  let x_axis = galaxies
    .iter()
    .map(|it| it.0)
    .sorted()
    .dedup()
    .collect_vec();
  let x_len = *x_axis.last().unwrap();

  (0..=x_len)
    .rev()
    .filter(|&x| !x_axis.iter().any(|&it| it == x))
    .for_each(|x| {
      galaxies
        .iter_mut()
        .filter(|it| it.0 > x)
        .for_each(|it| it.0 += EXPAND_BY);
    });

  let y_axis = galaxies
    .iter()
    .map(|it| it.1)
    .sorted()
    .dedup()
    .collect_vec();
  let y_len = *y_axis.last().unwrap();

  (0..=y_len)
    .rev()
    .filter(|&y| !y_axis.iter().any(|&it| it == y))
    .for_each(|y| {
      galaxies
        .iter_mut()
        .filter(|it| it.1 > y)
        .for_each(|it| it.1 += EXPAND_BY);
    });

  galaxies.println_dbg();
  calc_distance(galaxies).println();
}
