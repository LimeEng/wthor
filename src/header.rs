#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const HEADER_LENGTH: usize = 16;

pub fn parse_and_split(bytes: &[u8]) -> Option<(Header, &[u8])> {
    if bytes.len() < HEADER_LENGTH {
        return None;
    }
    let header = &bytes[..HEADER_LENGTH];
    let bytes = &bytes[HEADER_LENGTH..];
    let file_creation_date = CreationDate {
        century: header[0],
        year: header[1],
        month: header[2],
        day: header[3],
    };

    let n1 = u32::from_le_bytes(header[4..8].try_into().unwrap());
    let n2 = u16::from_le_bytes(header[8..10].try_into().unwrap());

    let year_of_games = u16::from_le_bytes(header[10..12].try_into().unwrap());

    let p1 = header[12];
    let p2 = header[13];
    let p3 = header[14];
    let reserved = header[15];

    Some((
        Header {
            file_creation_date,
            n1,
            n2,
            year_of_games,
            p1,
            p2,
            p3,
            reserved,
        },
        bytes,
    ))
}

#[derive(Clone, Debug)]
pub struct Header {
    pub file_creation_date: CreationDate,
    pub n1: u32,
    pub n2: u16,
    pub year_of_games: u16,
    pub p1: u8,
    pub p2: u8,
    pub p3: u8,
    pub reserved: u8,
}

impl Header {
    pub fn most_likely_game(&self) -> bool {
        self.n2 == 0
    }

    pub fn most_likely_records(&self) -> bool {
        !(self.most_likely_game() || self.is_solitaire())
    }

    pub fn is_solitaire(&self) -> bool {
        self.p2 == 1
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct CreationDate {
    pub century: u8,
    pub year: u8,
    pub month: u8,
    pub day: u8,
}
