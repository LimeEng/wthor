use std::convert::TryInto;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const HEADER_LENGTH: usize = 16;

pub(crate) fn parse_header(header: &[u8]) -> Result<Header, HeaderError> {
    if header.len() != HEADER_LENGTH {
        return Err(HeaderError::InvalidHeader);
    }
    let century = *header.get(0).unwrap();
    let year = *header.get(1).unwrap();
    let month = *header.get(2).unwrap();
    let day = *header.get(3).unwrap();
    let file_creation_date = FileCreationDate {
        century,
        year,
        month,
        day,
    };

    let n1 = u32::from_le_bytes(header[4..8].try_into().unwrap());
    let n2 = u16::from_le_bytes(header[8..10].try_into().unwrap());

    let year_of_games = u16::from_le_bytes(header[10..12].try_into().unwrap());

    let board_size = header.get(12).unwrap().to_le();
    let board_size = match board_size {
        0 | 8 => BoardSize::EightSquared,
        10 => BoardSize::TenSquared,
        _ => return Err(HeaderError::UnsupportedBoardSize),
    };

    let is_solitaire = *header.get(13).unwrap();
    let is_solitaire = match is_solitaire {
        0 => false,
        1 => true,
        _ => return Err(HeaderError::InvalidP2Record),
    };
    let p3 = *header.get(14).unwrap();

    Ok(Header {
        file_creation_date,
        n1,
        n2,
        year_of_games,
        board_size,
        is_solitaire,
        p3,
    })
}

#[derive(Debug)]
pub enum HeaderError {
    InvalidHeader,
    UnsupportedBoardSize,
    InvalidN1Record,
    InvalidP2Record,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Header {
    pub file_creation_date: FileCreationDate,
    pub n1: u32,
    pub n2: u16,
    pub year_of_games: u16,
    pub board_size: BoardSize,
    pub is_solitaire: bool,
    pub p3: u8,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct FileCreationDate {
    pub century: u8,
    pub year: u8,
    pub month: u8,
    pub day: u8,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum BoardSize {
    EightSquared,
    TenSquared,
}

impl BoardSize {
    pub(crate) fn record_size_in_bytes(&self) -> usize {
        use BoardSize::*;
        match &self {
            EightSquared => 68,
            TenSquared => 104,
        }
    }
}
