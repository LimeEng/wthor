#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    game::Position,
    game_archive::{Games, Metadata},
    players::Players,
    tournaments::Tournaments,
    wthor::Error,
};

/// A game record with player and tournament names resolved.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GameRecord {
    /// The name of the black player.
    pub black_player: String,
    /// The name of the white player.
    pub white_player: String,
    /// The name of the tournament.
    pub tournament: String,
    /// The final score for black (number of discs).
    pub real_score: u8,
    /// The theoretical score if both players played perfectly from some point.
    pub theoretical_score: u8,
    /// The sequence of moves played in the game.
    pub moves: Vec<Position>,
}

/// A collection of enriched game records with player and tournament names resolved.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GameRecords {
    pub metadata: Metadata,
    pub games: Vec<GameRecord>,
}

impl Games {
    /// Enrich raw game data with player and tournament names.
    ///
    /// # Errors
    ///
    /// Returns [`Error::PlayerIndexOutOfBounds`] or [`Error::TournamentIndexOutOfBounds`]
    /// if any game references an index outside the provided lookup tables. This
    /// should not happen with legitimate WTHOR files from a matching year.
    pub fn enrich(
        &self,
        players: &Players,
        tournaments: &Tournaments,
    ) -> Result<GameRecords, Error> {
        let games = self
            .games
            .iter()
            .map(|game| {
                let black_player = players
                    .get(game.black_player_index)
                    .ok_or(Error::PlayerIndexOutOfBounds)?
                    .to_string();
                let white_player = players
                    .get(game.white_player_index)
                    .ok_or(Error::PlayerIndexOutOfBounds)?
                    .to_string();
                let tournament = tournaments
                    .get(game.tournament_index)
                    .ok_or(Error::TournamentIndexOutOfBounds)?
                    .to_string();
                Ok(GameRecord {
                    black_player,
                    white_player,
                    tournament,
                    real_score: game.real_score,
                    theoretical_score: game.theoretical_score,
                    moves: game.moves.clone(),
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(GameRecords {
            metadata: self.metadata.clone(),
            games,
        })
    }
}
