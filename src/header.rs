use crate::constants::HEADER_LENGTH;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub(crate) fn parse(header: &[u8]) -> Result<Header, HeaderError> {
    if header.len() != HEADER_LENGTH {
        return Err(HeaderError::InvalidHeaderSize);
    }
    let file_creation_date = FileCreationDate {
        century: header[0],
        year: header[1],
        month: header[2],
        day: header[3],
    };

    let n1 = u32::from_le_bytes(header[4..8].try_into().unwrap());
    let n2 = u16::from_le_bytes(header[8..10].try_into().unwrap());

    let year_of_parties = u16::from_le_bytes(header[10..12].try_into().unwrap());

    let p1 = header[12];
    let p1 = match p1 {
        0 | 8 | 10 => p1,
        _ => return Err(HeaderError::InvalidP1Value),
    };

    let p2 = header[13];
    let p2 = match p2 {
        0 | 1 => p2,
        _ => return Err(HeaderError::InvalidP2Value),
    };

    let p3 = header[14];
    let reserved = header[15];

    Ok(Header {
        file_creation_date,
        n1,
        n2,
        year_of_parties,
        p1,
        p2,
        p3,
        reserved,
    })
}

#[derive(Debug)]
pub enum HeaderError {
    InvalidHeaderSize,
    InvalidN1Value,
    InvalidP1Value,
    InvalidP2Value,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Header {
    pub file_creation_date: FileCreationDate,
    pub n1: u32,
    pub n2: u16,
    pub year_of_parties: u16,
    pub p1: u8,
    pub p2: u8,
    pub p3: u8,
    pub reserved: u8,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct FileCreationDate {
    pub century: u8,
    pub year: u8,
    pub month: u8,
    pub day: u8,
}
