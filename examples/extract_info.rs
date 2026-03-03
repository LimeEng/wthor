use std::collections::HashSet;

fn main() -> Result<(), Error> {
    let game_bytes = include_bytes!("../wthor-database/WTH_2004.wtb");
    let player_bytes = include_bytes!("../wthor-database/WTHOR.JOU");
    let tournament_bytes = include_bytes!("../wthor-database/WTHOR.TRN");

    // Parse and enrich in one step
    let records = wthor::parse(game_bytes, player_bytes, tournament_bytes)?;

    let player_name = "Nicolet Stephane";

    // Collect all unique player names
    let all_players: HashSet<_> = records
        .games
        .iter()
        .flat_map(|r| [r.black_player.as_str(), r.white_player.as_str()])
        .collect();
    let mut all_players: Vec<_> = all_players.into_iter().collect();
    all_players.sort_unstable();
    for player in &all_players {
        println!("{player}");
    }

    // Find tournaments where a specific player participated
    let player_tournaments: HashSet<&str> = records
        .games
        .iter()
        .filter(|r| r.black_player.contains(player_name) || r.white_player.contains(player_name))
        .map(|r| r.tournament.as_str())
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
