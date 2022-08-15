use wthor::WthorError;

fn main() -> Result<(), Error> {
    let games = include_bytes!("../wthor-database/WTH_1977.wtb");
    let players = include_bytes!("../wthor-database/WTHOR.JOU");
    let tournaments = include_bytes!("../wthor-database/WTHOR.TRN");

    let games = wthor::parse(games)?;
    let players = wthor::parse(players)?;
    let tournaments = wthor::parse(tournaments)?;

    let games_json = serde_json::to_string(&games)?;
    let players_json = serde_json::to_string(&players)?;
    let tournaments_json = serde_json::to_string(&tournaments)?;

    let _deserialized_games: wthor::WthorFile = serde_json::from_str(&games_json)?;
    let _deserialized_players: wthor::WthorFile = serde_json::from_str(&players_json)?;
    let _deserialized_tournaments: wthor::WthorFile = serde_json::from_str(&tournaments_json)?;

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    Wthor(WthorError),
    Json(serde_json::Error),
}

impl From<WthorError> for Error {
    fn from(error: WthorError) -> Self {
        Error::Wthor(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}
