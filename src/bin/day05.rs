use std::fs;

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

    let input_parts: Vec<&str> = file.split("\n\n").collect::<Vec<_>>();

    println!("=== Part 1 ===");
    let seeds: Vec<i64> = input_parts
        .first()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|c| c.parse::<i64>().unwrap())
        .collect();

    let mut pipeline: Vec<PipeStep> = Vec::with_capacity(7);
    for idx in 1usize..=7usize {
        pipeline.push(PipeStep {
            _id: idx,
            converters: parts_to_converters(input_parts.get(idx).unwrap()),
        })
    }

    let lowest_p1 = seeds
        .iter()
        .map(|seed| process_pipeline(*seed, &pipeline))
        .min()
        .unwrap();

    println!("Lowest location = {lowest_p1}");

    println!("=== Part 2 ===");

    let seeds_p2: Vec<_> = seeds
        .chunks(2)
        .map(|chunk| {
            let start = *chunk.first().unwrap();
            let count = *chunk.get(1).unwrap();
            std::ops::Range {
                start,
                end: start + count,
            }
        })
        .collect();

    let lowest_p2 = seeds_p2
        .iter()
        .map(|seed_range| {
            seed_range
                .clone()
                .map(|seed| process_pipeline(seed, &pipeline))
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    println!("Lowest location = {lowest_p2}");
}

fn process_pipeline(seed: i64, pipeline: &Vec<PipeStep>) -> i64 {
    let mut loc: i64 = seed;
    for step in pipeline {
        loc = apply_converts(loc, &step.converters);
    }

    loc
}

fn parts_to_converters(input: &&str) -> Vec<Converter> {
    input
        .lines()
        .skip(1)
        .map(|line| {
            let num: Vec<_> = line
                .split_whitespace()
                .map(|c| c.parse::<i64>().unwrap())
                .collect();

            Converter {
                range_start: num[1],
                range_end: num[1] + num[2] - 1,
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
