use utils::file_loader;

pub fn part1(filename: String) -> String {
    let data = file_loader(filename).expect("Could not parse file");

    // Convert str to i64
    let mut result: Vec<i64> = vec![0; data[0].len()];

    for binary in data {
        for (index, i) in binary.chars().enumerate() {
            if i == '1' {
                result[index] += 1;
            } else {
                result[index] -= 1;
            }
        }
    }

    // Store gamma and epsilon
    let mut gamma: Vec<String> = Vec::new();
    let mut epsilon: Vec<String> = Vec::new();

    for number in result.iter() {
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

    let result = epsilon * gamma;

    String::from(format!("{}", result))
}

pub fn part2(filename: String) -> String {
    // load data from file
    let file = file_loader(filename).expect("Could not parse file");

    let oxygen = search(file.clone(), 0, "most");
    let oxygen = String::from_iter(oxygen);
    let oxygen =
        isize::from_str_radix(oxygen.as_str(), 2).expect("Could not parse int from binary");

    let co2 = search(file, 0, "least");
    let co2 = String::from_iter(co2);
    let co2 = isize::from_str_radix(co2.as_str(), 2).expect("Could not parse int from binary");

    String::from(format!("{}", oxygen * co2))
}

fn search(data: Vec<String>, bit_position: usize, target: &str) -> Vec<String> {
    if data.len() <= 1 {
        return data;
    }

    let (mut ones, mut zeroes) = (0, 0);
    for line in &data {
        if let Some(c) = line.chars().nth(bit_position) {
            if c == '1' {
                ones += 1
            } else {
                zeroes += 1
            }
        }
    }

    let mut v: Vec<String> = Vec::new();

    for line in &data {
        if let Some(c) = line.chars().nth(bit_position) {
            // Looking for most common
            if (target == "most") && (ones > zeroes) && (c == '1') {
                v.push(line.clone())
            } else if (target == "most") && (ones < zeroes) && (c == '0') {
                v.push(line.clone())
            } else if (target == "most") && (ones == zeroes) && (c == '1') {
                v.push(line.clone())
            }
            // Looking for least common
            else if (target == "least") && (ones > zeroes) && (c == '0') {
                v.push(line.clone())
            } else if (target == "least") && (ones < zeroes) && (c == '1') {
                v.push(line.clone())
            } else if (target == "least") && (ones == zeroes) && (c == '0') {
                v.push(line.clone())
            }
        }
    }
    return search(v, bit_position + 1, &target);
}
