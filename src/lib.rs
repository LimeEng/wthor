//! # Wthor
//!
//! Wthor is a rather archaic data-format used extensively by the [Fédération Française d'Othello](https://www.ffothello.org/informatique/la-base-wthor/) to document games of Othello. As a result of their massive collection, Wthor has become a popular data-format in this space. Unfortunately, the only [official documentation](https://www.ffothello.org/wthor/Format_WThor.pdf) is in French. Machine-translating the specification with Google Translate worked reasonably well and the result can be [found here](/spec/wthor_spec.pdf).
//!
//! The translated documentation is however still imprecise and as a result, this library makes very few assumptions about the data. The data extracted is rarely processed in any way but presented as is.
//!
//! Three types of Wthor files can be parsed as of now. They commonly have the extensions `.jou`, `.trn` and `.wtb`.
//!
//! - `.jou` - This type of file contains a list of players.
//! - `.trn` - This type of file contains a list of tournaments.
//! - `.wtb` - These are "game files", essentially a long list of games. Each game contain a header with additional information, such as the tournament it was played at and the name of the players. They are however not directly represented in the structure but instead contains an index which corresponds to an entry in the associated `.jou` and `.trn` files.
//!
//! Solitaire files are currently not supported.
//!
//! ## Example
//!
//! ```ignore
//! // One-shot: parse everything and get enriched records
//! let records = wthor::parse(game_bytes, player_bytes, tournament_bytes)?;
//! for record in &records.games {
//!     println!("{} vs {}", record.black_player, record.white_player);
//! }
//!
//! // Raw path: just moves and scores, ideal for AI training data
//! let games = wthor::Games::try_from(game_bytes)?;
//! println!("Year: {}", games.metadata.year_of_games);
//!
//! // Selective: parse independently and enrich later
//! let players = wthor::Players::try_from(player_bytes)?;
//! let tournaments = wthor::Tournaments::try_from(tournament_bytes)?;
//! let records = games.enrich(&players, &tournaments)?;
//! ```
//!
//! ## Installation
//!
//! ```sh
//! cargo add wthor
//! # If serialization with Serde is desired, activate the serde feature flag.
//! cargo add wthor -F serde
//! ```
//!

mod game;
mod game_archive;
mod game_record;
mod header;
mod players;
mod records;
mod tournaments;
mod wthor;

pub use game::{Game, Position};
pub use game_archive::{BoardSize, Games, Metadata};
pub use game_record::{GameRecord, GameRecords};
pub use header::CreationDate;
pub use players::Players;
pub use tournaments::Tournaments;
pub use wthor::Error;

/// Parse and enrich a complete set of WTHOR files in one step.
///
/// Parses the game archive (`.wtb`), player list (`.jou`), and tournament list
/// (`.trn`) and returns fully enriched [`GameRecords`] with names resolved.
///
/// For the raw path (moves and scores only, no name files), use
/// [`Games::try_from`] directly.
///
/// # Errors
///
/// Returns an error if any of the three byte slices are not valid WTHOR files,
/// or if a game references a player or tournament index that is out of bounds.
///
/// # Example
///
/// ```ignore
/// let records = wthor::parse(game_bytes, player_bytes, tournament_bytes)?;
/// for record in &records.games {
///     println!("{} vs {} — {}", record.black_player, record.white_player, record.tournament);
/// }
/// ```
pub fn parse(
    game_bytes: &[u8],
    player_bytes: &[u8],
    tournament_bytes: &[u8],
) -> Result<GameRecords, Error> {
    let games = Games::try_from(game_bytes)?;
    let players = Players::try_from(player_bytes)?;
    let tournaments = Tournaments::try_from(tournament_bytes)?;
    games.enrich(&players, &tournaments)
}
