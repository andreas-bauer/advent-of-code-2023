use aoc::read_lines;
use itertools::Itertools;

#[derive(Debug)]
struct Play {
    bid: u32,
    score: u32,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
#[repr(u8)]
enum HandType {
    HighCard = 1,
    OnePair,
    TwoPairs,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[repr(u8)]
enum Card {
    Two = 1,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

const MAX_PLAYS: usize = 1000;
const HAND_CARDS: usize = 5;

fn main() {
    let lines_result = read_lines("input/day07.txt");
    let lines = match lines_result {
        Ok(l) => l,
        Err(error) => panic!("Unable to open the file: {:?}", error),
    };

    let mut plays: Vec<Play> = Vec::with_capacity(MAX_PLAYS);
    for line in lines.flatten() {
        let cards: Vec<Card> = line.chars().take(HAND_CARDS).map(parse_card).collect();

        let bid: u32 = line
            .chars()
            .skip(HAND_CARDS + 1)
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        let hand = calc_hand_type(&cards);
        let score: u32 = calc_hand_score(&cards, hand);

        plays.push(Play { bid, score });
    }

    plays.sort_by_key(|p| p.score);

    let sum: u32 = plays
        .iter()
        .enumerate()
        .map(|(idx, play)| (idx as u32 + 1, play))
        .map(|(rank, play)| rank * play.bid)
        .sum();

    println!("Sum of total winnings: {sum}");
}

fn parse_card(c: char) -> Card {
    match c {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'J' => Card::Jack,
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => panic!("Unable to continue. Parsing of '{}' failed. ", c),
    }
}

fn calc_hand_type(cards: &[Card]) -> HandType {
    let counts_of_kind: Vec<_> = cards.iter().sorted().dedup_with_count().collect();
    let max_of_kind = counts_of_kind
        .iter()
        .map(|(count, _)| *count)
        .max()
        .unwrap();

    match max_of_kind {
        5 => return HandType::FiveOfKind,
        4 => return HandType::FourOfKind,
        _ => (),
    }

    let pairs_count: usize = counts_of_kind
        .iter()
        .filter(|(count, _)| *count == 2usize)
        .count();

    if max_of_kind == 3 && pairs_count == 1 {
        return HandType::FullHouse;
    }

    if max_of_kind == 3 {
        return HandType::ThreeOfKind;
    }

    match pairs_count {
        2 => return HandType::TwoPairs,
        1 => return HandType::OnePair,
        _ => (),
    }

    HandType::HighCard
}

fn calc_hand_score(cards: &[Card], hand: HandType) -> u32 {
    let cards_score: u32 = cards
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, &card)| (idx as u32, card as u32))
        .map(|(pos, card_value)| card_value * u32::pow(13, pos))
        .sum();

    let offset = hand as u32 * 1000000;

    cards_score + offset
}

#[test]
fn test_calc_hand_type() {
    assert_eq!(
        calc_hand_type(&[Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
        HandType::FiveOfKind
    );

    assert_eq!(
        calc_hand_type(&[Card::Six, Card::Six, Card::Six, Card::Six, Card::Ace]),
        HandType::FourOfKind
    );

    assert_eq!(
        calc_hand_type(&[Card::Two, Card::Two, Card::Three, Card::Three, Card::Three]),
        HandType::FullHouse
    );

    assert_eq!(
        calc_hand_type(&[Card::Ten, Card::Ten, Card::Ten, Card::Nine, Card::Eight]),
        HandType::ThreeOfKind
    );

    assert_eq!(
        calc_hand_type(&[Card::Two, Card::Three, Card::Two, Card::Three, Card::Four]),
        HandType::TwoPairs
    );

    assert_eq!(
        calc_hand_type(&[Card::Two, Card::Three, Card::Four, Card::Ace, Card::Ace]),
        HandType::OnePair
    );

    assert_eq!(
        calc_hand_type(&[Card::Two, Card::Three, Card::Four, Card::Five, Card::Six]),
        HandType::HighCard
    );
}

#[test]
fn test_calc_hand_score() {
    let s1 = calc_hand_score(
        &[Card::Three, Card::King, Card::King, Card::King, Card::Three],
        HandType::ThreeOfKind,
    );

    let s2 = calc_hand_score(
        &[Card::Four, Card::Four, Card::Four, Card::Three, Card::Three],
        HandType::ThreeOfKind,
    );

    assert_ne!(s1, s2);
}
