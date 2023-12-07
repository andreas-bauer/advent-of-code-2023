use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum = 0;
    let lines_result = read_lines("src/bin/input01.txt");
    let lines = match lines_result {
        Ok(l) => l,
        Err(error) => panic!("Unable to open the file: {:?}", error),
    };

    for line_result in lines {
        if let Ok(line) = line_result {
            let line_digits: Vec<_> = line.chars()
                .filter(|c| c.is_digit(10))
                .collect();

            if line_digits.is_empty() {
                continue;
            }

            let first = line_digits.first().unwrap();
            let last = line_digits.last().unwrap();
            let result: u32 = format!("{}{}", first, last).parse().unwrap();

            sum += result;

            // println!("{}", line);
            // println!("line_digits: {}", line_digits.len());
            // println!("{} + {} = {}", first, last, result)
        }
    }
    println!("Sum: {}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
