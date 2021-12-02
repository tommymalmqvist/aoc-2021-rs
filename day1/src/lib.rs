use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn day_1() -> String {
    let file = File::open("./day1/src/input.txt").expect("could not open file");
    let reader = BufReader::new(file);

    let mut counter: i64 = 0;
    let mut last: i64 = 0;

    for (index, line) in reader.lines().enumerate() {
        // parse line to i64
        let line = line.unwrap();
        let line = line.parse::<i64>().expect("Could not parse i64");

        // only do stuff when it's NOT the first line
        if index != 0 {
            // compare to previous number
            if line > last {
                counter += 1;
            }
        }
        last = line
    }

    counter.to_string()
}
