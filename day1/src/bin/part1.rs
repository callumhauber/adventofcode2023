use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;

fn main() {
  let mut total = 0;
  if let Ok(lines) = read_lines("./input.txt") {
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
      if let Ok(ip) = line {
        total += parse_number(ip);
      }
    }
  }
  println!("{}", total);
}

fn parse_number(line: String) -> i32 {
  // strip down to only numbers
  let chars: Vec<_> = line
    .chars()
    .filter(|c| c.is_numeric())
    .collect();

  let str_size: usize = chars.len();

  let number_str: String = format!("{}{}", chars[0], chars[str_size - 1]);
  let number: i32 = number_str.parse().unwrap();
  return number;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
