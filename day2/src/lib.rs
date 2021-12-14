use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn part1() -> String {
    let file = File::open("./day2/src/input_1.txt").expect("could not open file");
    let reader = BufReader::new(file);

    let mut x: usize = 0;
    let mut y: usize = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let v: Vec<&str> = line.split(' ').collect();
        match v[0] {
            "forward" => x += v[1].parse::<usize>().unwrap(),
            "down" => y += v[1].parse::<usize>().unwrap(),
            "up" => y -= v[1].parse::<usize>().unwrap(),
            _ => continue,
        }
    }
    String::from(format!("{}", x * y))
}

pub fn part2() -> String {
    let file = File::open("./day2/src/input_2.txt").expect("could not open file");
    let reader = BufReader::new(file);

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut aim: usize = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let v: Vec<&str> = line.split(' ').collect();
        match v[0] {
            "forward" => {
                x += v[1].parse::<usize>().unwrap();
                if aim != 0 {
                    y += v[1].parse::<usize>().unwrap() * aim
                }
            }
            "down" => aim += v[1].parse::<usize>().unwrap(),
            "up" => aim -= v[1].parse::<usize>().unwrap(),
            _ => continue,
        }
    }

    String::from(format!("{}", x * y))
}
