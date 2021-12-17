use utils::file_loader;

pub fn part1(filename: String) -> String {
    let data = file_loader(filename).expect("Could not parse file");

    let mut x: usize = 0;
    let mut y: usize = 0;

    for line in data.iter() {
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

pub fn part2(filename: String) -> String {
    let data = file_loader(filename).expect("Could not parse file");

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut aim: usize = 0;

    for line in data.iter() {
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
