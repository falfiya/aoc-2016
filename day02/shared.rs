use std::fs;

#[derive(Debug)]
pub enum Direction {U, D, L, R}
use Direction::*;
impl Direction {
   pub fn from(c: char) -> Direction {
      match c {
         'U' => U,
         'D' => D,
         'L' => L,
         'R' => R,
         _ => panic!("I don't know what {} is!", c)
      }
   }
}

pub fn get_input() -> Vec<Vec<Direction>> {
   fs::read_to_string("input.txt")
      .unwrap()
      .lines()
      .map(|line| line.chars().map(Direction::from).collect())
      .collect()
}

