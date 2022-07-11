#[path = "../day00/mod.rs"]
mod day00;
mod puzzle1;
mod puzzle2;
mod shared;

fn main() {
   day00::select_puzzle(
      file!(),
      &puzzle1::puzzle1,
      &puzzle2::puzzle2
   );
}
