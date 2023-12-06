use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;
// use regex::Regex;
enum CHECK {
  START,
  END,
}

struct Point {
  row: usize,
  col: usize,
}

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

  let row_max: usize = schematic.len();
  let col_max: usize = schematic[0].len();

  let mut buf: String = String::new();
  let mut cc: char;
  let mut in_part: bool = false;
  let mut skip_part: bool = false;
  for row in 0..row_max {
    for col in 0..col_max {
      cc = schematic[row][col];
      print!("{}", cc);
      let is_cc_digit = cc.is_digit(10);
      if is_cc_digit {
        buf.push(cc);
      } else if
        // the next char is not a number, so we need to do our last checks
        // & decide if it's a real part number
        !is_cc_digit &&
        buf.len() > 0 &&
        check_around(&schematic, Point { row, col }, row_max, col_max, CHECK::END)
      {
      }
    }
    println!();
  }
}

fn check_around(
  schematic: &Vec<Vec<char>>,
  pos: Point,
  row_max: usize,
  col_max: usize,
  check: CHECK
) -> bool {
  // always check above & below
  if pos.row >= 1 && is_symbol(schematic[pos.row - 1][pos.col]) {
    return true;
  }

  if pos.row + 1 < row_max && is_symbol(schematic[pos.row + 1][pos.col]) {
    return true;
  }

  match check {
    // check behind up/middle/down for start
    CHECK::START => {
      if pos.row >= 1 && pos.col >= 1 && is_symbol(schematic[pos.row - 1][pos.col - 1]) {
        return true;
      }

      if pos.col >= 1 && is_symbol(schematic[pos.row][pos.col - 1]) {
        return true;
      }

      if pos.row + 1 < row_max && pos.col >= 1 && is_symbol(schematic[pos.row + 1][pos.col - 1]) {
        return true;
      }
    }
    // check middle for end
    CHECK::END => {
      return is_symbol(schematic[pos.row][pos.col]);
    }
  }

  // if none of the checked chars were symbols
  return false;
}

fn is_symbol(char: char) -> bool {
  return !char.is_digit(10) && char != '.';
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
