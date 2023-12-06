/*
 * Copyright 2023 Golden_Water
 * All rights reserved
 */

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use std::sync::{Arc, Mutex, OnceLock, Weak};

use itertools::Itertools;

use crate::utils::{HashMapExt, OnceLockExt, Then};

#[derive(Debug)]
struct Number {
  number: u64,
  counted: AtomicBool,
  counted2: Mutex<Weak<()>>,
}

impl Number {
  pub fn new_uncounted(number: u64) -> Self {
    Self {
      number,
      counted: AtomicBool::new(false),
      counted2: Mutex::new(Weak::new()),
    }
  }
}

#[derive(Debug)]
struct GearInfo {
  /// 0 for none, 1 for one, 2 for two, 3 for more than two
  r#type: AtomicU8,
  ratio: AtomicU64,
}

impl GearInfo {
  pub fn new_none() -> Self {
    Self {
      r#type: AtomicU8::new(0),
      ratio: AtomicU64::new(0),
    }
  }
}

#[derive(Debug)]
enum Symbol {
  Gear(GearInfo),
  Other,
}

#[derive(Debug)]
enum Data {
  Number(Arc<OnceLock<Number>>),
  Symbol(Symbol),
}

impl From<Arc<OnceLock<Number>>> for Data {
  fn from(value: Arc<OnceLock<Number>>) -> Self {
    Self::Number(value)
  }
}

type Board = Box<HashMap<(usize, usize), Data>>;
type NumberAcc = Option<(u64, Arc<OnceLock<Number>>)>;

fn try_end_number(
  board: Board,
  loc: (usize, usize),
  num_acc: NumberAcc,
  new_digit: Option<u64>,
) -> Board {
  if let Some((num, number)) = num_acc {
    if let Some(new_digit) = new_digit {
      (num * 10 + new_digit)
        .then(|num| board.inserted_box(loc, number.inited_arc(Number::new_uncounted(num)).into()))
    } else {
      number
        .inited_arc(Number::new_uncounted(num))
        .then(|_| board)
    }
  } else if let Some(new_digit) = new_digit {
    board.inserted_box(
      loc,
      Arc::new(OnceLock::new())
        .inited_arc(Number::new_uncounted(new_digit))
        .into(),
    )
  } else {
    board
  }
}

fn parse(input: String) -> Board {
  input
    .split('\n')
    .enumerate()
    .fold(Board::default(), |board, (y, line)| {
      // chars
      line
        .char_indices()
        .fold((board, None), |(board, num_acc), (x, ch)| match ch {
          // digit
          digit if digit.is_ascii_digit() => {
            digit
              .to_digit(10)
              .unwrap()
              .then(|it| it as u64)
              .then(|digit| {
                if x + 1 == line.len() {
                  try_end_number(board, (x, y), num_acc, Some(digit)).then(|acc| (acc, None))
                } else if let Some((num, number)) = num_acc {
                  board
                    .inserted_box((x, y), number.clone().into())
                    .then(|board| (board, Some((num * 10 + digit, number))))
                } else {
                  Arc::new(OnceLock::new())
                    .then(|number| (board.inserted_box((x, y), number.clone().into()), number))
                    .then(|(board, number)| (board, Some((digit, number))))
                }
              })
          }
          // symbol
          symbol if symbol != '.' => try_end_number(board, (x, y), num_acc, None)
            .inserted_box(
              (x, y),
              Data::Symbol(if symbol == '*' {
                Symbol::Gear(GearInfo::new_none())
              } else {
                Symbol::Other
              }),
            )
            .then(|board| (board, None)),
          // empty
          _ => try_end_number(board, (x, y), num_acc, None).then(|board| (board, None)),
        })
        .then(|(board, _)| board)
    })
}

pub fn part_1(input: String) {
  parse(input)
    .then(|board| {
      board.iter().for_each(|(&(x, y), data)| match data {
        Data::Number(_) => {}
        Data::Symbol(_) => {
          [-1, 0, 1]
            .into_iter()
            .cartesian_product([-1, 0, 1])
            .for_each(|(off_x, off_y)| {
              if let Some(Data::Number(number)) = board.get(&(
                x.saturating_add_signed(off_x),
                y.saturating_add_signed(off_y),
              )) {
                number.get().unwrap().counted.store(true, Ordering::Relaxed)
              }
            });
        }
      });
      board
        .into_iter()
        .filter_map(|(_, data)| match data {
          Data::Number(number)
            if Arc::strong_count(&number) == 1
              && number.get().unwrap().counted.load(Ordering::Relaxed) =>
          {
            Some(number.get().unwrap().number)
          }
          _ => None,
        })
        .sum::<u64>()
    })
    .println_dbg()
}

pub fn part_2(input: String) {
  parse(input)
    .then(|board| {
      board.iter().for_each(|(&(x, y), data)| match data {
        Data::Number(_) => {}
        Data::Symbol(symbol) => {
          let counted = Arc::new(());
          [-1, 0, 1]
            .into_iter()
            .cartesian_product([-1, 0, 1])
            .for_each(|(off_x, off_y)| {
              if let Some((x, y)) = x
                .checked_add_signed(off_x)
                .and_then(|x| y.checked_add_signed(off_y).map(|y| (x, y)))
              {
                if let Some(Data::Number(number)) = board.get(&(x, y)) {
                  let mut counted2 = number.get().unwrap().counted2.lock().unwrap();
                  if counted2.strong_count() == 0 {
                    *counted2 = Arc::downgrade(&counted);
                    match symbol {
                      Symbol::Gear(gear) => match gear.r#type.load(Ordering::Relaxed) {
                        0 => {
                          gear.r#type.fetch_add(1, Ordering::Relaxed);
                          gear
                            .ratio
                            .store(number.get().unwrap().number, Ordering::Relaxed);
                        }
                        1 => {
                          gear.r#type.fetch_add(1, Ordering::Relaxed);
                          gear.ratio.store(
                            gear.ratio.load(Ordering::Relaxed) * number.get().unwrap().number,
                            Ordering::Relaxed,
                          );
                        }
                        2 => {
                          gear.r#type.fetch_add(1, Ordering::Relaxed);
                        }
                        3 => {}
                        _ => {
                          unreachable!()
                        }
                      },
                      Symbol::Other => {}
                    }
                  }
                }
              }
            });
        }
      });
      board
        .into_iter()
        .filter_map(|(_, data)| match data {
          Data::Symbol(Symbol::Gear(gear)) if gear.r#type.load(Ordering::Relaxed) == 2 => {
            Some(gear.ratio.load(Ordering::Relaxed))
          }
          _ => None,
        })
        .sum::<u64>()
    })
    .println_dbg()
}
