#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{database::Database, game_archive::BoardSize, wthor::Error};

pub fn parse(board_size: BoardSize, game: &[u8]) -> Result<Game, Error> {
    if board_size.record_size_in_bytes() != game.len() {
        return Err(Error::InvalidSize);
    }

    let [t0, t1, b0, b1, w0, w1, real_score, theoretical_score]: [u8; 8] =
        game[0..8].try_into().unwrap();

    let tournament_index = u16::from_le_bytes([t0, t1]);
    let black_player_index = u16::from_le_bytes([b0, b1]);
    let white_player_index = u16::from_le_bytes([w0, w1]);

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
        tournament_index,
        black_player_index,
        white_player_index,
        real_score,
        theoretical_score,
        moves,
    })
}

fn decode_move(board_size: BoardSize, byte: u8) -> Result<Position, Error> {
    let (rows, cols, base): (u8, u8, u8) = match board_size {
        BoardSize::EightSquared => (8, 8, 10),
        BoardSize::TenSquared => (10, 10, 12),
    };

    let rank = (byte / base)
        .checked_sub(1)
        .filter(|r| *r < rows)
        .ok_or(Error::InvalidMove)?;

    let file = (byte % base)
        .checked_sub(1)
        .filter(|f| *f < cols)
        .ok_or(Error::InvalidMove)?;

    Ok(Position { rank, file })
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
    /// 1-based index into tournament records. Use `tournament()` to look up the name.
    pub tournament_index: u16,
    /// 1-based index into player records. Use `black_player()` to look up the name.
    pub black_player_index: u16,
    /// 1-based index into player records. Use `white_player()` to look up the name.
    pub white_player_index: u16,
    /// The final score for black (number of discs).
    pub real_score: u8,
    /// The theoretical score if both players played perfectly from some point.
    pub theoretical_score: u8,
    /// The sequence of moves played in the game.
    pub moves: Vec<Position>,
}

impl Game {
    /// Look up the black player's name from a database.
    ///
    /// Returns `None` if the database has no player data or the index is invalid.
    #[must_use]
    pub fn black_player<'a>(&self, db: &'a Database) -> Option<&'a str> {
        db.player(self.black_player_index)
    }

    /// Look up the white player's name from a database.
    ///
    /// Returns `None` if the database has no player data or the index is invalid.
    #[must_use]
    pub fn white_player<'a>(&self, db: &'a Database) -> Option<&'a str> {
        db.player(self.white_player_index)
    }

    /// Look up the tournament name from a database.
    ///
    /// Returns `None` if the database has no tournament data or the index is invalid.
    #[must_use]
    pub fn tournament<'a>(&self, db: &'a Database) -> Option<&'a str> {
        db.tournament(self.tournament_index)
    }
}
