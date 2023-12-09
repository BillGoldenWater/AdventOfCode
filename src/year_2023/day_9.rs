/*
 * Copyright 2023 Golden_Water
 * All rights reserved
 */

use itertools::Itertools;

use crate::utils::Then;

fn parse(input: String) -> Vec<Vec<i64>> {
  input
    .lines()
    .map(|it| {
      it.split(' ')
        .map(|it| it.parse::<i64>().unwrap())
        .collect_vec()
    })
    .collect_vec()
}

fn gen_diffs(history: Vec<i64>) -> Vec<Vec<i64>> {
  let mut diffs = vec![];
  let mut diff = history.clone();
  loop {
    diff = diff.windows(2).map(|it| it[1] - it[0]).collect_vec();
    diffs.push(diff.clone());
    if !diff.iter().any(|&it| it != 0) {
      break;
    }
  }

  diffs.reverse();
  diffs.push(history);
  diffs
}

pub fn part_1(input: String) {
  let histories = parse(input);

  let mut result = 0;

  for history in histories {
    let diffs = gen_diffs(history);

    let mut cur = *diffs[0].last().unwrap();
    for diff in diffs.iter().skip(1) {
      cur += diff.last().unwrap();
    }

    result += cur;
  }

  result.println();
}

pub fn part_2(input: String) {
  let histories = parse(input);

  let mut result = 0;

  for history in histories {
    let diffs = gen_diffs(history);

    let mut cur = diffs[0][0];
    for diff in diffs.iter().skip(1) {
      cur = diff[0] - cur;
    }

    result += cur;
  }

  result.println();
}
