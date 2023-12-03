use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;
use fancy_regex::Regex;
use std::collections::HashMap;
use once_cell::sync::Lazy;

static NUMBERS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
  return HashMap::from([
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
  ]);
});

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
  let re = Regex::new(
    r".*?(?=(one|two|three|four|five|six|seven|eight|nine|\d))(?:.*(one|two|three|four|five|six|seven|eight|nine|\d).*?)+"
  ).unwrap();

  // let mut results = vec![];
  let regex: Result<Option<fancy_regex::Captures<'_>>, fancy_regex::Error> = re.captures(&line);
  // println!("{}", line);

  match regex {
    Ok(Some(value)) => {
      // let mat = value.get(0).unwrap().as_str();
      let first = value.get(1).unwrap().as_str();
      let second = value.get(2).unwrap().as_str();

      // attempt to convert first & second to actual numbers
      let first_num;
      let second_num;

      match NUMBERS.get(first) {
        Some(&num) => {
          first_num = num;
        }
        None => {
          first_num = first;
        }
      }

      match NUMBERS.get(second) {
        Some(&num) => {
          second_num = num;
        }
        None => {
          second_num = second;
        }
      }

      let number_str: String = format!("{}{}", first_num, second_num);
      let number: i32 = number_str.parse().unwrap();
      // println!("{}: {} + {}", mat, first, second);
      // println!("({}, {}): {}\n", first_num, second_num, number);

      return number;
    }
    Ok(None) => {
      return 0;
    }
    Err(err) => {
      println!("{}", err);
      return 0;
    }
  }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
