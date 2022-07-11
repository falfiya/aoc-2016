use crate::shared;
use shared::{Instruction, Direction};
use Direction::*;
use Instruction::*;

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
