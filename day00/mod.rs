use std::{env, path};

type Puzzle = dyn Fn() -> ();

pub fn select_puzzle(file: &'static str, puzzle1: &Puzzle, puzzle2: &Puzzle) {
   let mut that_path = path::PathBuf::from(file);
   let copy = that_path.to_owned();
   let filename = copy.file_name().unwrap().to_str().unwrap();
   that_path.pop();
   env::set_current_dir(that_path).unwrap();

   if let Some(selection) = env::args().nth(1) {
      match selection.as_str() {
         "1" => {
            println!("{}::puzzle1", filename);
            puzzle1();
         }
         "2" => {
            println!("{}::puzzle2", filename);
            puzzle2();
         }
         _ => panic!("I don't know what {} means!", selection)
      }
   } else {
      println!("{}", filename);
      puzzle1();
      puzzle2();
   }
}
