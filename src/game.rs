#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{game_archive::BoardSize, wthor::Error};

const VALID_8X8_MOVES: [u8; 64] = [
    11, 12, 13, 14, 15, 16, 17, 18, 21, 22, 23, 24, 25, 26, 27, 28, 31, 32, 33, 34, 35, 36, 37, 38,
    41, 42, 43, 44, 45, 46, 47, 48, 51, 52, 53, 54, 55, 56, 57, 58, 61, 62, 63, 64, 65, 66, 67, 68,
    71, 72, 73, 74, 75, 76, 77, 78, 81, 82, 83, 84, 85, 86, 87, 88,
];
const VALID_10X10_MOVES: [u8; 100] = [
    13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 61, 62, 63, 64, 65, 66, 67, 68,
    69, 70, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 97, 98,
    99, 100, 101, 102, 103, 104, 105, 106, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 121,
    122, 123, 124, 125, 126, 127, 128, 129, 130,
];

pub fn parse(board_size: BoardSize, game: &[u8]) -> Result<Game, Error> {
    if board_size.record_size_in_bytes() != game.len() {
        return Err(Error::InvalidSize);
    }

    let tournament_label_number = u16::from_le_bytes(game[0..2].try_into().unwrap());
    let black_player_number = u16::from_le_bytes(game[2..4].try_into().unwrap());
    let white_player_number = u16::from_le_bytes(game[4..6].try_into().unwrap());
    let real_score = *game.get(6).unwrap();
    let theoretical_score = *game.get(7).unwrap();

    let moves: Result<Vec<Position>, Error> = game[8..]
        .iter()
        .filter_map(|byte| {
            match byte {
                0 => None, // No move was made, skip this entry
                _ => Some(decode_move(board_size, *byte)),
            }
        })
        .collect();

    let moves = moves?;

    Ok(Game {
        tournament_label_number,
        black_player_number,
        white_player_number,
        real_score,
        theoretical_score,
        moves,
    })
}

fn decode_move(board_size: BoardSize, byte: u8) -> Result<Position, Error> {
    match board_size {
        BoardSize::EightSquared => match byte {
            byte if VALID_8X8_MOVES.contains(&byte) => Ok(Position {
                rank: (byte / 10) - 1,
                file: (byte % 10) - 1,
            }),
            _ => Err(Error::InvalidMove),
        },
        BoardSize::TenSquared => match byte {
            byte if VALID_10X10_MOVES.contains(&byte) => Ok(Position {
                rank: (byte / 12) - 1,
                file: (byte % 12) - 1,
            }),
            _ => Err(Error::InvalidMove),
        },
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Position {
    pub rank: u8,
    pub file: u8,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Game {
    pub tournament_label_number: u16,
    pub black_player_number: u16,
    pub white_player_number: u16,
    pub real_score: u8,
    pub theoretical_score: u8,
    pub moves: Vec<Position>,
}
