use std::fs;

pub fn main() {
    // Example 1: Parse games with player and tournament names
    example_with_names();

    // Example 2: Parse games with indices only
    example_indexed_only();
}

fn example_with_names() {
    println!("=== Example: Games with Names ===\n");

    // Read the game file
    let game_bytes = fs::read("wthor-database/WTH_2021.wtb").expect("Failed to read game file");

    // Read player and tournament files
    let player_bytes = fs::read("wthor-database/WTHOR.JOU").expect("Failed to read player file");
    let tournament_bytes =
        fs::read("wthor-database/WTHOR.TRN").expect("Failed to read tournament file");

    // Parse with the fluent API - returns Archive<Named>
    let archive = wthor::parse(&game_bytes)
        .players(&player_bytes)
        .tournaments(&tournament_bytes)
        .build()
        .expect("Failed to parse archive");

    println!("Archive metadata:");
    println!("  Year: {}", archive.metadata.year_of_games);
    println!("  Board size: {:?}", archive.metadata.board_size);

    // Access games directly - no Option wrapping needed!
    println!("\nFirst 3 games:");
    for (i, game) in archive.games.iter().take(3).enumerate() {
        println!("\nGame {}:", i + 1);
        println!("  Tournament: {}", game.state.tournament);
        println!("  Black: {}", game.state.black_player);
        println!("  White: {}", game.state.white_player);
        println!("  Score: {}", game.real_score);
        println!("  Moves: {}", game.moves.len());
    }
}

fn example_indexed_only() {
    println!("\n\n=== Example: Games with Indices Only ===\n");

    let game_bytes = fs::read("wthor-database/WTH_2021.wtb").expect("Failed to read game file");

    // Parse without name resolution - returns Archive<Indexed>
    let archive = wthor::parse(&game_bytes)
        .build()
        .expect("Failed to parse archive");

    // Access games directly
    if let Some(game) = archive.games.first() {
        println!("First game:");
        println!("  Tournament index: {}", game.state.tournament_index);
        println!("  Black player index: {}", game.state.black_player_index);
        println!("  White player index: {}", game.state.white_player_index);
        println!("  Score: {}", game.real_score);
    }
}
