/*
 * Copyright 2023 Golden_Water
 * All rights reserved
 */

use itertools::Itertools;

use crate::utils::Then;

#[derive(Debug)]
struct MapItem {
  src: u64,
  src_end: u64,
  off: i64,
}

impl MapItem {
  pub fn new(src: u64, dst: u64, len: u64) -> Self {
    Self {
      src,
      src_end: src + len,
      off: -(src as i64 - dst as i64),
    }
  }

  #[inline(always)]
  pub fn is_in_range(&self, value: u64) -> bool {
    (self.src..self.src_end).contains(&value)
  }

  #[inline(always)]
  pub fn map(&self, value: u64) -> u64 {
    value.wrapping_add_signed(self.off)
  }
}

fn parse(input: String) -> (Vec<u64>, Vec<Vec<MapItem>>) {
  let mut lines = input.lines();
  let seeds = lines.next().unwrap();

  let seeds = seeds
    .split_once(':')
    .unwrap()
    .1
    .trim()
    .split(' ')
    .map(str::parse::<u64>)
    .map(Result::unwrap)
    .collect_vec();

  let mut maps = vec![];

  while let Some(_) = lines.next() {
    lines
      .take_while_ref(|it| !it.is_empty())
      .skip(1)
      .map(|it| {
        it.split(' ')
          .map(str::parse::<u64>)
          .map(Result::unwrap)
          .collect_tuple::<(_, _, _)>()
          .unwrap()
      })
      .map(|(dst, src, len)| MapItem::new(src, dst, len))
      .sorted_by_key(|it| it.src)
      .collect_vec()
      .then(|map| maps.push(map));
  }

  (seeds, maps)
}

pub fn part_1(input: String) {
  let (seeds, maps) = parse(input);

  seeds
    .into_iter()
    .map(|it| {
      maps.iter().fold(it, |seed, map| {
        map
          .iter()
          .find(|it| it.is_in_range(seed))
          .map(|it| it.map(seed))
          .unwrap_or(seed)
      })
    })
    .min()
    .println_dbg()
}

pub fn part_2(input: String) {
  let (seeds, maps) = parse(input);

  let mut ranges = seeds.chunks(2).map(|it| it[0]..it[0] + it[1]).collect_vec();

  // map
  for map in maps {
    // each map item
    let mut mapped = vec![];
    for map_item in map {
      // apply range
      let mut unmapped = vec![];
      for range in ranges {
        let left = range.start..range.end.min(map_item.src);
        let mid = range.start.max(map_item.src)..range.end.min(map_item.src_end);
        let right = range.start.max(map_item.src_end)..range.end;
        if !left.is_empty() {
          unmapped.push(left);
        }
        if !mid.is_empty() {
          mapped.push(
            mid.start.wrapping_add_signed(map_item.off)..mid.end.wrapping_add_signed(map_item.off),
          );
        }
        if !right.is_empty() {
          unmapped.push(right);
        }
      }
      ranges = unmapped;
    }
    ranges.extend(mapped);
  }

  ranges
    .into_iter()
    .min_by_key(|it| it.start)
    .unwrap()
    .start
    .println_dbg();
}
