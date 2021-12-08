use std::fs;

pub fn run() {
    let filename = "../inputs/day02.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let line_contents = contents
        .lines()
        .map(|s| {
            let s_split = s.split(" ").collect::<Vec<&str>>();
            return (s_split[0], s_split[1].parse::<i32>().unwrap());
        })
        .collect::<Vec<(&str, i32)>>();

    part1(&line_contents);
    part2(&line_contents);
}

fn part1(line_contents: &Vec<(&str, i32)>) {
    let mut forward_position = 0;
    let mut vertical_position = 0;

    for line_tuple in line_contents {
        if line_tuple.0 == "forward" {
            forward_position = forward_position + line_tuple.1;
        } else if line_tuple.0 == "down" {
            vertical_position = vertical_position + line_tuple.1;
        } else if line_tuple.0 == "up" {
            vertical_position = vertical_position - line_tuple.1;
        }
    }
    println!("Part 2: {:?}", forward_position * vertical_position);
}

fn part2(line_contents: &Vec<(&str, i32)>) {
    let mut forward_position = 0;
    let mut vertical_position = 0;
    let mut aim = 0;
    for line_tuple in line_contents {
        if line_tuple.0 == "forward" {
            forward_position = forward_position + line_tuple.1;
            vertical_position = vertical_position + aim * line_tuple.1;
        } else if line_tuple.0 == "down" {
            aim = aim + line_tuple.1;
        } else if line_tuple.0 == "up" {
            aim = aim - line_tuple.1;
        }
    }
    println!("Part 1: {:?}", forward_position * vertical_position);
}
