//! Database for player and tournament name lookups.
//!
//! The WTHOR game files contain only indices into separate player and tournament
//! files. This module provides a `Database` struct that loads these files and
//! allows efficient name lookups.

use crate::{
    records::Records,
    wthor::{Error, WthorFile},
};

/// A database of player and tournament names for resolving game indices.
///
/// # Example
///
/// ```ignore
/// let db = Database::new()
///     .with_players(player_bytes)?
///     .with_tournaments(tournament_bytes)?;
///
/// let archive = wthor::parse(game_bytes)?;
/// for game in &archive.games {
///     if let Some(name) = game.black_player(&db) {
///         println!("Black: {name}");
///     }
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct Database {
    players: Option<Vec<String>>,
    tournaments: Option<Vec<String>>,
}

impl Database {
    /// Create an empty database.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Load player names from a `.jou` file.
    ///
    /// # Errors
    ///
    /// Returns an error if the bytes cannot be parsed as a valid WTHOR records file.
    pub fn with_players(mut self, bytes: &[u8]) -> Result<Self, Error> {
        let records = Records::parse(bytes)?;
        self.players = Some(records.names);
        Ok(self)
    }

    /// Load tournament names from a `.trn` file.
    ///
    /// # Errors
    ///
    /// Returns an error if the bytes cannot be parsed as a valid WTHOR records file.
    pub fn with_tournaments(mut self, bytes: &[u8]) -> Result<Self, Error> {
        let records = Records::parse(bytes)?;
        self.tournaments = Some(records.names);
        Ok(self)
    }

    /// Look up a player name by 1-based index.
    ///
    /// Returns `None` if:
    /// - No player data has been loaded
    /// - The index is 0 (invalid in WTHOR format)
    /// - The index is out of bounds
    #[must_use]
    pub fn player(&self, index: u16) -> Option<&str> {
        let players = self.players.as_ref()?;
        let idx = index.checked_sub(1)? as usize;
        players.get(idx).map(String::as_str)
    }

    /// Look up a tournament name by 1-based index.
    ///
    /// Returns `None` if:
    /// - No tournament data has been loaded
    /// - The index is 0 (invalid in WTHOR format)
    /// - The index is out of bounds
    #[must_use]
    pub fn tournament(&self, index: u16) -> Option<&str> {
        let tournaments = self.tournaments.as_ref()?;
        let idx = index.checked_sub(1)? as usize;
        tournaments.get(idx).map(String::as_str)
    }

    /// Returns `true` if player data has been loaded.
    #[must_use]
    pub fn has_players(&self) -> bool {
        self.players.is_some()
    }

    /// Returns `true` if tournament data has been loaded.
    #[must_use]
    pub fn has_tournaments(&self) -> bool {
        self.tournaments.is_some()
    }
}
