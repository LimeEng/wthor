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
//! ## Installation
//!
//! ```sh
//! cargo add wthor
//! # If serialization with Serde is desired, activate the serde feature flag.
//! cargo add wthor -F serde
//! ```
//!

pub mod game_archive;
mod header;
pub mod records;
mod wthor;

pub use self::wthor::{Error, parse};
pub use header::CreationDate;
