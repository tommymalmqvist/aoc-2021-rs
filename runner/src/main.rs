use day1;
use day2;
use day3;

fn main() {
    let d1_p1 = day1::part1(String::from("./day1/src/input_1.txt"));
    println!("Solution d1p1: {}", d1_p1);

    let d1_p2 = day1::part2(String::from("./day1/src/input_2.txt"));
    println!("Solution d1p2: {}", d1_p2);

    let d2_p1 = day2::part1(String::from("./day2/src/input_1.txt"));
    println!("Solution d2p1: {}", d2_p1);

    let d2_p2 = day2::part2(String::from("./day2/src/input_2.txt"));
    println!("Solution d2p2: {}", d2_p2);

    let d3_p1 = day3::part1(String::from("./day3/src/input_1.txt"));
    println!("Solution d3p1: {}", d3_p1);

    let d3_p2 = day3::part2();
    println!("Solution d3p2: {}", d3_p2);
}
