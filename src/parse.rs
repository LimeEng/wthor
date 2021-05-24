use serde::{Deserialize, Serialize};

const HEADER_LENGTH: usize = 16;

pub fn parse(bytes: &[u8]) -> Result<WthorFile, WthorError> {
    let header_bytes = &bytes[..HEADER_LENGTH];
    let games_bytes = &bytes[HEADER_LENGTH..];

    let header = parse_header(header_bytes)?;
    let predicated_size = header.num_games * header.board_size.record_size_in_bytes() as u32;
    if predicated_size != games_bytes.len() as u32 {
        return Err(WthorError::Header(HeaderError::InvalidN1Record));
    }

    let mut games = Vec::with_capacity(header.num_games as usize);
    let step = header.board_size.record_size_in_bytes();
    for i in 0..(header.num_games - 1) {
        let start = (i as usize) * step;
        games.push(parse_game(&header, &games_bytes[start..start + step])?);
    }

    Ok(WthorFile { header, games })
}

fn parse_header(header: &[u8]) -> Result<Header, WthorError> {
    if header.len() != HEADER_LENGTH {
        return Err(WthorError::Header(HeaderError::InvalidHeader));
    }

    let mut bytes_u32: [u8; 4] = Default::default();
    bytes_u32.copy_from_slice(&header[4..8]);
    let num_games = as_u32_le(&bytes_u32);

    let board_size = header.get(12).unwrap().to_le();
    let board_size = match board_size {
        0 | 8 => BoardSize::EightSquared,
        10 => BoardSize::TenSquared,
        _ => return Err(WthorError::Header(HeaderError::UnsupportedBoardSize)),
    };
    Ok(Header {
        board_size,
        num_games,
    })
}

fn parse_game(header: &Header, game: &[u8]) -> Result<Game, WthorError> {
    if header.board_size.record_size_in_bytes() != game.len() {
        return Err(WthorError::Record(RecordError::InvalidSize));
    }
    let moves: Result<Vec<Position>, WthorError> = game[8..]
        .iter()
        .filter_map(|byte| {
            match byte {
                0 => None, // No move was made, skip this entry
                _ => Some(header.board_size.decode_move(*byte)),
            }
        })
        .collect();

    let moves = moves?;

    Ok(Game { moves })
}

// https://commandcenter.blogspot.com/2012/04/byte-order-fallacy.html
fn as_u32_le(bytes: &[u8; 4]) -> u32 {
    (bytes[0] as u32)
        + ((bytes[1] as u32) << 8)
        + ((bytes[2] as u32) << 16)
        + ((bytes[3] as u32) << 24)
}

#[derive(Debug)]
pub enum WthorError {
    IoError(std::io::Error),
    Header(HeaderError),
    Record(RecordError),
}

#[derive(Debug)]
pub enum RecordError {
    InvalidSize,
    InvalidMove,
}

#[derive(Debug)]
pub enum HeaderError {
    InvalidHeader,
    UnsupportedBoardSize,
    InvalidN1Record,
}

impl From<std::io::Error> for WthorError {
    fn from(error: std::io::Error) -> Self {
        WthorError::IoError(error)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Header {
    pub board_size: BoardSize,
    num_games: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BoardSize {
    EightSquared,
    TenSquared,
}

impl BoardSize {
    fn record_size_in_bytes(&self) -> usize {
        use BoardSize::*;
        match &self {
            EightSquared => 68,
            TenSquared => 104,
        }
    }
    fn decode_move(&self, byte: u8) -> Result<Position, WthorError> {
        use BoardSize::*;
        match &self {
            EightSquared => match byte {
                11..=88 => Ok(Position {
                    rank: (byte / 10) - 1,
                    file: (byte % 10) - 1,
                }),
                _ => Err(WthorError::Record(RecordError::InvalidMove)),
            },
            TenSquared => match byte {
                13..=130 => Ok(Position {
                    rank: (byte / 12) - 1,
                    file: (byte % 12) - 1,
                }),
                _ => Err(WthorError::Record(RecordError::InvalidMove)),
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub rank: u8,
    pub file: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WthorFile {
    pub header: Header,
    pub games: Vec<Game>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub moves: Vec<Position>,
}
