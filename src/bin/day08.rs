use aoc::read_lines;
use std::collections::HashMap;

#[derive(Debug)]
enum Dir {
    L = 0,
    R = 1,
}

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

    println!("=== Part 1 ===");
    process_part1(&maps, &dirs);
    println!("=== Part 2 ===");
    process_part22(&maps, &dirs);
}

#[allow(dead_code)]
fn process_part1(maps: &HashMap<&str, (&str, &str)>, dirs: &Vec<Dir>) {
    const START: &str = "AAA";
    const GOAL: &str = "ZZZ";

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
        last = next;
    }

    println!("Total amount of steps: {counter}");
}

fn process_part22(maps: &HashMap<&str, (&str, &str)>, dirs: &Vec<Dir>) {
    let positions: Vec<_> = maps.keys().filter(|k| k.ends_with('A')).copied().collect();

    let all_counter: Vec<usize> = positions
        .iter()
        .map(|start| {
            let mut counter: usize = 0;
            let mut last = start;
            while !last.ends_with('Z') {
                let (l, r) = maps.get(last).unwrap();
                let next = match dirs.get(counter % dirs.len()) {
                    Some(Dir::L) => l,
                    Some(Dir::R) => r,
                    _ => panic!("unclear direction"),
                };

                counter += 1;
                last = next;
            }

            println!("({start}) steps: {counter}");
            counter
        })
        .collect();

    let lcm: usize = lcm(&all_counter);
    println!("Total amount of steps: {lcm}");
}

// Code copied from: https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
// Copyright (c) 2019 The Algorithms
// License: MIT
fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

// Code copied from: https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
// Copyright (c) 2019 The Algorithms
// License: MIT
fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
