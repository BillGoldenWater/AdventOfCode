/*
 * Copyright 2023 Golden_Water
 * All rights reserved
 */

use itertools::Itertools;

use crate::utils::Then;

#[derive(Debug)]
enum Item {
  Round,
  Square,
  Empty,
}

pub fn part_1(input: String) {
  let plate = parse(input);

  let blockers = calc_blocker_north(&plate);

  let mut sum = 0;
  for ((_, y), count) in blockers {
    let num = plate.len() - y;
    sum += (0..count).map(|it| num - it).sum::<usize>();
  }
  sum.println_dbg();
}

pub fn part_2(input: String) {
  let mut plate = parse(input);

  let mut sums = vec![];
  // choose a number that large enough to find a repeat
  for _ in 1..=1000 {
    let blockers = calc_blocker_north(&plate);
    plate = apply_blockers(plate, &blockers, (0, 1));
    // println_plate(&plate);
    let blockers = calc_blocker_west(&plate);
    plate = apply_blockers(plate, &blockers, (1, 0));
    // println_plate(&plate);
    let blockers = calc_blocker_south(&plate);
    plate = apply_blockers(plate, &blockers, (0, -1));
    // println_plate(&plate);
    let blockers = calc_blocker_east(&plate);
    plate = apply_blockers(plate, &blockers, (-1, 0));
    // println_plate(&plate);

    let mut sum = 0;
    for ((_, y), count) in blockers {
      sum += count * (plate.len() - y);
    }
    sums.push(sum);
  }

  let last = sums.pop().unwrap();
  let (idx, _) = sums
    .iter()
    .enumerate()
    .rfind(|(_, &it)| it == last)
    .unwrap();

  let repeat_len = sums.len() - idx;
  let len_before_repeat = idx + 1;
  let offset = ((((1000000000 - len_before_repeat) as f64 / repeat_len as f64).fract()) * 18_f64)
    .round() as usize;
  sums[idx + offset].println_dbg()
}

fn parse(input: String) -> Vec<Vec<Item>> {
  input
    .lines()
    .map(|it| {
      it.chars()
        .map(|it| match it {
          'O' => Item::Round,
          '#' => Item::Square,
          '.' => Item::Empty,
          _ => {
            unreachable!()
          }
        })
        .collect_vec()
    })
    .collect_vec()
}

fn apply_blockers(
  plate: Vec<Vec<Item>>,
  blockers: &[((usize, usize), usize)],
  offset: (isize, isize),
) -> Vec<Vec<Item>> {
  let mut plate = plate
    .into_iter()
    .map(|row| {
      row
        .into_iter()
        .map(|it| match it {
          Item::Round => Item::Empty,
          it => it,
        })
        .collect_vec()
    })
    .collect_vec();

  for ((x, y), count) in blockers {
    for i in 0..*count as isize {
      let (x, y) = (
        x.checked_add_signed(offset.0 * i).unwrap(),
        y.checked_add_signed(offset.1 * i).unwrap(),
      );
      plate[y][x] = Item::Round;
    }
  }

  plate
}

fn calc_blocker_north(plate: &[Vec<Item>]) -> Vec<((usize, usize), usize)> {
  let mut blockers = vec![];
  for (y, row) in plate.iter().enumerate() {
    for (x, item) in row.iter().enumerate() {
      let loc = if matches!(item, Item::Square) {
        y.checked_add(1).map(|y| (x, y))
      } else if y == 0 {
        Some((x, y))
      } else {
        None
      };

      if let Some((x, y)) = loc {
        let mut count = 0_usize;
        for row in plate.iter().skip(y) {
          if matches!(row[x], Item::Square) {
            break;
          } else if matches!(row[x], Item::Round) {
            count += 1;
          }
        }
        if count > 0 {
          blockers.push(((x, y), count))
        }
      }
    }
  }
  blockers
}

fn calc_blocker_west(plate: &[Vec<Item>]) -> Vec<((usize, usize), usize)> {
  let mut blockers = vec![];
  for (y, row) in plate.iter().enumerate() {
    for (x, item) in row.iter().enumerate() {
      let loc = if matches!(item, Item::Square) {
        x.checked_add(1).map(|x| (x, y))
      } else if x == 0 {
        Some((x, y))
      } else {
        None
      };

      if let Some((x, y)) = loc {
        let mut count = 0_usize;
        for item in plate[y].iter().skip(x) {
          if matches!(item, Item::Square) {
            break;
          } else if matches!(item, Item::Round) {
            count += 1;
          }
        }
        if count > 0 {
          blockers.push(((x, y), count))
        }
      }
    }
  }
  blockers
}

fn calc_blocker_south(plate: &[Vec<Item>]) -> Vec<((usize, usize), usize)> {
  let mut blockers = vec![];
  for (y, row) in plate.iter().enumerate() {
    for (x, item) in row.iter().enumerate() {
      let loc = if matches!(item, Item::Square) {
        y.checked_add_signed(-1).map(|y| (x, y))
      } else if y == plate.len() - 1 {
        Some((x, y))
      } else {
        None
      };

      if let Some((x, y)) = loc {
        let mut count = 0_usize;
        for row in plate.iter().rev().skip(plate.len() - y - 1) {
          if matches!(row[x], Item::Square) {
            break;
          } else if matches!(row[x], Item::Round) {
            count += 1;
          }
        }
        if count > 0 {
          blockers.push(((x, y), count))
        }
      }
    }
  }
  blockers
}

fn calc_blocker_east(plate: &[Vec<Item>]) -> Vec<((usize, usize), usize)> {
  let mut blockers = vec![];
  for (y, row) in plate.iter().enumerate() {
    for (x, item) in row.iter().enumerate() {
      let loc = if matches!(item, Item::Square) {
        x.checked_add_signed(-1).map(|x| (x, y))
      } else if x == row.len() - 1 {
        Some((x, y))
      } else {
        None
      };

      if let Some((x, y)) = loc {
        let mut count = 0_usize;
        for item in plate[y].iter().rev().skip(row.len() - x - 1) {
          if matches!(item, Item::Square) {
            break;
          } else if matches!(item, Item::Round) {
            count += 1;
          }
        }
        if count > 0 {
          blockers.push(((x, y), count))
        }
      }
    }
  }
  blockers
}
