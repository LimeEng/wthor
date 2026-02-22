fn main() -> Result<(), Error> {
    let game_bytes = include_bytes!("../wthor-database/WTH_1977.wtb");

    // Parse the game archive
    let archive = wthor::parse(game_bytes)?;

    // Serialize to JSON
    let json = serde_json::to_string(&archive)?;

    // Deserialize back
    let _deserialized: wthor::Archive = serde_json::from_str(&json)?;

    println!("Successfully serialized and deserialized {} games", archive.games.len());
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
