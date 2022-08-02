use crate::{
    constants::{HEADER_LENGTH, PLAYER_RECORD_SIZE, TOURNAMENT_RECORD_SIZE},
    header, name_file, wtb_file,
    wtb_file::Game,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub fn parse(bytes: &[u8]) -> Result<WthorFile, WthorError> {
    if bytes.len() < HEADER_LENGTH {
        // Will return an error
        header::parse(bytes)?;
    }
    let header_bytes = &bytes[..HEADER_LENGTH];
    let header = header::parse(header_bytes)?;

    let bytes = &bytes[HEADER_LENGTH..];

    // P2 == 1 means solitaire file
    if header.p2 == 1 {
        return Ok(WthorFile {
            header,
            games: None,
            tournaments: None,
            players: None,
        });
    }

    // N2 == 0 means game file
    if header.n2 == 0 {
        let games = wtb_file::parse(&header, bytes)?;
        return Ok(WthorFile {
            header,
            games: Some(games),
            tournaments: None,
            players: None,
        });
    }

    // In all other cases, it should be a file with the names of either players
    // or tournaments.
    let tournaments = name_file::parse(TOURNAMENT_RECORD_SIZE, &header, bytes);
    if let Ok(tournaments) = tournaments {
        return Ok(WthorFile {
            header,
            games: None,
            tournaments: Some(tournaments),
            players: None,
        });
    }

    let players = name_file::parse(PLAYER_RECORD_SIZE, &header, bytes);
    if let Ok(players) = players {
        return Ok(WthorFile {
            header,
            games: None,
            tournaments: None,
            players: Some(players),
        });
    }

    Err(WthorError::InvalidFormat)
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct WthorFile {
    pub header: header::Header,
    pub games: Option<Vec<Game>>,
    pub tournaments: Option<Vec<String>>,
    pub players: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum WthorError {
    Header(header::HeaderError),
    WtbFile(wtb_file::WtbError),
    NameFile(name_file::NameFileError),
    InvalidFormat,
}

impl From<header::HeaderError> for WthorError {
    fn from(error: header::HeaderError) -> Self {
        WthorError::Header(error)
    }
}

impl From<wtb_file::WtbError> for WthorError {
    fn from(error: wtb_file::WtbError) -> Self {
        WthorError::WtbFile(error)
    }
}

impl From<name_file::NameFileError> for WthorError {
    fn from(error: name_file::NameFileError) -> Self {
        WthorError::NameFile(error)
    }
}
