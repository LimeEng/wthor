use std::{collections::HashSet, ops::Index};
use wthor::WthorError;

fn main() -> Result<(), Error> {
    let games = include_bytes!("../wthor-database/WTH_2004.wtb");
    let players = include_bytes!("../wthor-database/WTHOR.JOU");
    let tournaments = include_bytes!("../wthor-database/WTHOR.TRN");

    let games = wthor::parse(games)?
        .games
        .expect("Unexpected parsing error");
    let players = wthor::parse(players)?
        .players
        .expect("Unexpected parsing error");
    let tournaments = wthor::parse(tournaments)?
        .tournaments
        .expect("Unexpected parsing error");

    // Random index
    let player_index = 15;

    let tournaments: HashSet<String> = games
        .iter()
        .filter(|game| {
            game.black_player_number == player_index || game.white_player_number == player_index
        })
        .map(|game| game.tournament_label_number)
        .map(|tour_index| tournaments.index(tour_index as usize))
        .cloned()
        .collect();

    let player = players[player_index as usize].clone();

    println!(
        "{} participated in the following tournaments in 2004:",
        player
    );
    println!("=====");
    for tournament in tournaments {
        println!("{}", tournament);
    }
    Ok(())
}

#[derive(Debug)]
pub enum Error {
    Wthor(WthorError),
    Json(serde_json::Error),
    OptionWasNone,
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

impl<T> From<Option<T>> for Error {
    fn from(_: Option<T>) -> Self {
        Error::OptionWasNone
    }
}
