use std::{collections::HashSet, ops::Index};

fn main() -> Result<(), Error> {
    let game_archive = include_bytes!("../wthor-database/WTH_2004.wtb");
    let players = include_bytes!("../wthor-database/WTHOR.JOU");
    let tournaments = include_bytes!("../wthor-database/WTHOR.TRN");

    let game_archive = wthor::parse::<wthor::game_archive::GameArchive>(game_archive)?;
    let players = wthor::parse::<wthor::records::Records>(players)?;
    let tournaments = wthor::parse::<wthor::records::Records>(tournaments)?;

    // Random index
    let player_index = 15;

    let tournaments: HashSet<String> = game_archive
        .games
        .iter()
        .filter(|game| {
            game.black_player_number == player_index || game.white_player_number == player_index
        })
        .map(|game| game.tournament_label_number)
        .map(|tour_index| tournaments.names.index(tour_index as usize))
        .cloned()
        .collect();

    let player = players.names.get(player_index as usize).unwrap().clone();

    println!("{player} participated in the following tournaments in 2004:");
    println!("=====");
    for tournament in tournaments {
        println!("{tournament}");
    }
    Ok(())
}

#[derive(Debug)]
pub enum Error {
    Wthor(wthor::Error),
    Json(serde_json::Error),
    // OptionWasNone,
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

// impl<T> From<Option<T>> for Error {
//     fn from(_: Option<T>) -> Self {
//         Error::OptionWasNone
//     }
// }
