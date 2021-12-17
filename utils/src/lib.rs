use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn file_loader(path: String) -> Result<Vec<String>, io::Error> {
    let mut result: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(data) = line {
                result.push(data)
            }
        }
    }

    Ok(result)
}
