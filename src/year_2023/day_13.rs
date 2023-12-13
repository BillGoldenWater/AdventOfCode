/*
 * Copyright 2023 Golden_Water
 * All rights reserved
 */

use itertools::Itertools;

use crate::utils::Then;

pub fn part_1(input: String) {
  let patterns = parse(input);

  let mut result = 0;
  for pattern in patterns {
    let (vert, _) = find_vertical(&pattern);
    let (horizontal, _) = find_horizontal(&pattern);
    result += vert + horizontal * 100;
  }
  result.println_dbg();
}

pub fn part_2(input: String) {
  let patterns = parse(input);

  let mut result = 0;
  for pattern in patterns {
    let (_, vert_fixed) = find_vertical(&pattern);
    let (_, horizontal) = find_horizontal(&pattern);
    result += if vert_fixed > 0 {
      vert_fixed
    } else {
      horizontal * 100
    };
  }
  result.println_dbg();
}

fn parse(input: String) -> Vec<Vec<Vec<bool>>> {
  input
    .split("\n\n")
    .map(|pattern| {
      pattern
        .lines()
        .map(|line| line.chars().map(|it| matches!(it, '#')).collect_vec())
        .collect_vec()
    })
    .collect_vec()
}

pub fn find_vertical(pattern: &[Vec<bool>]) -> (usize, usize) {
  let mut result = (0, 0);
  let len = pattern[0].len();
  for idx in 0..(len - 1) {
    fn check_column_eq(pattern: &[Vec<bool>], idx: usize, to_idx: usize) -> i8 {
      let mut diff = 0;
      for line in pattern {
        if line[idx] != line[to_idx] {
          diff += 1;
          if diff > 1 {
            break;
          }
        }
      }

      if diff > 1 {
        -1
      } else {
        diff
      }
    }

    let diff = check_column_eq(pattern, idx, idx + 1);
    if diff != -1 {
      let mut diff = diff;
      let mut left = idx as isize - 1;
      let mut right = idx as isize + 2;
      let mut non_perfect = false;
      while left >= 0 && right <= (len as isize - 1) {
        let diff2 = check_column_eq(pattern, left as usize, right as usize);
        if diff2 == -1 || diff2 + diff > 1 {
          non_perfect = true;
          break;
        }
        diff += diff2;
        left -= 1;
        right += 1;
      }
      if !non_perfect {
        if diff > 0 {
          result.1 = idx + 1;
        } else if result.0 == 0 {
          result.0 = idx + 1;
        }
      }
    }
  }

  result
}

pub fn find_horizontal(pattern: &[Vec<bool>]) -> (usize, usize) {
  let mut result = (0, 0);

  let len = pattern.len();
  for idx in 0..(len - 1) {
    fn check_row_eq(pattern: &[Vec<bool>], idx: usize, to_idx: usize) -> i8 {
      let mut diff = 0;
      let from = &pattern[idx];
      let to = &pattern[to_idx];
      for (idx, &value) in from.iter().enumerate() {
        if value != to[idx] {
          diff += 1;
          if diff > 1 {
            break;
          }
        }
      }
      if diff > 1 {
        -1
      } else {
        diff
      }
    }

    let diff = check_row_eq(pattern, idx, idx + 1);
    if diff != -1 {
      let mut diff = diff;
      let mut top = idx as isize - 1;
      let mut bottom = idx + 2;
      let mut non_perfect = false;
      while top >= 0 && bottom <= (len - 1) {
        let diff2 = check_row_eq(pattern, top as usize, bottom);
        if diff2 == -1 || diff2 + diff > 1 {
          non_perfect = true;
          break;
        }
        diff += diff2;
        top -= 1;
        bottom += 1;
      }
      if !non_perfect {
        if diff > 0 {
          result.1 = idx + 1;
          continue; // for idea's code duplicate check
        } else if result.0 == 0 {
          result.0 = idx + 1;
        }
      }
    }
  }

  result
}
