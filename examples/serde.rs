use wthor::{game_archive::GameArchive, records::Records};

fn main() -> Result<(), Error> {
    let game_archive = include_bytes!("../wthor-database/WTH_1977.wtb");
    let players = include_bytes!("../wthor-database/WTHOR.JOU");
    let tournaments = include_bytes!("../wthor-database/WTHOR.TRN");

    let game_archive = wthor::parse::<GameArchive>(game_archive)?;
    let players = wthor::parse::<Records>(players)?;
    let tournaments = wthor::parse::<Records>(tournaments)?;

    let games_json = serde_json::to_string(&game_archive)?;
    let players_json = serde_json::to_string(&players)?;
    let tournaments_json = serde_json::to_string(&tournaments)?;

    let _deserialized_games: GameArchive = serde_json::from_str(&games_json)?;
    let _deserialized_players: Records = serde_json::from_str(&players_json)?;
    let _deserialized_tournaments: Records = serde_json::from_str(&tournaments_json)?;

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
