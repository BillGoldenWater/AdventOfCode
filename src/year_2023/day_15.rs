/*
 * Copyright 2023 Golden_Water
 * All rights reserved
 */

use itertools::Itertools;

use crate::utils::Then;

pub fn part_1(input: String) {
  input
    .lines()
    .next()
    .unwrap()
    .split(',')
    .map(hash)
    .sum::<usize>()
    .println_dbg();
}

pub fn part_2(input: String) {
  let mut map = (0..256).map(|_| Vec::<(&str, u64)>::new()).collect_vec();

  let items = input.lines().next().unwrap().split(',').collect_vec();

  for item in items {
    let (op_idx, op) = item
      .chars()
      .find_position(|it| ['-', '='].contains(it))
      .unwrap();
    let label = &item[..op_idx];
    let label_hash = hash(label);
    let slots = &mut map[label_hash];
    let exists = slots.iter_mut().find_position(|it| it.0 == label);

    match op {
      '-' => {
        if let Some((idx, _)) = exists {
          slots.remove(idx);
        }
      }
      '=' => {
        let v = item.chars().nth(op_idx + 1).unwrap().to_digit(10).unwrap() as u64;
        if let Some((_, value)) = exists {
          value.1 = v;
        } else {
          slots.push((label, v))
        }
      }
      _ => {
        unreachable!()
      }
    }
  }

  map
    .iter()
    .enumerate()
    .flat_map(|(box_id, it)| {
      it.iter()
        .enumerate()
        .map(|(slot_id, value)| (box_id, slot_id, value.1))
        .collect_vec()
    })
    .map(|(box_id, slot_id, focal_len)| (box_id + 1) * (slot_id + 1) * focal_len as usize)
    .sum::<usize>()
    .println_dbg();
}

fn hash(item: &str) -> usize {
  let mut current = 0;
  for ch in item.chars() {
    let code = ch as usize;
    current += code;
    current *= 17;
    current %= 256;
  }
  current
}
