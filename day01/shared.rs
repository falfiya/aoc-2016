use std::fs;

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
      .filter(Option::is_some)
      .map(Option::unwrap)
      .collect()
}
