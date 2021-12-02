use std::fs;

fn main() {
    let filename = "../inputs/day1.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let line_contents: Vec<i32> = contents
        .lines()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    println!("Part 1: {:?}", part1(&line_contents));
    println!("Part 2: {:?}", part2(&line_contents));
}

fn part1(line_contents: &Vec<i32>) -> usize {
    line_contents.windows(2).filter(|x| x[1] > x[0]).count()
}

fn part2(line_contents: &Vec<i32>) -> usize {
    line_contents
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|x| x[1] > x[0])
        .count()
}
