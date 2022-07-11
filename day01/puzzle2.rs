use crate::shared;
use shared::{Instruction, Direction};
use Direction::*;
use Instruction::*;
use std::collections::HashSet;

type Coord = (i64, i64);

pub fn puzzle2() {
   let mut dir: Direction = North;
   let mut x: i64 = 0;
   let mut y: i64 = 0;
   let mut visited: HashSet<Coord> = HashSet::new();
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
      for _ in 0..blocks {
         match dir {
            North => y += 1,
            East  => x += 1,
            South => y -= 1,
            West  => x -= 1,
         }
         let first_time = visited.insert((x, y));
         if !first_time {
            println!("have already walked over {:?}", (x, y));
            println!("distance from origin: {}", x.abs() + y.abs());
            return;
         }
      }
   }
   panic!("No intersection was crossed twice!");

}
