use std::fs;
use std::cmp;

const IDX_SEED: usize = 0;

#[derive(Debug)]
struct Converter {
    range_start: i64,
    range_end: i64,
    offset: i64,
}

#[derive(Debug)]
struct PipeStep {
    _id: usize,
    converters: Vec<Converter>,
}

fn main() {
    let file = match fs::read_to_string("input/day05.txt") {
        Ok(content) => content,
        Err(error) => panic!("Unable to open the file: {:?}", error),
    };

    let input_parts:Vec<&str> = file.split("\n\n").collect::<Vec<_>>();

    let seeds: Vec<i64> = input_parts.get(IDX_SEED).unwrap()
        .split_whitespace()
        .skip(1)
        .map(|c| c.parse::<i64>().unwrap())
        .collect();

    let mut pipeline: Vec<PipeStep> = Vec::with_capacity(7);
    for idx in 1usize..=7usize {
        pipeline.push(
            PipeStep {
                _id: idx,
                converters: parts_to_converters(input_parts.get(idx).unwrap())
            }
        )
    }

    let mut lowest: i64 = i64::MAX;
    for seed in seeds {
        let mut loc: i64 = seed;
        for step in &pipeline {
            loc = apply_converts(loc, &step.converters);
        }
        lowest = cmp::min(lowest, loc);
        println!("Input seed = {} output pos = {}",seed,loc);
    }

    println!("Lowest location = {lowest}");
}

fn parts_to_converters(input: &&str) -> Vec<Converter> {
    input.lines()
        .skip(1)
        .map(|line| {
            let num: Vec<_> = line.split_whitespace()
            .map(|c| c.parse::<i64>().unwrap())
            .collect();

            Converter {
                range_start: num[1],
                range_end: num[1] + num[2] -1,
                offset: num[0] - num[1],
            }
        })
        .collect()
}

fn apply_converts(input: i64, converts: &Vec<Converter>) -> i64 {
    for c in converts {
        if input >= c.range_start && input <= c.range_end {
            return input + c.offset;
        }
    }

    input
}
