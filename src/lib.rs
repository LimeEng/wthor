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
//! // Parse games (always available, indices only)
//! let archive = wthor::parse(game_bytes)?;
//! println!("Year: {}", archive.metadata.year_of_games);
//! println!("First game score: {}", archive.games[0].real_score);
//!
//! // Optionally load a database for name lookups
//! let db = wthor::Database::new()
//!     .with_players(player_bytes)?
//!     .with_tournaments(tournament_bytes)?;
//!
//! for game in &archive.games {
//!     let black = game.black_player(&db).unwrap_or("Unknown");
//!     let white = game.white_player(&db).unwrap_or("Unknown");
//!     println!("{black} vs {white}");
//! }
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

mod database;
mod game;
mod game_archive;
mod header;
mod records;
mod wthor;

use game_archive::GameArchive;
use wthor::WthorFile;

pub use database::Database;
pub use game::{Game, Position};
pub use game_archive::{BoardSize, Metadata};
pub use header::CreationDate;
pub use wthor::Error;

/// A parsed archive of Othello games.
pub type Archive = GameArchive;

/// Parse a WTHOR game archive (`.wtb` file).
///
/// Returns an archive containing game metadata and a list of games.
/// Games contain indices into player/tournament records. Use [`Database`]
/// to look up names by index.
///
/// # Errors
///
/// Returns an error if the bytes are not a valid WTHOR game archive.
///
/// # Example
///
/// ```ignore
/// let archive = wthor::parse(bytes)?;
/// for game in &archive.games {
///     println!("Score: {}", game.real_score);
/// }
/// ```
pub fn parse(bytes: &[u8]) -> Result<Archive, Error> {
    GameArchive::parse(bytes)
}
