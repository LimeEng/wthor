fn main() -> Result<(), Error> {
    let game_bytes = include_bytes!("../wthor-database/WTH_1977.wtb");

    // Parse the raw game archive
    let games = wthor::Games::try_from(game_bytes.as_ref())?;

    // Serialize to JSON
    let json = serde_json::to_string(&games)?;

    // Deserialize back
    let _deserialized: wthor::Games = serde_json::from_str(&json)?;

    println!(
        "Successfully serialized and deserialized {} games",
        games.games.len()
    );
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
