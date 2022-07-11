use std::{env, path};

type Puzzle = dyn Fn() -> ();

pub fn select_puzzle(file: &'static str, puzzle1: &Puzzle, puzzle2: &Puzzle) {
   let mut that_path = path::PathBuf::from(file);
   that_path.pop();
   let clone_path = that_path.clone();
   let day = clone_path.file_name().unwrap().to_str().unwrap();
   env::set_current_dir(that_path).unwrap();

   if let Some(selection) = env::args().nth(1) {
      match selection.as_str() {
         "1" => {
            println!("--- {}::puzzle1 ---", day);
            puzzle1();
         }
         "2" => {
            println!("--- {}::puzzle2 ---", day);
            puzzle2();
         }
         _ => panic!("I don't know what {} means!", selection)
      }
   } else {
      println!("--- {}::puzzle1 ---", day);
      puzzle1();
      println!("--- {}::puzzle2 ---", day);
      puzzle2();
   }
}
