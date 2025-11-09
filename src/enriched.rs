#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    game::Position,
    game_archive::{GameArchive, Metadata},
    records::Records,
    wthor::Error,
};

/// An archive with type-level state indicating whether it has indexed or named games
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Archive<State> {
    pub metadata: Metadata,
    pub games: Vec<Game<State>>,
}

impl Archive<Indexed> {
    /// Create an archive with only indices (no name resolution)
    pub(crate) fn indexed(game_archive: GameArchive) -> Self {
        let games = game_archive.games.into_iter().map(Game::from_raw).collect();

        Archive {
            metadata: game_archive.metadata,
            games,
        }
    }
}

impl Archive<Named> {
    /// Create an archive with resolved names
    pub(crate) fn with_names(
        game_archive: GameArchive,
        players: &Records,
        tournaments: &Records,
    ) -> Result<Self, Error> {
        let games = game_archive
            .games
            .into_iter()
            .map(|g| Game::resolve_names(g, players, tournaments))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Archive {
            metadata: game_archive.metadata,
            games,
        })
    }
}

/// A game with state that can be either Indexed or Named
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Game<Meta> {
    pub meta: Meta,
    pub real_score: u8,
    pub theoretical_score: u8,
    pub moves: Vec<Position>,
}

/// State for games with only indices
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Indexed {
    pub tournament_index: u16,
    pub black_player_index: u16,
    pub white_player_index: u16,
}

/// State for games with resolved names
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Named {
    pub tournament: String,
    pub black_player: String,
    pub white_player: String,
}

impl Game<Indexed> {
    pub(crate) fn from_raw(game: crate::game::Game) -> Self {
        Game {
            meta: Indexed {
                tournament_index: game.tournament_label_number,
                black_player_index: game.black_player_number,
                white_player_index: game.white_player_number,
            },
            real_score: game.real_score,
            theoretical_score: game.theoretical_score,
            moves: game.moves,
        }
    }
}

impl Game<Named> {
    pub(crate) fn resolve_names(
        game: crate::game::Game,
        players: &Records,
        tournaments: &Records,
    ) -> Result<Self, Error> {
        let tournament = tournaments
            .names
            .get(game.tournament_label_number.saturating_sub(1) as usize)
            .ok_or(Error::InvalidTournamentIndex)?
            .clone();

        let black_player = players
            .names
            .get(game.black_player_number.saturating_sub(1) as usize)
            .ok_or(Error::InvalidPlayerIndex)?
            .clone();

        let white_player = players
            .names
            .get(game.white_player_number.saturating_sub(1) as usize)
            .ok_or(Error::InvalidPlayerIndex)?
            .clone();

        Ok(Game {
            meta: Named {
                tournament,
                black_player,
                white_player,
            },
            real_score: game.real_score,
            theoretical_score: game.theoretical_score,
            moves: game.moves,
        })
    }
}
