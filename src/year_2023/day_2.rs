/*
 * Copyright 2023 Golden_Water
 * All rights reserved
 */

use itertools::Itertools;

#[derive(Debug, Default)]
struct CubeSet {
  red: u64,
  green: u64,
  blue: u64,
}

fn parse(input: String) -> Vec<(u64, CubeSet)> {
  input
    .split('\n')
    .map(|it| {
      let it = it.trim_start_matches("Game ");

      let (game_id, sets) = it.split_at(it.find(':').unwrap());
      let game_id = game_id.parse::<u64>().unwrap();

      let color_max = sets[1..]
        .split(';')
        .map(|set| {
          // parse each set
          let mut cube_set = CubeSet::default();
          set[1..]
            .split(',')
            .map(|it| it.trim_start())
            .for_each(|it| {
              // parse each cube
              let (num, color) = it.split_at(it.find(' ').unwrap());
              let num = num.parse::<u64>().unwrap();
              match color.trim_start() {
                "red" => {
                  cube_set.red = num;
                }
                "green" => {
                  cube_set.green = num;
                }
                "blue" => {
                  cube_set.blue = num;
                }
                _ => {
                  unreachable!()
                }
              }
            });
          cube_set
        })
        .fold(CubeSet::default(), |acc, it| {
          // get max of each color
          CubeSet {
            red: acc.red.max(it.red),
            green: acc.green.max(it.green),
            blue: acc.blue.max(it.blue),
          }
        });
      (game_id, color_max)
    })
    .collect_vec()
}

pub fn part_1(input: String) {
  let result = parse(input)
    .iter()
    .filter_map(|(game_id, color_max)| {
      if color_max.red > 12 || color_max.green > 13 || color_max.blue > 14 {
        None
      } else {
        Some(game_id)
      }
    })
    .sum::<u64>();

  println!("{}", result);
}

pub fn part_2(input: String) {
  let result = parse(input)
    .iter()
    .map(|(_, color_max)| color_max.red * color_max.green * color_max.blue)
    .sum::<u64>();

  println!("{}", result);
}
