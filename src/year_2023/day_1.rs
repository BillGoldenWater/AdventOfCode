/*
 * Copyright 2023 Golden_Water
 * All rights reserved
 */

use std::cmp::Ordering;

use itertools::Itertools;

pub fn part_1(input: String) {
  let sum = input
    .split('\n')
    .map(|it| {
      let first = it
        .chars()
        .find(char::is_ascii_digit)
        .unwrap()
        .to_digit(10)
        .unwrap();
      let last = it
        .chars()
        .rfind(char::is_ascii_digit)
        .unwrap()
        .to_digit(10)
        .unwrap();
      first * 10 + last
    })
    .sum::<u32>();

  println!("{sum}");
}

pub fn part_2(input: String) {
  const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
  ];

  let sum = input
    .split('\n')
    .map(|it| {
      let first = it
        .char_indices()
        .find(|(_, it)| it.is_ascii_digit())
        .map(|(idx, it)| (idx, it.to_digit(10).unwrap()));
      let last = it
        .char_indices()
        .rfind(|(_, it)| it.is_ascii_digit())
        .map(|(idx, it)| (idx, it.to_digit(10).unwrap()));

      let first_str = DIGITS
        .iter()
        .enumerate()
        .filter_map(|(idx, digit_str)| it.find(digit_str).map(|it| (it, idx + 1)))
        .sorted_unstable_by_key(|it| it.0)
        .next();
      let last_str = DIGITS
        .iter()
        .enumerate()
        .filter_map(|(idx, digit_str)| it.rfind(digit_str).map(|it| (it, idx + 1)))
        .sorted_unstable_by_key(|it| it.0)
        .next_back();

      fn choose(
        digit: Option<(usize, u32)>,
        word: Option<(usize, usize)>,
        ordering: Ordering,
      ) -> u32 {
        match (digit, word) {
          (Some((idx, v)), Some((str_idx, str_v))) => {
            if idx.cmp(&str_idx) == ordering {
              v
            } else {
              str_v as u32
            }
          }
          (None, Some((_, v))) => v as u32,
          (Some((_, v)), None) => v,
          _ => {
            unreachable!()
          }
        }
      }

      let first = choose(first, first_str, Ordering::Less);
      let last = choose(last, last_str, Ordering::Greater);

      first * 10 + last
    })
    .sum::<u32>();

  println!("{sum}");
}
