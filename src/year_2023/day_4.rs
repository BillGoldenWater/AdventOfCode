/*
 * Copyright 2023 Golden_Water
 * All rights reserved
 */

use itertools::Itertools;

use crate::utils::Then;

struct Card {
  win: Vec<u64>,
  has: Vec<u64>,
  copies: u64,
}

impl Card {
  fn matching_num(&self) -> usize {
    self
      .has
      .iter()
      .filter(move |it| self.win.contains(it))
      .count()
  }
}

fn parse(input: String) -> Vec<Card> {
  input
    .lines()
    .map(|it| it.split(':').nth(1).unwrap())
    .map(|it| it.split('|').collect_tuple::<(_, _)>().unwrap())
    .map(|it| (it.0, it.1.trim_start_matches('|')))
    .map(|(win, has)| {
      [win, has]
        .into_iter()
        .map(|it| {
          it.trim()
            .split(' ')
            .map(str::parse::<u64>)
            .filter_map(Result::ok)
            .collect_vec()
        })
        .collect_tuple::<(_, _)>()
        .unwrap()
    })
    .map(|(win, has)| Card {
      win,
      has,
      copies: 1,
    })
    .collect_vec()
}

pub fn part_1(input: String) {
  parse(input)
    .iter()
    .map(Card::matching_num)
    .map(|win_count| {
      if win_count > 0 {
        2_u64.pow(win_count as u32 - 1)
      } else {
        0
      }
    })
    .sum::<u64>()
    .println()
}

pub fn part_2(input: String) {
  let mut cards = parse(input);

  for idx in 0..cards.len() {
    let matching_num = cards[idx].matching_num();
    let copies = cards[idx].copies;

    cards[idx + 1..=idx + matching_num]
      .iter_mut()
      .for_each(|card| card.copies += copies)
  }

  cards.iter().map(|it| it.copies).sum::<u64>().println()
}
