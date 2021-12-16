use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn part1() -> String {
    let file = File::open("./day3/src/input_1.txt").expect("could not open file");
    let reader = BufReader::new(file);

    // Store binary
    let mut binary_str_vec: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        binary_str_vec.push(line);
    }

    // Convert str to i64
    let mut binary_vec: Vec<i64> = vec![0; 12];

    for binary_str in binary_str_vec {
        for (index, i) in binary_str.chars().enumerate() {
            if i == '1' {
                binary_vec[index] += 1;
            } else {
                binary_vec[index] -= 1;
            }
        }
    }

    // Store gamma and epsilon
    let mut gamma: Vec<String> = Vec::new();
    let mut epsilon: Vec<String> = Vec::new();

    for number in binary_vec.iter() {
        if number > &0 {
            gamma.push("1".to_string());
            epsilon.push("0".to_string());
        } else {
            gamma.push("0".to_string());
            epsilon.push("1".to_string());
        }
    }

    let gamma = String::from_iter(gamma);
    let epsilon = String::from_iter(epsilon);

    let gamma = isize::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon = isize::from_str_radix(epsilon.as_str(), 2).unwrap();

    String::from(format!("{:?}", epsilon * gamma))
}

pub fn part2() -> String {
    String::from("day3part2")
}
