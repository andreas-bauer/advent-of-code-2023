struct Game {
    time: u32,
    dist: u32,
}

fn main() {
    let g1 = Game { time: 47, dist: 207 };
    let g2 = Game { time: 84, dist: 1394 };
    let g3 = Game { time: 74, dist: 1209 };
    let g4: Game = Game { time: 67, dist: 1014 };

    let g1_win = solutions_for_game(g1);
    let g2_win = solutions_for_game(g2);
    let g3_win = solutions_for_game(g3);
    let g4_win = solutions_for_game(g4);

    println!("G1 wins: {}", g1_win.len());
    println!("G2 wins: {}", g2_win.len());
    println!("G3 wins: {}", g3_win.len());
    println!("G4 wins: {}", g4_win.len());

    let sum = g1_win.len() * g2_win.len() * g3_win.len() * g4_win.len();
    println!("Sum: {sum}");
}

fn solutions_for_game(game: Game) -> Vec<u32> {
    let mut winnings: Vec<u32> = vec![];
    let half = game.time / 2;

    for t in (1..=half).rev() {
        let dist = t * (game.time - t);
        if dist <= game.dist {
            // println!("T:{} wit dist:{} => out",t, dist);
            break;
        }
        // println!("T:{} wit dist:{} => in",t, dist);
        winnings.push(dist);
    }
    for t in half+1..game.time {
        let dist = t * (game.time - t);
        // println!("T:{} wit dist:{} => out",t, dist);
        if dist <= game.dist {
            // println!("T:{} wit dist:{} => in",t, dist);
            break;
        }
        winnings.push(dist);
    }
    winnings
}
