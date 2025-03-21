use magpie::othello::{Board, Position, Stone};
use std::{cmp::Ordering, convert::TryFrom};
use wthor::WthorError;

#[derive(Debug, Default)]
struct GameResults {
    black_wins: u64,
    white_wins: u64,
    draws: u64,
}

fn main() -> Result<(), WthorError> {
    let games = include_bytes!("../wthor-database/WTH_2004.wtb");
    let games = wthor::parse(games)?
        .games
        .expect("Unexpected parsing error");

    let results =
        games
            .iter()
            .map(calculate_winner)
            .fold(GameResults::default(), |mut results, winner| {
                match winner {
                    Some(winner) => match winner {
                        Stone::Black => results.black_wins += 1,
                        Stone::White => results.white_wins += 1,
                    },
                    None => results.draws += 1,
                };
                results
            });

    println!("Wins by color among all games in 2004");
    println!("Black: {}", results.black_wins);
    println!("White: {}", results.white_wins);
    println!("Draws: {}", results.draws);
    Ok(())
}

fn calculate_winner(game: &wthor::Game) -> Option<Stone> {
    let mut board = Board::standard();
    let mut stone = Stone::Black;

    let positions: Vec<Position> = game
        .moves
        .iter()
        .filter_map(|pos| Position::try_from((pos.rank, pos.file)).ok())
        .collect();

    for pos in positions {
        board.play(stone, pos);
        if !board.moves_for(stone.flip()).is_empty() {
            stone = stone.flip();
        }
    }

    let black_stones = board.bits_for(Stone::Black).count_set();
    let white_stones = board.bits_for(Stone::White).count_set();

    match black_stones.cmp(&white_stones) {
        Ordering::Greater => Some(Stone::Black),
        Ordering::Less => Some(Stone::White),
        Ordering::Equal => None,
    }
}
