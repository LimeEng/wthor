fn main() -> Result<(), Error> {
    let game_archive = include_bytes!("../wthor-database/WTH_1977.wtb");
    let players = include_bytes!("../wthor-database/WTHOR.JOU");
    let tournaments = include_bytes!("../wthor-database/WTHOR.TRN");

    // Example 1: Using the new builder API with names
    let archive_with_names = wthor::parse(game_archive)
        .players(players)
        .tournaments(tournaments)
        .build()?;

    // Example 2: Using the builder API with just indices
    let archive_indexed = wthor::parse(game_archive).build()?;

    // Serialize both versions
    let named_json = serde_json::to_string(&archive_with_names)?;
    let indexed_json = serde_json::to_string(&archive_indexed)?;

    // Deserialize them back
    let _deserialized_named: wthor::Archive<wthor::Named> = serde_json::from_str(&named_json)?;
    let _deserialized_indexed: wthor::Archive<wthor::Indexed> =
        serde_json::from_str(&indexed_json)?;

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    Wthor(wthor::Error),
    Json(serde_json::Error),
}

impl From<wthor::Error> for Error {
    fn from(error: wthor::Error) -> Self {
        Error::Wthor(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}
