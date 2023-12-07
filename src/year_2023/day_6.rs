/*
 * Copyright 2023 Golden_Water
 * All rights reserved
 */

use itertools::Itertools;

use crate::utils::Then;

fn parse(input: String) -> Vec<(u64, u64)> {
  let mut lines = input
    .lines()
    .map(|it| {
      it.trim_start_matches("Time:")
        .trim_start_matches("Distance:")
    })
    .map(|it| {
      it.split_whitespace()
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect_vec()
    });
  let times = lines.next().unwrap();
  let distance = lines.next().unwrap();

  times.into_iter().zip_eq(distance).collect_vec()
}

pub fn part_1(input: String) {
  parse(input)
    .into_iter()
    .map(|(time, distance)| {
      (1..time)
        .filter_map(|it| ((time - it) * it > distance).then_some(()))
        .count()
    })
    .product::<usize>()
    .println_dbg();
}

pub fn part_2(input: String) {
  let (time, distance) = parse(input).into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
  let (time, distance) = [time, distance]
    .into_iter()
    .map(|it| {
      it.iter()
        .map(u64::to_string)
        .join("")
        .parse::<u64>()
        .unwrap()
    })
    .collect_tuple::<(_, _)>()
    .unwrap();

  let left = (1..time).find(|&it| (time - it) * it > distance).unwrap();
  let right = (1..time).rfind(|&it| (time - it) * it > distance).unwrap();

  (right - left + 1).println_dbg();
}
