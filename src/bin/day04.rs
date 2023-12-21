use aoc::read_lines;

const SEPARATOR: char = '|';
const MAX_CARDS: usize = 223;

#[derive(Debug, PartialEq)]
struct Card {
    id: u32,
    winners: Vec<u32>,
    mine: Vec<u32>,
}

#[derive(Debug, PartialEq)]
struct CardWins {
    id: u32,
    diff: Vec<u32>,
}

fn main() {
    let lines_result = read_lines("input/day04.txt");
    let lines = match lines_result {
        Ok(l) => l,
        Err(error) => panic!("Unable to open the file: {:?}", error),
    };

    let cards: Vec<Card> = lines
        .into_iter()
        .flatten()
        .map(|l| parse_to_card(&l))
        .collect();

    println!("=== PART 1 ===");
    process_part1(&cards);
    println!("=== PART 2 ===");
    process_part2(&cards);
}

fn parse_to_card(input: &str) -> Card {
    let id: u32 = input
        .chars()
        .skip(5)
        .take(3)
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u32>()
        .unwrap();

    let pos_sep = input.chars().position(|c| c == SEPARATOR).unwrap();
    let winners_str: String = input.chars().skip(9).take(pos_sep - 9).collect::<String>();
    let winners: Vec<_> = winners_str
        .split_whitespace()
        .map(|c| c.parse::<u32>().unwrap())
        .collect();

    let mine_str: String = input.chars().skip(pos_sep + 2).collect::<String>();
    let mine: Vec<_> = mine_str
        .split_whitespace()
        .map(|c| c.parse::<u32>().unwrap())
        .collect();

    Card { id, winners, mine }
}

fn process_part1(cards: &Vec<Card>) {
    let mut sum: u32 = 0;
    for card in cards {
        let mine = card.mine.clone();
        let diff: Vec<_> = mine
            .into_iter()
            .filter(|num| card.winners.contains(num))
            .collect();

        if diff.is_empty() {
            continue;
        }

        let exponent: u32 = diff.len() as u32 - 1;
        let score: u32 = u32::pow(2, exponent);

        sum += score;
    }

    println!("Sum of all scores = {sum}");
}

fn process_part2(cards: &Vec<Card>) {
    let mut card_wins: [u32; MAX_CARDS] = [0; MAX_CARDS];
    for card in cards {
        let mine = card.mine.clone();
        let diff: Vec<_> = mine
            .into_iter()
            .filter(|num| card.winners.contains(num))
            .collect();

        let start = usize::try_from(card.id).unwrap() - 1;
        card_wins[start] += 1;

        if diff.is_empty() {
            continue;
        }

        let copies = card_wins[start];
        for i in start + 1..=start + diff.len() {
            card_wins[i] += copies;
        }
    }

    let sum: u32 = card_wins.iter().sum();
    println!("Sum of won cards = {sum}");
}
