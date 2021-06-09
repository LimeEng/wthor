use crate::parse::header;
use crate::parse::header::BoardSize;
use crate::parse::header::Header;
use crate::parse::header::HeaderError;
use std::convert::TryInto;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const HEADER_LENGTH: usize = 16;

pub fn parse(bytes: &[u8]) -> Result<WthorFile, WthorError> {
    let header_bytes = &bytes[..HEADER_LENGTH];
    let games_bytes = &bytes[HEADER_LENGTH..];

    let header = header::parse_header(header_bytes)?;
    if header.is_solitaire {
        return Err(WthorError::SolitaireNotSupported);
    }

    let games = parse_games(&header, games_bytes)?;

    Ok(WthorFile { header, games })
}

fn parse_games(header: &Header, games_bytes: &[u8]) -> Result<Vec<Game>, WthorError> {
    let predicated_size = header.n1 * header.board_size.record_size_in_bytes() as u32;
    if predicated_size != games_bytes.len() as u32 {
        return Err(WthorError::Header(HeaderError::InvalidN1Record));
    }

    let mut games = Vec::with_capacity(header.n1 as usize);
    let step = header.board_size.record_size_in_bytes();
    for i in 0..(header.n1 - 1) {
        let start = (i as usize) * step;
        games.push(parse_game(&header, &games_bytes[start..start + step])?);
    }
    Ok(games)
}

fn parse_game(header: &Header, game: &[u8]) -> Result<Game, WthorError> {
    if header.board_size.record_size_in_bytes() != game.len() {
        return Err(WthorError::Record(RecordError::InvalidSize));
    }

    let tournament_label_number = u16::from_le_bytes(game[0..2].try_into().unwrap());
    let black_player_number = u16::from_le_bytes(game[2..4].try_into().unwrap());
    let white_player_number = u16::from_le_bytes(game[4..6].try_into().unwrap());
    let real_score = *game.get(6).unwrap();
    let theoretical_score = *game.get(7).unwrap();

    let moves: Result<Vec<Position>, WthorError> = game[8..]
        .iter()
        .filter_map(|byte| {
            match byte {
                0 => None, // No move was made, skip this entry
                _ => Some(decode_move(&header, *byte)),
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

fn decode_move(header: &Header, byte: u8) -> Result<Position, WthorError> {
    match header.board_size {
        BoardSize::EightSquared => match byte {
            11..=88 => Ok(Position {
                rank: (byte / 10) - 1,
                file: (byte % 10) - 1,
            }),
            _ => Err(WthorError::Record(RecordError::InvalidMove)),
        },
        BoardSize::TenSquared => match byte {
            13..=130 => Ok(Position {
                rank: (byte / 12) - 1,
                file: (byte % 12) - 1,
            }),
            _ => Err(WthorError::Record(RecordError::InvalidMove)),
        },
    }
}

#[derive(Debug)]
pub enum WthorError {
    IoError(std::io::Error),
    Header(HeaderError),
    Record(RecordError),
    SolitaireNotSupported,
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

impl From<std::io::Error> for WthorError {
    fn from(error: std::io::Error) -> Self {
        WthorError::IoError(error)
    }
}

impl From<HeaderError> for WthorError {
    fn from(error: HeaderError) -> Self {
        WthorError::Header(error)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct WthorFile {
    pub header: Header,
    pub games: Vec<Game>,
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
