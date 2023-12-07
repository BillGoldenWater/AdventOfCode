/*
 * Copyright 2023 Golden_Water
 * All rights reserved
 */

use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};

use itertools::Itertools;

use crate::utils::Then;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
enum Card {
  J,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  T,
  // part1 J loc
  Q,
  K,
  A,
}

impl From<char> for Card {
  fn from(value: char) -> Self {
    match value {
      '2' => Card::Two,
      '3' => Card::Three,
      '4' => Card::Four,
      '5' => Card::Five,
      '6' => Card::Six,
      '7' => Card::Seven,
      '8' => Card::Eight,
      '9' => Card::Nine,
      'T' => Card::T,
      'J' => Card::J,
      'Q' => Card::Q,
      'K' => Card::K,
      'A' => Card::A,
      _ => {
        unreachable!()
      }
    }
  }
}

impl Display for Card {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let char = match self {
      Card::Two => '2',
      Card::Three => '3',
      Card::Four => '4',
      Card::Five => '5',
      Card::Six => '6',
      Card::Seven => '7',
      Card::Eight => '8',
      Card::Nine => '9',
      Card::T => 'T',
      Card::J => 'J',
      Card::Q => 'Q',
      Card::K => 'K',
      Card::A => 'A',
    };
    write!(f, "{}", char)
  }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
enum CardsType {
  HighCard,
  OnePair,
  TwoPair,
  ThreeOfAKind,
  FullHouse,
  FourOfAKind,
  FiveOfAKind,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
struct CardsInfo {
  r#type: CardsType,
  cards: Vec<Card>,
}

impl CardsInfo {
  pub fn new(cards: Vec<Card>) -> Self {
    let mut grouped = cards
      .iter()
      .into_grouping_map_by(|it| **it)
      .fold(0u64, |acc, _, _| acc + 1)
      .into_iter()
      .sorted_by(|(_, count), (_, count_b)| count.cmp(count_b))
      .rev()
      .collect_vec();

    // region part2
    let j_count = grouped
      .iter()
      .enumerate()
      .find(|(_, (c, _))| matches!(c, Card::J));
    if let Some((idx, _)) = j_count {
      if grouped.len() >= 2 {
        let (_, count) = grouped.remove(idx);
        grouped[0].1 += count;
      }
    }
    // endregion

    let r#type = match grouped.len() {
      1 => CardsType::FiveOfAKind,
      2 => {
        if grouped[0].1 == 4 {
          CardsType::FourOfAKind
        } else {
          CardsType::FullHouse
        }
      }
      3 => {
        if grouped[0].1 == 3 {
          CardsType::ThreeOfAKind
        } else {
          CardsType::TwoPair
        }
      }
      4 => CardsType::OnePair,
      _ => CardsType::HighCard,
    };

    Self { r#type, cards }
  }
}

impl Debug for CardsInfo {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Cards: {{ {}, {:?} }}",
      self.cards.iter().map(|it| it.to_string()).join(""),
      self.r#type
    )
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Hand {
  card: CardsInfo,
  bid: u64,
}

impl PartialOrd<Self> for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Hand {
  fn cmp(&self, other: &Self) -> Ordering {
    self.card.cmp(&other.card)
  }
}

pub fn part_1(input: String) {
  input
    .lines()
    .map(|it| {
      it.split_once(' ').unwrap().then(|(cards, bid)| Hand {
        card: CardsInfo::new(cards.chars().map(Card::from).collect_vec()),
        bid: bid.parse().unwrap(),
      })
    })
    .sorted()
    .enumerate()
    .fold(0, |acc, (idx, hand)| acc + (idx as u64 + 1) * hand.bid)
    .println_dbg();
}

pub fn part_2(input: String) {
  // type determent and order of enum Card has changed
  part_1(input);
}
