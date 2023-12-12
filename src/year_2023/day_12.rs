/*
 * Copyright 2023 Golden_Water
 * All rights reserved
 */

use std::collections::HashMap;
use std::hash::Hash;

use itertools::Itertools;

use crate::utils::Then;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Spring {
  Ok,
  Dmg,
  Unknown,
}

fn parse(input: String) -> Vec<(Vec<Spring>, Vec<usize>)> {
  input
    .lines()
    .map(|it| {
      it.split_once(' ').unwrap().then(|(springs, groups)| {
        let springs = springs
          .chars()
          .map(|it| match it {
            '.' => Spring::Ok,
            '#' => Spring::Dmg,
            '?' => Spring::Unknown,
            _ => {
              unreachable!()
            }
          })
          .collect_vec();
        let groups = groups
          .split(',')
          .map(str::parse::<usize>)
          .map(Result::unwrap)
          .collect_vec();
        (springs, groups)
      })
    })
    .collect_vec()
}

pub fn part_1(input: String) {
  let spring_groups = parse(input);

  solve(spring_groups).println();
}

pub fn part_2(input: String) {
  let springs = parse(input);

  let springs = springs
    .into_iter()
    .map(|(springs, groups)| {
      (
        Itertools::intersperse((0..5).map(|_| springs.clone()), vec![Spring::Unknown])
          .flatten()
          .collect_vec(),
        groups.repeat(5),
      )
    })
    .collect_vec();

  solve(springs).println();
}

fn solve(spring_groups: Vec<(Vec<Spring>, Vec<usize>)>) -> usize {
  spring_groups
    .into_iter()
    .map(|(mut springs, groups)| {
      solve_inner_2(&mut springs, &groups, 0, None, 0, &mut HashMap::new())
    })
    .sum::<usize>()
}

fn solve_inner_2<'a>(
  springs: &mut [Spring],
  groups: &'a [usize],
  idx: usize,
  cur_replace: Option<Spring>,
  dmg_size: usize,

  cache: &mut HashMap<(&'a [usize], usize, Spring, usize), usize>,
) -> usize {
  if idx == springs.len() {
    let mut group_len = groups.len();

    if dmg_size > 0 {
      if let Some(&size) = groups.first() {
        if size != dmg_size {
          return 0;
        }
        group_len -= 1;
      } else {
        return 0;
      }
    }

    return if group_len != 0 { 0 } else { 1 };
  }

  let cur = if let Some(cur_replace) = cur_replace {
    cur_replace
  } else {
    springs[idx]
  };

  if let Some(cached) = cache.get(&(groups, idx, cur, dmg_size)) {
    return *cached;
  };

  let result = match cur {
    Spring::Ok => {
      let groups = if dmg_size > 0 {
        if let Some((&size, groups)) = groups.split_first() {
          if size != dmg_size {
            return 0;
          }
          groups
        } else {
          return 0;
        }
      } else {
        groups
      };
      solve_inner_2(springs, groups, idx + 1, None, 0, cache)
    }
    Spring::Dmg => solve_inner_2(springs, groups, idx + 1, None, dmg_size + 1, cache),
    Spring::Unknown => {
      solve_inner_2(springs, groups, idx, Some(Spring::Ok), dmg_size, cache)
        + solve_inner_2(springs, groups, idx, Some(Spring::Dmg), dmg_size, cache)
    }
  };

  cache.insert((groups, idx, cur, dmg_size), result);

  result
}
