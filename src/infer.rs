use crate::parse::{Game, Position};
use magpie::othello::{OthelloBoard, Stone};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Maximum number of moves a standard game of 8x8 Othello can have
const MAX_MOVES: usize = 60;

pub fn infer_play_order(game: &Game) -> Result<OrderedGame, InferError> {
    if game.moves.len() > MAX_MOVES {
        return Err(InferError::InvalidMoveQuantity);
    }
    let game = infer_order(&game.moves)?;
    Ok(game)
}

fn infer_order(moves: &[Position]) -> Result<OrderedGame, InferError> {
    let mut board = OthelloBoard::standard();
    let mut stone = Stone::Black;

    let mut inferred_moves = Vec::with_capacity(moves.len());

    for pos in moves {
        let bitboard = as_bitboard(pos)?;
        board.place_stone(stone, bitboard)?;
        inferred_moves.push(Move {
            stone,
            bitboard,
            position: pos.clone(),
        });
        if board.moves_for(stone.flip()).count_ones() != 0 {
            stone = stone.flip();
        }
    }

    Ok(OrderedGame {
        moves: inferred_moves,
    })
}

fn as_bitboard(pos: &Position) -> Result<u64, InferError> {
    if pos.rank > 7 || pos.file > 7 {
        Err(InferError::PositionOutOfBounds)
    } else {
        Ok(RANKS[pos.rank as usize] & FILES[pos.file as usize])
    }
}

#[derive(Debug)]
pub enum InferError {
    InvalidMoveQuantity,
    PositionOutOfBounds,
    IllegalMove,
}

impl From<magpie::othello::OthelloError> for InferError {
    fn from(_error: magpie::othello::OthelloError) -> Self {
        InferError::IllegalMove
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct OrderedGame {
    pub moves: Vec<Move>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Move {
    pub stone: Stone,
    pub bitboard: u64,
    pub position: Position,
}

const RANK_1: u64 = 0xff_00_00_00_00_00_00_00;
const RANK_2: u64 = 0x00_ff_00_00_00_00_00_00;
const RANK_3: u64 = 0x00_00_ff_00_00_00_00_00;
const RANK_4: u64 = 0x00_00_00_ff_00_00_00_00;
const RANK_5: u64 = 0x00_00_00_00_ff_00_00_00;
const RANK_6: u64 = 0x00_00_00_00_00_ff_00_00;
const RANK_7: u64 = 0x00_00_00_00_00_00_ff_00;
const RANK_8: u64 = 0x00_00_00_00_00_00_00_ff;
const RANKS: [u64; 8] = [
    RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8,
];

const FILE_A: u64 = 0x80_80_80_80_80_80_80_80;
const FILE_B: u64 = 0x40_40_40_40_40_40_40_40;
const FILE_C: u64 = 0x20_20_20_20_20_20_20_20;
const FILE_D: u64 = 0x10_10_10_10_10_10_10_10;
const FILE_E: u64 = 0x08_08_08_08_08_08_08_08;
const FILE_F: u64 = 0x04_04_04_04_04_04_04_04;
const FILE_G: u64 = 0x02_02_02_02_02_02_02_02;
const FILE_H: u64 = 0x01_01_01_01_01_01_01_01;
const FILES: [u64; 8] = [
    FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H,
];
