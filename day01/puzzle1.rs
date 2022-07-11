use crate::shared;
use Direction::*;
use shared::Instruction::{Left, Right};

#[derive(Debug)]
enum Direction {
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

pub fn puzzle1() {
   let mut dir: Direction = North;
   let mut ns: i64 = 0;
   let mut ew: i64 = 0;
   for i in shared::get_input() {
      let blocks: i64;
      match i {
         Left(n) => {
            dir = dir.left();
            blocks = n;
         }
         Right(n) => {
            dir = dir.right();
            blocks = n;
         }
      }
      match dir {
         North => ns += blocks,
         East  => ew += blocks,
         South => ns -= blocks,
         West  => ew -= blocks,
      }
   }
   println!("ns: {}, ew: {}", ns, ew);
   println!("total: {}", ns.abs() + ew.abs());
}
