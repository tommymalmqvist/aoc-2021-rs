use utils::file_loader;

pub fn part1(filename: String) -> String {
    let data = file_loader(filename).expect("Could not parse file");

    let mut counter: i64 = 0;
    let mut last: i64 = 0;

    for (index, line) in data.iter().enumerate() {
        // parse line to i64
        let line = line.parse::<i64>().expect("Could not parse number");

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

pub fn part2(filename: String) -> String {
    let data = file_loader(filename).expect("Could not parse file");

    let mut numbers = Vec::new();

    // convert String to i64
    for line in data.iter() {
        let number = line.parse::<i64>().expect("Could not parse number");
        numbers.push(number);
    }

    let mut row: usize = 0;
    let mut series: Vec<i64> = Vec::new();

    while row + 3 <= numbers.len() {
        let sum = numbers[row + 0] + numbers[row + 1] + numbers[row + 2];

        series.push(sum);

        row += 1;
    }

    // Calculate changes
    let mut increases: i64 = 0;
    let mut previous: i64 = 0;
    for (index, number) in series.iter().enumerate() {
        if index == 0 {
            previous = *number;
        } else if number > &previous {
            increases += 1;
            previous = *number;
        } else {
            previous = *number;
        }
    }

    String::from(format!("{}", increases))
}
