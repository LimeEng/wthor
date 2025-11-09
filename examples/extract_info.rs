use std::collections::HashSet;

fn main() -> Result<(), Error> {
    let game_archive = include_bytes!("../wthor-database/WTH_2004.wtb");
    let players = include_bytes!("../wthor-database/WTHOR.JOU");
    let tournaments = include_bytes!("../wthor-database/WTHOR.TRN");

    // Use the new builder API to parse with names resolved
    let archive = wthor::parse(game_archive)
        .players(players)
        .tournaments(tournaments)
        .build()?;

    let player = "Nicolet Stephane";

    let players: HashSet<_> = archive
        .games
        .iter()
        .flat_map(|t| vec![t.meta.black_player.clone(), t.meta.white_player.clone()])
        .collect();
    let mut players: Vec<_> = players.iter().cloned().collect();
    players.sort();
    for player in players {
        println!("{player}");
    }

    let player_tournaments: HashSet<String> = archive
        .games
        .iter()
        .filter(|game| {
            game.meta.black_player.contains(player) || game.meta.white_player.contains(player)
        })
        .map(|game| game.meta.tournament.clone())
        .collect();

    println!("{player} participated in the following tournaments in 2004:");
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
