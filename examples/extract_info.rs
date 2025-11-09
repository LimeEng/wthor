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

    let player_tournaments: HashSet<String> = archive
        .games
        .iter()
        .filter(|game| {
            // Access indices from the Named state (though we're looking at the resolved names)
            // In this case we'd need to track the original indices differently
            // For now, let's just show all tournaments this player participated in
            game.state.black_player.contains("Ben Seeley")
                || game.state.white_player.contains("Ben Seeley")
        })
        .map(|game| game.state.tournament.clone())
        .collect();

    // The player at index 15 is Ben Seeley
    let player = "Ben Seeley";

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
