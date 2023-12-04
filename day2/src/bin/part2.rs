use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

fn main() {
  let mut total: u32 = 0;
  if let Ok(lines) = read_lines("./input.txt") {
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
      if let Ok(ip) = line {
        total += game_power(ip);
      }
    }
  }
  println!("{}", total);
}

fn game_power(line: String) -> u32 {
  // println!("---------------------------");
  // println!("{}", line);
  let game_regex = Regex::new(r"Game \d+: (.+)").unwrap();
  let game_regex_result = game_regex.captures(&line);
  // recording max values
  let mut max_vals: HashMap<String, u32> = HashMap::from([
    ("red".to_string(), 0),
    ("green".to_string(), 0),
    ("blue".to_string(), 0),
  ]);

  match game_regex_result {
    Some(value) => {
      let (_, [game_record]) = value.extract();

      // now split games apart for parsing
      let cubes_regex = Regex::new(r"(\d+) (\w+)").unwrap();
      let split_regex = Regex::new(r", |; ").unwrap();
      // first, split each game into a vector of game tokens
      let game_tokens: Vec<String> = split_regex
        .split(game_record)
        .map(|x| x.to_string())
        .collect();

      // for each game token, split into a number and a color
      // and record if that number is greater than the current max
      for game_token in game_tokens {
        // println!("game round: {}", game_token);

        let cubes_result = cubes_regex.captures(&game_token);
        match cubes_result {
          Some(value) => {
            // extract & parse to number
            let (_str, [cube_num_str, cube_color]) = value.extract();
            let cube_num: u32 = cube_num_str.parse().unwrap();
            // println!("{}: {} {}", str, cube_num, cube_color);

            let max_val = max_vals.get(cube_color).unwrap().clone();
            // println!("{} max = {}", cube_color, max_val);

            // update max if it's greater
            if max_val < cube_num {
              max_vals.insert(cube_color.to_string(), cube_num);
              // println!("{} < {}, updating max", max_val, cube_num);
            } else {
              // println!("{} >= {}, continue", max_val, cube_num);
            }
          }
          None => {
            println!("error");
          }
        }
      }

      // println!();
    }
    None => {
      println!("error");
    }
  }

  let red = max_vals.get("red").unwrap();
  let green = max_vals.get("green").unwrap();
  let blue = max_vals.get("blue").unwrap();

  // println!("maxes found: ");
  // println!("red: {}, green: {}, blue: {}", red, green, blue);

  // return game power
  return red * green * blue;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
