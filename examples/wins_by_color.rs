use magpie::othello::{OthelloBoard, Stone};
use std::cmp::Ordering;
use wthor::{Position, WthorError};

#[derive(Debug, Default)]
struct GameResults {
    black_wins: u64,
    white_wins: u64,
    draws: u64,
}

fn main() -> Result<(), Error> {
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

fn as_bitboard(pos: &Position) -> u64 {
    RANKS[pos.rank as usize] & FILES[pos.file as usize]
}

fn calculate_winner(game: &wthor::Game) -> Option<Stone> {
    let mut board = OthelloBoard::standard();
    let mut stone = Stone::Black;

    for pos in &game.moves {
        board.place_stone(stone, as_bitboard(pos)).unwrap();
        if board.moves_for(stone.flip()).count_ones() != 0 {
            stone = stone.flip();
        }
    }

    let black_stones = board.bits_for(Stone::Black).count_ones();
    let white_stones = board.bits_for(Stone::White).count_ones();

    match black_stones.cmp(&white_stones) {
        Ordering::Greater => Some(Stone::Black),
        Ordering::Less => Some(Stone::White),
        Ordering::Equal => None,
    }
}

#[derive(Debug)]
pub enum Error {
    Wthor(WthorError),
    Json(serde_json::Error),
}

impl From<WthorError> for Error {
    fn from(error: WthorError) -> Self {
        Error::Wthor(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}

const FILE_A: u64 = 0x80_80_80_80_80_80_80_80;
const FILE_B: u64 = 0x40_40_40_40_40_40_40_40;
const FILE_C: u64 = 0x20_20_20_20_20_20_20_20;
const FILE_D: u64 = 0x10_10_10_10_10_10_10_10;
const FILE_E: u64 = 0x08_08_08_08_08_08_08_08;
const FILE_F: u64 = 0x04_04_04_04_04_04_04_04;
const FILE_G: u64 = 0x02_02_02_02_02_02_02_02;
const FILE_H: u64 = 0x01_01_01_01_01_01_01_01;
const FILES: [u64; 8] = [
    FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H,
];

const RANK_1: u64 = 0xff_00_00_00_00_00_00_00;
const RANK_2: u64 = 0x00_ff_00_00_00_00_00_00;
const RANK_3: u64 = 0x00_00_ff_00_00_00_00_00;
const RANK_4: u64 = 0x00_00_00_ff_00_00_00_00;
const RANK_5: u64 = 0x00_00_00_00_ff_00_00_00;
const RANK_6: u64 = 0x00_00_00_00_00_ff_00_00;
const RANK_7: u64 = 0x00_00_00_00_00_00_ff_00;
const RANK_8: u64 = 0x00_00_00_00_00_00_00_ff;
const RANKS: [u64; 8] = [
    RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8,
];

#[allow(unused)]
fn debug_board(title: &str, board: u64) {
    let char_at = |rank: usize, file: usize| {
        let nth_bit = (rank * 8) + file;
        let result = (board >> (63 - nth_bit)) & 1;
        if result == 1 {
            "#"
        } else {
            "."
        }
    };

    println!("{}", title);
    println!("   ABCDEFGH");
    println!("  +--------+");
    for rank in 0..8 {
        print!("{} |", rank + 1);
        for file in 0..8 {
            print!("{}", char_at(rank, file));
        }
        println!("|");
    }
    println!("  +--------+");
}
