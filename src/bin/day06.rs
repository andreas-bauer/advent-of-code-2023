struct Game {
    time: u64,
    dist: u64,
}

fn main() {
    println!("=== Part 1 ===");
    process_part1();

    println!("=== Part 2 ===");
    process_part2();
}

fn process_part1() {
    let g1 = Game {
        time: 47,
        dist: 207,
    };
    let g2 = Game {
        time: 84,
        dist: 1394,
    };
    let g3 = Game {
        time: 74,
        dist: 1209,
    };
    let g4 = Game {
        time: 67,
        dist: 1014,
    };

    let g1_wins = solutions_for_game(g1);
    let g2_wins = solutions_for_game(g2);
    let g3_wins = solutions_for_game(g3);
    let g4_wins = solutions_for_game(g4);

    println!("G1 wins: {}", g1_wins);
    println!("G2 wins: {}", g2_wins);
    println!("G3 wins: {}", g3_wins);
    println!("G4 wins: {}", g4_wins);

    let sum = g1_wins * g2_wins * g3_wins * g4_wins;
    println!("Sum: {sum}");
}

fn process_part2() {
    let game = Game {
        time: 47847467,
        dist: 207139412091014,
    };
    let sum = solutions_for_game(game);
    println!("Sum: {sum}");
}

fn solutions_for_game(game: Game) -> u64 {
    let mut winnings: u64 = 0;
    let half = game.time / 2;

    for t in (1..=half).rev() {
        let dist = t * (game.time - t);
        if dist <= game.dist {
            break;
        }
        winnings += 1;
    }
    for t in half + 1..game.time {
        let dist = t * (game.time - t);
        if dist <= game.dist {
            break;
        }
        winnings += 1;
    }

    winnings
}
