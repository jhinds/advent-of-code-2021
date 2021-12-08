use std::{fs, collections::HashMap, io::Empty};

pub fn run() {
  let filename = "../inputs/day03.txt";

  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let line_contents: Vec<&str> = contents
      .lines()
      .collect();
  let count = line_contents.len();
  println!("{}", count);
      part1(&line_contents);
      part2(&line_contents);

}

#[derive(Debug, Default)]
struct Counter {
  state0: HashMap<usize,i32>, // index => 0 count
  state1: HashMap<usize,i32>, // index => 0 count
}

fn calc_gamma(counter: &Counter) -> isize {
  let mut gamma_bin = String::from("");
  for position in 1..13 {
    if counter.state0.get(&position).unwrap() > counter.state1.get(&position).unwrap() {
      gamma_bin.push_str("0");
    } else {
      gamma_bin.push_str("1");
    }
    
  }
  println!("gamma bin: {}", gamma_bin);
  let decimal = isize::from_str_radix(&gamma_bin, 2).unwrap();
  println!("gamma decimal: {}", decimal);
  return decimal;
}

fn calc_epsilon(counter: &Counter) -> isize {
  let mut epsilon_bin = String::from("");
  for position in 1..13 {
    if counter.state1.get(&position).unwrap() < counter.state0.get(&position).unwrap() {
      epsilon_bin.push_str("1");
    } else {
      epsilon_bin.push_str("0");
    }
    
  }
  println!("epsilon bin: {}", epsilon_bin);
  let decimal = isize::from_str_radix(&epsilon_bin, 2).unwrap();
  println!("epsilon decimal: {}", decimal);
  return decimal;

}

impl Counter {
  fn incr0(&mut self, idx: usize) {
    match self.state0.get(&idx) {
      Some(_)=> {self.state0.insert(idx, self.state0.get(&idx).unwrap() + 1);},
      None => {self.state0.insert(idx,  1);},
    }
  }
  fn incr1(&mut self, idx: usize) {
    match self.state1.get(&idx) {
      Some(_)=> {self.state1.insert(idx, self.state1.get(&idx).unwrap() + 1);},
      None => {self.state1.insert(idx,  1);},
    }
  }
  fn calculate_power(self) -> isize{
    let gamma = calc_gamma(&self);
    let epsilon = calc_epsilon(&self);
    return gamma * epsilon;
  }

  fn most_common(self, position: usize) -> isize {
    if self.state1.get(&position).unwrap() > self.state0.get(&position).unwrap() {
      return 1
    } else {
      return 0
    }
  }
}

fn part1(line_contents: &Vec<&str>) { 
  let mut counter = Counter{..Default::default()};
  for line in line_contents {
    for (position, binary) in line.split("").map(|x| {
      match x.parse().unwrap_or(3) {
        0 => 0, 
        1 => 1,
        _ => 3,
      }
    }).collect::<Vec<i32>>().iter().enumerate() {
      // println!("{} {}", position, binary);
      match binary {
        0 => counter.incr0(position),
        1 => counter.incr1(position),
        _ => (),
      }
    }
  }
  println!("{:?}",counter);
  println!("Power: {:?}", &counter.calculate_power());
}
fn part2(line_contents: &Vec<&str>) {
  let mut counter = Counter{..Default::default()};
  for line in line_contents {
    for (position, binary) in line.split("").map(|x| {
      match x.parse().unwrap_or(3) {
        0 => 0, 
        1 => 1,
        _ => 3,
      }
    }).collect::<Vec<i32>>().iter().enumerate() {
      // println!("{} {}", position, binary);
      match binary {
        0 => counter.incr0(position),
        1 => counter.incr1(position),
        _ => (),
      }
    }
  }
}

// fn oxygen_rating(counter: &Counter) {
//   if 
//   return
// }

