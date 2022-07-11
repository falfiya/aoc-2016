use std::fs;
use Direction::*;

#[derive(Debug)]
pub enum Instruction {
   Left(i64),
   Right(i64),
}

impl Instruction {
   pub fn from(s: &str) -> Option<Instruction> {
      if s.starts_with("L") {
         return Some(Instruction::Left(s[1..].parse::<u32>().ok()?.into()));
      }
      if s.starts_with("R") {
         return Some(Instruction::Right(s[1..].parse::<u32>().ok()?.into()));
      }
      return None;
   }
}

pub fn get_input() -> Vec<Instruction> {
   fs::read_to_string("input.txt")
      .unwrap()
      .trim()
      .split(", ")
      .map(Instruction::from)
      .map(Option::unwrap)
      .collect()
}

#[derive(Debug)]
pub enum Direction {
   North,
   East,
   South,
   West,
}

impl Direction {
   pub fn left(&self) -> Direction {
      match *self {
         North => West,
         East  => North,
         South => East,
         West  => South,
      }
   }
   pub fn right(&self) -> Direction {
      match *self {
         North => East,
         East  => South,
         South => West,
         West  => North,
      }
   }
}
