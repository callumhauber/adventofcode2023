use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;
// use regex::Regex;

fn main() {
  let mut schematic: Vec<Vec<char>> = Vec::new();

  // read input into 2d array of chars
  if let Ok(lines) = read_lines("./test_input.txt") {
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
      if let Ok(line_result) = line {
        schematic.push(line_result.chars().collect());
      }
    }
  }
  // let mut pos = (0, 0);

  // regex for parsing numbers

  for row in 0..schematic.len() {
    for col in 0..schematic[row].len() {
      print!("{}", schematic[row][col]);
      if schematic[row][col].is_digit(10) {
      }
    }
    println!();
  }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
