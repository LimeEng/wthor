use magpie::othello::{Board, Position, Stone};
use wthor::game_archive::{GameArchive, Game};
use std::{cmp::Ordering, fs};

// macro_rules! move_order_test {
//     ($($year:literal)*) => {
//         $(
//             move_order_test!(#[test] $year);
//         )*
//     };
//     ($(#[$m:meta])* $year:literal) => {
//         paste::item! {
//             $(#[$m])*
//             fn [< test_move_order_ $year >] () {
//                 let t = $year;
//                 println!("{t}");
//                 let bytes = include_bytes!($year);
//                 // test_move_order_tmp(bytes);
//                 // let file_name = std::concat!("../wthor-database/WTH_", [<$year>], ".wtb");
//                 // println!("{file_name}");
//             }
//         }
//     };
// }

#[test]
fn test_move_order() {
    let years = 1977..=2021;

    for year in years {
        let file_name = format!("wthor-database/WTH_{year}.wtb");
        println!("{file_name}");
        let bytes = fs::read(file_name).unwrap();
        test_move_order_inner(&bytes);
    }
}

fn test_move_order_inner(bytes: &[u8]) {
    let file = wthor::parse::<GameArchive>(bytes).unwrap();
    for game in file.games {
        let score = calculate_score(&game);
        // if score != game.real_score as u32 {
        //     println!("Actual: {}", score);
        //     println!("Expected: {}", game.real_score);
        //     // assert!(false);
        // }
        assert_eq!(score, u32::from(game.real_score));
    }
}

fn calculate_score(game: &Game) -> u32 {
    let mut board = Board::standard();
    let mut stone = Stone::Black;

    let positions: Vec<Position> = game
        .moves
        .iter()
        .filter_map(|pos| Position::try_from((pos.rank, pos.file)).ok())
        .collect();

    for pos in positions {
        board.play(stone, pos);
        if !board.moves_for(stone.flip()).is_empty() {
            stone = stone.flip();
        }
    }

    let black = board.bits_for(Stone::Black).raw();
    let white = board.bits_for(Stone::White).raw();

    // debug_board("Black", black);
    // debug_board("White", white);
    // debug_board("Total", black | white);

    let black = black.count_ones();
    let white = white.count_ones();

    // The score should always add up to 64
    // with the winner taking the empty squares
    match black.cmp(&white) {
        Ordering::Greater => 64 - white,
        Ordering::Less => 64 - (64 - black),
        Ordering::Equal => 32,
    }
}

fn debug_board(title: &str, board: u64) {
    let char_at = |rank: usize, file: usize| {
        let nth_bit = (rank * 8) + file;
        let result = (board >> (63 - nth_bit)) & 1;
        if result == 1 {
            "#"
        } else {
            "."
        }
    };

    println!("{title}");
    println!("   ABCDEFGH");
    println!("  +--------+");
    for rank in 0..8 {
        print!("{} |", rank + 1);
        for file in 0..8 {
            print!("{}", char_at(rank, file));
        }
        println!("|");
    }
    println!("  +--------+");
}
