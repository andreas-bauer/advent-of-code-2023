use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines_result = read_lines("input/day01.txt");
    let lines = match lines_result {
        Ok(l) => l,
        Err(error) => panic!("Unable to open the file: {:?}", error),
    };

    let mut sum1: u32 = 0;
    let mut sum2: u32 = 0;

    for line in lines.into_iter().flatten() {
        // Part 1
        let first_c = line.chars().find(|c| c.is_ascii_digit()).unwrap();
        let last_c = line.chars().rfind(|c| c.is_ascii_digit()).unwrap();
        let result: u32 = format!("{}{}", first_c, last_c).parse().unwrap();
        sum1 += result;

        // Part 2
        let fix = line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "th3ee")
            .replace("four", "f4ur")
            .replace("five", "fi5e")
            .replace("six", "s6x")
            .replace("seven", "se7en")
            .replace("eight", "ei8ht")
            .replace("nine", "n9ne")
            .replace("zero", "z0ro");

        let first_c = fix.chars().find(|c| c.is_ascii_digit()).unwrap();
        let last_c = fix.chars().rfind(|c| c.is_ascii_digit()).unwrap();
        let result: u32 = format!("{}{}", first_c, last_c).parse().unwrap();

        sum2 += result;
    }

    println!("Sum part 1: {}", sum1);
    println!("Sum part 2: {}", sum2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
