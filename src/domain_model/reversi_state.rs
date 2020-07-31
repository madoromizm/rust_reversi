use crate::error::ReversiError;
use super::cell_pos::CellPos;
use crate::domain_model::cell::CellState;
use super::board::Board;
use super::r#move::Move;
use super::turn::Turn;

pub struct ReversiState {
    pub board: Board,
    pub turn: Turn,
    pub gameover: bool,
    pub undo_buffer: Vec<Move>,
    pub redo_buffer: Vec<Move>
}

impl ReversiState {
    pub fn new() -> ReversiState {
        ReversiState {
            board: Board::new(),
            turn: Turn::Black,
            gameover: false,
            undo_buffer: Vec::new(),
            redo_buffer: Vec::new(),
        }
    }

    pub fn register_move(&mut self, row: usize, col: usize, flipped_cells: Vec<CellPos>) {
        let r#move = Move { 
            turn: self.turn,
            put_pos: CellPos { row: row as i8, col: col as i8},
            flipped_pos_list: flipped_cells,
        };

        self.undo_buffer.push(r#move);
    }

    // pub fn initialize(&mut self) {
    //     for i in 1..=8 {
    //         for j in 1..=8 {
    //             self.board.cells[i][j] = 1;
    //         }
    //     }

    //     self.board.cells[4][4] = 2;
    //     self.board.cells[4][5] = 3;
    //     self.board.cells[5][4] = 3;
    //     self.board.cells[5][5] = 2;

    //     self.turn = Turn::Black;
    //     self.gameover = false;

    //     self.undo_buffer.clear();
    //     self.redo_buffer.clear();
    // }

    pub fn undo(&mut self) -> Result<(), ReversiError> {
        let last_move = match self.undo_buffer.pop() {
            Some(mv) => mv,
            None => return Err(ReversiError::new("履歴がありません"))
        };
 
        self.board.flip(&last_move.flipped_pos_list);
        self.board.set_cell(
            last_move.put_pos.row as usize, last_move.put_pos.col as usize, 
            CellState::BlankCell)?;
        self.turn = last_move.turn;

        self.redo_buffer.push(last_move);

        Ok(())
    }

    pub fn redo(&mut self) -> Result<(), ReversiError> {
        let redo_move = match self.redo_buffer.pop() {
            Some(mv) => mv,
            None => return Err(ReversiError::new("redoバッファが空です"))
        };

        self.board.flip(&redo_move.flipped_pos_list);
        self.board.set_cell(
            redo_move.put_pos.row as usize,
            redo_move.put_pos.col as usize,
            if redo_move.turn == Turn::Black { CellState::BlackStone } else { CellState::WhiteStone })?;
        self.turn = if redo_move.turn == Turn::Black { Turn::White } else { Turn::Black };

        self.undo_buffer.push(redo_move);

        Ok(())
    }
}