use crate::{
    enriched::{Archive, Indexed, Named},
    game_archive::GameArchive,
    records::Records,
    wthor::{Error, WthorFile},
};

// State markers
pub struct NoNames;
pub struct WithNames;

#[allow(clippy::struct_field_names)]
pub struct ArchiveBuilder<'a, State> {
    game_bytes: &'a [u8],
    player_bytes: Option<&'a [u8]>,
    tournament_bytes: Option<&'a [u8]>,
    _state: std::marker::PhantomData<State>,
}

#[must_use]
pub fn parse(bytes: &[u8]) -> ArchiveBuilder<'_, NoNames> {
    ArchiveBuilder {
        game_bytes: bytes,
        player_bytes: None,
        tournament_bytes: None,
        _state: std::marker::PhantomData,
    }
}

impl<'a> ArchiveBuilder<'a, NoNames> {
    /// Add both player and tournament data at once
    pub fn with_names(
        self,
        players: &'a [u8],
        tournaments: &'a [u8],
    ) -> ArchiveBuilder<'a, WithNames> {
        ArchiveBuilder {
            game_bytes: self.game_bytes,
            player_bytes: Some(players),
            tournament_bytes: Some(tournaments),
            _state: std::marker::PhantomData,
        }
    }

    /// Add player data
    pub fn players(self, bytes: &'a [u8]) -> ArchiveBuilder<'a, WithNames> {
        ArchiveBuilder {
            game_bytes: self.game_bytes,
            player_bytes: Some(bytes),
            tournament_bytes: self.tournament_bytes,
            _state: std::marker::PhantomData,
        }
    }

    /// Build the archive with indexed games only (no name data)
    pub fn build(self) -> Result<Archive<Indexed>, Error> {
        let game_archive = GameArchive::parse(self.game_bytes)?;
        Ok(Archive::indexed(game_archive))
    }
}

impl<'a> ArchiveBuilder<'a, WithNames> {
    /// Add tournament data (after players were added)
    pub fn tournaments(mut self, bytes: &'a [u8]) -> Self {
        self.tournament_bytes = Some(bytes);
        self
    }

    /// Build the archive with named games (requires both player and tournament data)
    pub fn build(self) -> Result<Archive<Named>, Error> {
        match (self.player_bytes, self.tournament_bytes) {
            (Some(p), Some(t)) => {
                let game_archive = GameArchive::parse(self.game_bytes)?;
                let players = Records::parse(p)?;
                let tournaments = Records::parse(t)?;
                Archive::with_names(game_archive, &players, &tournaments)
            }
            _ => Err(Error::IncompleteNameData),
        }
    }
}
