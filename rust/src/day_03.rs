use std::{collections::HashMap, fs, io::Empty};
fn main() {
    let bin_idx = "10110";
    let decimal = isize::from_str_radix(bin_idx, 2).unwrap();
    println!("{:?}", decimal);
    let filename = "../inputs/day03.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let line_contents: Vec<&str> = contents.lines().collect();
    let count = line_contents.len();
    println!("{}", count);
    part1(&line_contents);
    part2(&line_contents);
}

#[derive(Debug, Default)]
struct Counter0 {
    state: HashMap<usize, i32>, // index => 0 count
}

fn calc_gamma(state: HashMap<usize, i32>) -> isize {
    let mut gamma_bin = String::from("");
    for position in 1..12 {
        if state.get(&position).unwrap().clone() > 500 {
            // let mut bin= self.state.get(&position).unwrap().to_string();
            &gamma_bin.push_str("0");
        } else {
            &gamma_bin.push_str("1");
        }
    }
    println!("gamma bin: {}", gamma_bin);
    let decimal = isize::from_str_radix(&gamma_bin, 2).unwrap();
    println!("gamma decimal: {}", decimal);
    return decimal;
}

fn calc_epsilon(state: HashMap<usize, i32>) -> isize {
    let mut epsilon_bin = String::from("");
    for position in 1..12 {
        if state.get(&position).unwrap().clone() > 500 {
            // let mut bin= self.state.get(&position).unwrap().to_string();
            &epsilon_bin.push_str("1");
        } else {
            &epsilon_bin.push_str("0");
        }
    }
    println!("epsilon bin: {}", epsilon_bin);
    let decimal = isize::from_str_radix(&epsilon_bin, 2).unwrap();
    println!("epsilon decimal: {}", decimal);
    return decimal;
}

impl Counter0 {
    fn incr(&mut self, idx: usize) {
        match self.state.get(&idx) {
            Some(_) => {
                self.state.insert(idx, self.state.get(&idx).unwrap() + 1);
            }
            None => {
                self.state.insert(idx, 1);
            }
        }
    }

    fn calculate_power(self) -> isize {
        let gamma = calc_gamma(self.state);
        let epsilon = calc_epsilon(&self.state);
        return gamma * epsilon;
    }
}

fn part1(line_contents: &Vec<&str>) {
    let mut counter = Counter0 {
        ..Default::default()
    };
    let zero = String::from("0");
    let one = String::from("1");
    for line in line_contents {
        for (position, binary) in line
            .split("")
            .map(|x| match x.parse().unwrap_or(3) {
                0 => 0,
                1 => 1,
                _ => 3,
            })
            .collect::<Vec<i32>>()
            .iter()
            .enumerate()
        {
            // println!("{} {}", position, binary);
            match binary {
                0 => counter.incr(position),
                1 => (),
                _ => (),
            }
        }
    }
    println!("{:?}", counter);
    &counter.calculate_power();
}
fn part2(line_contents: &Vec<&str>) {}
