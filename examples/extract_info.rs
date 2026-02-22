use std::collections::HashSet;

fn main() -> Result<(), Error> {
    let game_bytes = include_bytes!("../wthor-database/WTH_2004.wtb");
    let player_bytes = include_bytes!("../wthor-database/WTHOR.JOU");
    let tournament_bytes = include_bytes!("../wthor-database/WTHOR.TRN");

    // Parse the game archive
    let archive = wthor::parse(game_bytes)?;

    // Load the database for name lookups
    let db = wthor::Database::new()
        .with_players(player_bytes)?
        .with_tournaments(tournament_bytes)?;

    let player_name = "Nicolet Stephane";

    // Collect all unique player names
    let players: HashSet<_> = archive
        .games
        .iter()
        .flat_map(|game| {
            vec![
                game.black_player(&db).map(String::from),
                game.white_player(&db).map(String::from),
            ]
        })
        .flatten()
        .collect();
    let mut players: Vec<_> = players.into_iter().collect();
    players.sort();
    for player in &players {
        println!("{player}");
    }

    // Find tournaments where a specific player participated
    let player_tournaments: HashSet<String> = archive
        .games
        .iter()
        .filter(|game| {
            let black = game.black_player(&db).unwrap_or_default();
            let white = game.white_player(&db).unwrap_or_default();
            black.contains(player_name) || white.contains(player_name)
        })
        .filter_map(|game| game.tournament(&db).map(String::from))
        .collect();

    println!("{player_name} participated in the following tournaments in 2004:");
    println!("=====");
    for tournament in player_tournaments {
        println!("{tournament}");
    }
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
