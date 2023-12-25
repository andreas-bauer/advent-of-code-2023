use std::collections::HashMap;

use aoc::read_lines;

#[derive(Debug)]
enum Dir {
    L = 0,
    R = 1,
}

const START: &str = "AAA";
const GOAL: &str = "ZZZ";

fn main() {
    let lines = match read_lines("input/day08.txt") {
        Ok(l) => l,
        Err(error) => panic!("Unable to open the file: {:?}", error),
    };

    let lines: Vec<String> = lines.flatten().collect();

    let dirs: Vec<Dir> = lines
        .first()
        .unwrap()
        .chars()
        .filter_map(|c| match c {
            'L' => Some(Dir::L),
            'R' => Some(Dir::R),
            _ => None,
        })
        .collect();

    let mut maps: HashMap<&str, (&str, &str)> = HashMap::with_capacity(5);

    for line in lines.iter().skip(2) {
        let (id, dirs) = line.split_once(" = ").unwrap();
        let (l, r) = dirs
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();

        maps.insert(id, (l, r));
    }

    let mut counter: usize = 0;
    let mut last = START;
    while last != GOAL {
        let (l, r) = maps.get(last).unwrap();
        let next = match dirs.get(counter % dirs.len()) {
            Some(Dir::L) => l,
            Some(Dir::R) => r,
            _ => panic!("unclear direction"),
        };

        counter += 1;
        println!("{counter}: {last} => {next}");
        last = next;
    }

    println!("Total amount of steps: {counter}");
}
