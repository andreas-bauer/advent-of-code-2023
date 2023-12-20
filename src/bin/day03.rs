use aoc::read_lines;
use std::cmp;

// const WIDTH: usize=10;
const HEIGHT: usize= 10;
const SPACE: char = '.';

#[derive(PartialEq)]
struct FoundNum {
    pos_x_start: usize,
    pos_x_end: usize,
    number: i32,
} 

fn main() {
    let lines_result = read_lines("input/day03.txt");
    let lines = match lines_result {
        Ok(l) => l,
        Err(error) => panic!("Unable to open the file: {:?}", error),
    };
   
    let mut vec: Vec<Vec<char>> = Vec::with_capacity(HEIGHT);
    for line in lines.into_iter().flatten() {
        vec.push(line.chars().collect());
    }

    process_part1(&vec);
}



fn process_part1(vec: &Vec<Vec<char>>) {
    let mut findings: Vec<FoundNum> = Vec::new();
    
    for y in 0..vec.len() {
        for x in 0..vec[y].len() {
            if !vec[y][x].is_ascii_digit() {
                continue;
            }
            
            let is_valid = is_valid_num(vec, y, x);
            if is_valid {
                let found = extract_number(vec, y, x);
                findings.push(found);
            }
        }
    }

    findings.dedup();

    let sum: i32 = findings.iter()
        .map(|f| &f.number)
        .sum();
  
    println!("Sum of all found number: {sum}");
}

fn is_symbol(c: char) -> bool {
    if SPACE == c || c.is_ascii_digit() {
        return false;
    }
    return true;
}

fn is_valid_num(vec: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    let y_min: usize = y.checked_sub(1).unwrap_or(y);
    let y_max: usize = cmp::min(y + 1, vec.len() -1);
    let x_min: usize = x.checked_sub(1).unwrap_or(x);
    let x_max: usize = cmp::min(x + 1, vec[y_min].len() -1);

    for iy in y_min..=y_max {
        for ix in x_min..=x_max {
            if is_symbol(vec[iy][ix]) {
                return true;
            }
        }
    }    

    return false;    
}

fn extract_number(vec: &Vec<Vec<char>>, y: usize, x: usize) -> FoundNum {
    let mut pos_start: usize = 0;
    for pos in (0..=x).rev() {
        if !vec[y][pos].is_ascii_digit() {
            break;
        }
        pos_start = pos;
    }
    
    let mut pos_end: usize = 0;
    for pos in x..vec[y].len() {
        if !vec[y][pos].is_ascii_digit() {
            break;
        }
        pos_end = pos;
    }

    let num_as_str: String = vec[y][pos_start..=pos_end].iter().collect();
    let num = num_as_str.parse::<i32>().unwrap();

    return FoundNum {
        pos_x_start: pos_start,
        pos_x_end: pos_end,
        number: num
    }
}