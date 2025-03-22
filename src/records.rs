use crate::{
    CreationDate,
    header::{self, Header},
    wthor::{Error, WthorFile},
};
use encoding::{DecoderTrap, Encoding, all::ISO_8859_1};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const PLAYER_RECORD_SIZE: usize = 20;
const TOURNAMENT_RECORD_SIZE: usize = 26;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Metadata {
    pub file_creation_date: CreationDate,
}

impl From<&Header> for Metadata {
    fn from(header: &Header) -> Self {
        Self {
            file_creation_date: header.file_creation_date,
        }
    }
}

fn validate_header(header: &Header, bytes: &[u8]) -> Result<RecordType, Error> {
    if header.n1 != 0 {
        Err(Error::InvalidN1Value)
    } else if header.n2 == 0 {
        Err(Error::InvalidN2Value)
    } else if header.p1 != 0 {
        Err(Error::InvalidP1Value)
    } else if header.p2 != 0 {
        Err(Error::InvalidP2Value)
    } else {
        let record_count = header.n2;
        RecordType::guess_type(record_count, bytes.len()).ok_or(Error::SizeMismatch)
    }
}

fn validate_null_termination(record: &[u8]) -> Result<&[u8], Error> {
    record
        .last()
        .is_some_and(|last| *last == 0)
        .then_some(record)
        .ok_or(Error::RecordNotNullTerminated)
}

fn decode_record(record: &[u8]) -> Result<String, Error> {
    ISO_8859_1
        .decode(record, DecoderTrap::Strict)
        .map_err(|_| Error::EncodingNotIso8859_1)
        .map(|s| s.trim_matches(char::from(0)).to_string())
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum RecordType {
    Players,
    Tournaments,
}

impl RecordType {
    fn guess_type(n2: u16, bytes_len: usize) -> Option<Self> {
        let n2 = n2 as usize;
        if n2 * Self::Players.record_size() == bytes_len {
            Some(Self::Players)
        } else if n2 * Self::Tournaments.record_size() == bytes_len {
            Some(Self::Tournaments)
        } else {
            None
        }
    }

    fn record_size(&self) -> usize {
        match self {
            RecordType::Players => PLAYER_RECORD_SIZE,
            RecordType::Tournaments => TOURNAMENT_RECORD_SIZE,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Records {
    pub metadata: Metadata,
    pub names: Vec<String>,
    pub record_type: RecordType,
}

impl WthorFile for Records {
    type Output = Self;

    fn parse(bytes: &[u8]) -> Result<Self::Output, Error> {
        let (header, bytes) = header::parse_and_split(bytes).ok_or(Error::InvalidHeader)?;
        let record_type = validate_header(&header, bytes)?;

        let names: Result<Vec<_>, _> = bytes
            .chunks_exact(record_type.record_size())
            .map(|record| validate_null_termination(record).and_then(decode_record))
            .collect();
        Ok(Records {
            metadata: Metadata::from(&header),
            names: names?,
            record_type,
        })
    }
}
