use aoc::read_lines;
use regex::Regex;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn main() {
    let lines_result = read_lines("input/day02.txt");
    let lines = match lines_result {
        Ok(l) => l,
        Err(error) => panic!("Unable to open the file: {:?}", error),
    };

    let re_game = Regex::new(r"^Game\s(\d+):").unwrap();
    let re_cubes = Regex::new(r"(\d+) (\w+.?)").unwrap();
    let mut sum_games: i32 = 0;
    for line in lines.into_iter().flatten() {
        let mut reds = 0;
        let mut blues = 0;
        let mut greens = 0;
        let mut is_valid_game: bool = true;

        for (_, [count, color_with_sep]) in re_cubes.captures_iter(&line).map(|c| c.extract()) {
            let col_count: i32 = count.parse().unwrap_or(0);

            let color: &str = color_with_sep.trim_matches(|c| c == ',' || c == ';');
            match color {
                "red" => reds += col_count,
                "blue" => blues += col_count,
                "green" => greens += col_count,
                _ => (),
            }

            let is_valid_draw = reds <= MAX_RED && blues <= MAX_BLUE && greens <= MAX_GREEN;
            if !is_valid_draw {
                is_valid_game = false;
            }

            let is_put_back = color_with_sep.ends_with(';');
            if is_put_back {
                reds = 0;
                blues = 0;
                greens = 0;
            }
        }

        // Extract game nr only if game is valid
        if is_valid_game {
            let game_cap = re_game
                .captures(&line)
                .and_then(|cap| cap.get(1))
                .map(|nr| nr.as_str().parse::<i32>().unwrap());
            match game_cap {
                Some(nr) => {
                    println!("Valid game with nr: {}", nr);
                    sum_games += nr;
                }
                None => println!("Unable to process number"),
            }
        }
    }

    println!("Sum of valid games: {}", sum_games);
}
