/*
 * Copyright 2023 Golden_Water
 * All rights reserved
 */

use std::ops::Neg;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::Relaxed;

use itertools::Itertools;
use num::traits::real::Real;

use crate::utils::Then;

const DIRS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Pipe {
  Vertical,
  Horizontal,
  TopRight,
  TopLeft,
  BottomLeft,
  BottomRight,
  Start,
  Empty,
}

impl Pipe {
  pub fn is_connected(&self, to: Pipe, off: (isize, isize)) -> bool {
    match off {
      (0, -1) => {
        matches!(
          *self,
          Pipe::Start | Pipe::Vertical | Pipe::TopLeft | Pipe::TopRight
        ) && matches!(to, Pipe::Vertical | Pipe::BottomRight | Pipe::BottomLeft)
      }
      (0, 1) => {
        matches!(
          *self,
          Pipe::Start | Pipe::Vertical | Pipe::BottomRight | Pipe::BottomLeft
        ) && matches!(to, Pipe::Vertical | Pipe::TopLeft | Pipe::TopRight)
      }
      (-1, 0) => {
        matches!(
          *self,
          Pipe::Start | Pipe::Horizontal | Pipe::BottomLeft | Pipe::TopLeft
        ) && matches!(to, Pipe::Horizontal | Pipe::BottomRight | Pipe::TopRight)
      }
      (1, 0) => {
        matches!(
          *self,
          Pipe::Start | Pipe::Horizontal | Pipe::BottomRight | Pipe::TopRight
        ) && matches!(to, Pipe::Horizontal | Pipe::BottomLeft | Pipe::TopLeft)
      }
      (_, _) => {
        unreachable!()
      }
    }
  }
}

fn get_connected_pipes(
  map: &[Vec<(Pipe, AtomicU8)>],
  x: usize,
  y: usize,
  visited: u8,
  unvisited: u8,
) -> Vec<(usize, usize)> {
  let loc = &map[y][x];
  loc.1.store(visited, Relaxed);
  let pipe = loc.0;

  DIRS
    .into_iter()
    .filter_map(|(x_off, y_off)| {
      if let (Some(x), Some(y)) = (x.checked_add_signed(x_off), y.checked_add_signed(y_off)) {
        if y < map.len() && x < map[y].len() {
          let it = &map[y][x];
          (it.1.load(Relaxed) == unvisited && pipe.is_connected(it.0, (x_off, y_off)))
            .then_some((x, y))
        } else {
          None
        }
      } else {
        None
      }
    })
    .collect_vec()
}

impl From<char> for Pipe {
  fn from(value: char) -> Self {
    match value {
      '|' => Self::Vertical,
      '-' => Self::Horizontal,
      'L' => Self::TopRight,
      'J' => Self::TopLeft,
      '7' => Self::BottomLeft,
      'F' => Self::BottomRight,
      'S' => Self::Start,
      '.' => Self::Empty,
      _ => {
        unreachable!()
      }
    }
  }
}

pub fn part_1(input: String) {
  let map = parse(input);
  let mut search_front = get_starts(&map).2;

  let mut count = 0;
  while !search_front.is_empty() {
    search_front = search_front
      .into_iter()
      .flat_map(|(x, y)| get_connected_pipes(&map, x, y, 1, 0))
      .collect_vec();
    count += 1;
  }

  count.println();
}

pub fn part_2(input: String) {
  let map = parse(input);
  let (x, y, starts) = get_starts(&map);
  let mut search_front = starts.clone();

  while !search_front.is_empty() {
    search_front = search_front
      .into_iter()
      .flat_map(|(x, y)| get_connected_pipes(&map, x, y, 1, 0))
      .collect_vec();
  }

  map[y][x].1.store(2, Relaxed);
  let mut prev = (x, y);
  let mut cur = Some(starts[0]);

  loop {
    let (x, y) = cur.take().unwrap();
    let next = get_connected_pipes(&map, x, y, 2, 1);
    let next = next.into_iter().next();

    let next = if let Some(next) = next { next } else { break };

    let prev_off = (
      (prev.0 as isize).wrapping_sub_unsigned(x),
      (prev.1 as isize).wrapping_sub_unsigned(y),
    );
    let cur_dir = (
      (x as isize).wrapping_sub_unsigned(prev.0),
      (y as isize).wrapping_sub_unsigned(prev.1),
    );
    let next_off = (
      (next.0 as isize).wrapping_sub_unsigned(x),
      (next.1 as isize).wrapping_sub_unsigned(y),
    );

    /// return true if turn right
    fn is_turn_right(to_prev: (isize, isize), to_next: (isize, isize)) -> bool {
      let (x1, y1) = (to_prev.0 as f64, to_prev.1 as f64);
      let (x2, y2) = (to_next.0 as f64, to_next.1 as f64);
      let dot = x1 * x2 + y1 * y2;
      let det = x1 * y2 - y1 * x2;
      det.atan2(dot).is_sign_negative()
    }

    // left: false, right: true
    let straight = cur_dir == next_off;
    let dirs = if straight {
      DIRS
        .into_iter()
        .filter(|it| (next_off.0 * it.0 + next_off.1 * it.1) == 0)
        .map(|it| (it, is_turn_right(prev_off, it)))
        .collect_vec()
    } else {
      let is_right = !is_turn_right(prev_off, next_off);
      vec![
        (cur_dir, is_right),
        ((next_off.0.neg(), next_off.1.neg()), is_right),
      ]
    };

    for ((x_off, y_off), is_right) in dirs.into_iter() {
      let next = (x.checked_add_signed(x_off), y.checked_add_signed(y_off));
      if let (Some(x), Some(y)) = next {
        if y < map.len() && x < map[y].len() && map[y][x].1.load(Relaxed) == 0 {
          map[y][x].1.store(is_right as u8 + 3, Relaxed);
        }
      }
    }

    prev = (x, y);
    cur = Some(next);
  }

  let mut outer_or_inner = map
    .iter()
    .enumerate()
    .flat_map(|(y, line)| {
      line
        .iter()
        .enumerate()
        .filter_map(|(x, (_, flag))| {
          (flag.load(Relaxed) == 3 || flag.load(Relaxed) == 4).then_some((x, y))
        })
        .collect_vec()
    })
    .collect_vec();

  while !outer_or_inner.is_empty() {
    outer_or_inner = outer_or_inner
      .into_iter()
      .flat_map(|(x, y)| {
        let cur = map[y][x].1.load(Relaxed);

        DIRS
          .into_iter()
          .filter_map(|(x_off, y_off)| {
            let next = (x.checked_add_signed(x_off), y.checked_add_signed(y_off));
            if let (Some(x), Some(y)) = next {
              if y < map.len() && x < map[y].len() && map[y][x].1.load(Relaxed) == 0 {
                map[y][x].1.store(cur, Relaxed);
                Some((x, y))
              } else {
                None
              }
            } else {
              None
            }
          })
          .collect_vec()
      })
      .collect_vec();
  }

  for line in map.iter() {
    line.iter().map(|it| it.1.load(Relaxed)).join("").println();
  }

  map
    .iter()
    .map(|it| it.iter().filter(|it| it.1.load(Relaxed) == 3).count())
    .sum::<usize>()
    .println();
  map
    .iter()
    .map(|it| it.iter().filter(|it| it.1.load(Relaxed) == 4).count())
    .sum::<usize>()
    .println();
}

fn get_starts(map: &[Vec<(Pipe, AtomicU8)>]) -> (usize, usize, Vec<(usize, usize)>) {
  for (y, line) in map.iter().enumerate() {
    for (x, &(pipe, _)) in line.iter().enumerate() {
      if pipe == Pipe::Start {
        return (x, y, get_connected_pipes(map, x, y, 1, 0));
      }
    }
  }
  unreachable!()
}

fn parse(input: String) -> Vec<Vec<(Pipe, AtomicU8)>> {
  input
    .lines()
    .map(|it| it.chars().map(Pipe::from).collect_vec())
    .map(|it| {
      it.into_iter()
        .map(|it| (it, AtomicU8::new(0)))
        .collect_vec()
    })
    .collect_vec()
}
