#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    CreationDate,
    game::{self, Game},
    header::{self, Header},
    wthor::{Error, WthorFile},
};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Metadata {
    pub file_creation_date: CreationDate,
    pub year_of_games: u16,
    pub board_size: BoardSize,
}

impl TryFrom<&Header> for Metadata {
    type Error = Error;

    fn try_from(header: &Header) -> Result<Self, Self::Error> {
        let board_size = match header.p1 {
            0 | 8 => BoardSize::EightSquared,
            10 => BoardSize::TenSquared,
            _ => return Err(Error::InvalidP1Value),
        };
        Ok(Self {
            file_creation_date: header.file_creation_date,
            year_of_games: header.year_of_games,
            board_size,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardSize {
    EightSquared,
    TenSquared,
}

impl BoardSize {
    pub(crate) fn record_size_in_bytes(self) -> usize {
        match &self {
            BoardSize::EightSquared => 68,
            BoardSize::TenSquared => 104,
        }
    }
}

fn extract_metadata(header: &Header, bytes: &[u8]) -> Result<Metadata, Error> {
    if header.n2 != 0 {
        Err(Error::InvalidN1Value)
    } else if header.p2 != 0 {
        Err(Error::InvalidP2Value)
    } else {
        let metadata = Metadata::try_from(header)?;

        let predicated_size =
            u64::from(header.n1) * metadata.board_size.record_size_in_bytes() as u64;
        if predicated_size != bytes.len() as u64 {
            return Err(Error::SizeMismatch);
        }

        Ok(metadata)
    }
}

impl WthorFile for GameArchive {
    type Output = GameArchive;

    fn parse(bytes: &[u8]) -> Result<Self::Output, Error> {
        let (header, bytes) = header::parse_and_split(bytes).ok_or(Error::InvalidHeader)?;
        let metadata = extract_metadata(&header, bytes)?;

        let mut games = Vec::with_capacity(header.n1 as usize);
        let step = metadata.board_size.record_size_in_bytes();
        for i in 0..header.n1 {
            let start = (i as usize) * step;
            games.push(game::parse(
                metadata.board_size,
                &bytes[start..start + step],
            )?);
        }
        Ok(GameArchive { metadata, games })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GameArchive {
    pub metadata: Metadata,
    pub games: Vec<Game>,
}
