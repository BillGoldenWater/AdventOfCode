/*
 * Copyright 2023 Golden_Water
 * All rights reserved
 */
#![warn(missing_debug_implementations)]

use std::io::Read;

pub mod utils;
pub mod year_2023;

fn main() -> anyhow::Result<()> {
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input)?;

  year_2023::day_12::part_2(input);

  Ok(())
}
