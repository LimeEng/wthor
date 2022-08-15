use crate::{
    constants::{VALID_10X10_MOVES, VALID_8X8_MOVES},
    header::{Header, HeaderError},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub(crate) fn parse(header: &Header, bytes: &[u8]) -> Result<Vec<Game>, WtbError> {
    if header.p2 == 1 {
        return Ok(Vec::new());
    }

    parse_games(header, bytes)
}

fn parse_games(header: &Header, games_bytes: &[u8]) -> Result<Vec<Game>, WtbError> {
    let predicated_size =
        u64::from(header.n1) * p1_to_board_size(header.p1).record_size_in_bytes() as u64;
    if predicated_size != games_bytes.len() as u64 {
        return Err(WtbError::Header(HeaderError::InvalidN1Value));
    }

    let mut games = Vec::with_capacity(header.n1 as usize);
    let step = p1_to_board_size(header.p1).record_size_in_bytes();
    for i in 0..header.n1 {
        let start = (i as usize) * step;
        games.push(parse_game(header, &games_bytes[start..start + step])?);
    }
    Ok(games)
}

fn parse_game(header: &Header, game: &[u8]) -> Result<Game, WtbError> {
    if p1_to_board_size(header.p1).record_size_in_bytes() != game.len() {
        return Err(WtbError::Record(RecordError::InvalidSize));
    }

    let tournament_label_number = u16::from_le_bytes(game[0..2].try_into().unwrap());
    let black_player_number = u16::from_le_bytes(game[2..4].try_into().unwrap());
    let white_player_number = u16::from_le_bytes(game[4..6].try_into().unwrap());
    let real_score = *game.get(6).unwrap();
    let theoretical_score = *game.get(7).unwrap();

    let moves: Result<Vec<Position>, WtbError> = game[8..]
        .iter()
        .filter_map(|byte| {
            match byte {
                0 => None, // No move was made, skip this entry
                _ => Some(decode_move(header, *byte)),
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

fn decode_move(header: &Header, byte: u8) -> Result<Position, WtbError> {
    match p1_to_board_size(header.p1) {
        BoardSize::EightSquared => match byte {
            byte if VALID_8X8_MOVES.contains(&byte) => Ok(Position {
                rank: (byte / 10) - 1,
                file: (byte % 10) - 1,
            }),
            _ => Err(WtbError::Record(RecordError::InvalidMove)),
        },
        BoardSize::TenSquared => match byte {
            byte if VALID_10X10_MOVES.contains(&byte) => Ok(Position {
                rank: (byte / 12) - 1,
                file: (byte % 12) - 1,
            }),
            _ => Err(WtbError::Record(RecordError::InvalidMove)),
        },
    }
}

fn p1_to_board_size(p1: u8) -> BoardSize {
    // The header should have been checked to be valid earlier
    match p1 {
        0 | 8 => BoardSize::EightSquared,
        10 => BoardSize::TenSquared,
        _ => panic!("Unsupported board size"),
    }
}

#[derive(Debug)]
pub enum WtbError {
    Header(HeaderError),
    Record(RecordError),
}

#[derive(Debug)]
pub enum RecordError {
    InvalidSize,
    InvalidMove,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Position {
    pub rank: u8,
    pub file: u8,
}

impl From<HeaderError> for WtbError {
    fn from(error: HeaderError) -> Self {
        WtbError::Header(error)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct WtbFile {
    pub header: Header,
    pub games: Option<Vec<Game>>,
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
enum BoardSize {
    EightSquared,
    TenSquared,
}

impl BoardSize {
    pub(crate) fn record_size_in_bytes(&self) -> usize {
        match &self {
            BoardSize::EightSquared => 68,
            BoardSize::TenSquared => 104,
        }
    }
}
