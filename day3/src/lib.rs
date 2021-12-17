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

pub fn part2() -> String {
    /*
    input: input_1.txt

    - To find oxygen generator rating, determine the most common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 1 in the position being considered.

    - To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 0 in the position being considered.

    Examples:
    --------
    Bit position 1, most common is 1 for O-rating and 0 for C02-rating
    [1]01011111100
    [1]11011001000
    [0]11110010101
    [0]10011000110

    Bit position 2, most common is 0 for O-rating and 0 for C02-rating
    1[0]1011111100
    1[1]1011001000
    0[1]1110010101
    0[1]0011000110

    */

    // let mut o_rating: i64 = 0;
    // let mut c02_rating: i64 = 0;

    String::from("day3part2")
}
