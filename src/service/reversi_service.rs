use std::fs::File;
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use regex::Regex;

use crate::error::ReversiError;
use crate::domain_model::reversi_state::ReversiState;
use crate::domain_model::turn::Turn;
use crate::domain_model::cell::CellState;
use crate::domain_model::cell_pos::CellPos;
use crate::domain_model::board::Board;
use crate::domain_model::r#move::Move;
use crate::data::fileio;

pub fn put_stone(state: &mut ReversiState, row: usize, col: usize) -> Result<(), ReversiError> {
    let stone = match state.turn {
        Turn::Black => CellState::BlackStone,
        Turn::White => CellState::WhiteStone,
    };

    // 盤面の範囲内か確認
    if !state.board.is_in_range(row, col) {
        return Err(ReversiError::new("盤面の範囲外です"));
    }

    // 既に置いてあるか確認
    match state.board.get_cell(row, col) {
        Some(cell) => match cell {
            CellState::BlankCell => {},
            _ => { return Err(ReversiError::new("既に石が置かれています")) },
        },
        None => return Err(ReversiError::new(format!("石の位置が範囲外です: row={}, col={}", row, col))),
    }

    // 反転できるか探索する
    let flip_cells = state.board.find_flippable_cells(row, col, stone)?;

    // 反転する石があるか？
    if flip_cells.is_empty() { 
        return Err(ReversiError::new("反転できる石がありません"))
    }

    // 石を置いて
    state.board.set_cell(row, col, stone);

    // ひっくり返す
    state.board.flip(&flip_cells);


    // 手を記録する
    state.register_move(row, col, flip_cells);

    // redoバッファはクリア
    state.redo_buffer.clear();

    // ゲーム終了か調べる
    if state.board.has_blank_cell() {
        // BlankCellがあればゲーム継続
        // ターンを切り替える
        switch_turn(state);
    } else {
        // BlankCellがなければゲーム終了
        state.gameover = true;
    }

    Ok(())
}

pub fn get_result_string(state: &ReversiState) -> String {
    format!("{}の勝ち", if state.board.count_black_stones() > 32 {"黒"} else {"白"})
}

pub fn switch_turn(state: &mut ReversiState) {
    state.turn = if state.turn == Turn::Black { Turn::White } else { Turn::Black }
}

pub fn pass(state: &mut ReversiState) {
    switch_turn(state);
}

pub fn get_history_size(state: &ReversiState) -> usize {
    state.undo_buffer.len()
}

// init_state() は不要な気がする

pub fn undo(state: &mut ReversiState) -> Result<(), ReversiError> {
    state.undo()
}

pub fn redo(state: &mut ReversiState) -> Result<(), ReversiError> {
    state.redo()
}


pub fn save(state: &ReversiState) -> Result<(), ReversiError> {
    match fileio::write_file(&state) {
        Ok(_) => Ok(()),
        Err(e) => Err(ReversiError::new(format!("保存に失敗しました: {:?}", e))),
    }
}


pub fn load() -> Result<ReversiState, ReversiError> {
	// ファイル読み込みに成功していればゲーム状態を復元する。
    fn restore_state(undo_list: Vec<Move>, redo_list: Vec<Move>) -> Result<ReversiState, Box<dyn std::error::Error>> {
        fn do_one_move(state: &mut ReversiState, mv: &Move) -> Result<(), ReversiError> {
            // 順番を決めて
            state.turn = mv.turn;

            // 次に置く石を決めて
            let stone = if mv.turn == Turn::Black { CellState::BlackStone } else { CellState::WhiteStone };

            // 石を置いて
            state.board.set_cell(
                mv.put_pos.row as usize, mv.put_pos.col as usize,
                stone)?;

            // ひっくり返す
            let flip_cells = state.board.find_flippable_cells(
                mv.put_pos.row as usize, mv.put_pos.col as usize, stone)?;
            state.board.flip(&flip_cells);

            // 手を記録する
            state.register_move(
                mv.put_pos.row as usize, mv.put_pos.col as usize, flip_cells);

            switch_turn(state);

            Ok(())
        }

        let mut state = ReversiState::new();

        for mv in &undo_list {
            do_one_move(&mut state, &mv)?;
        }
        for mv in &redo_list {
            do_one_move(&mut state, &mv)?;
        }
        for _ in 0..redo_list.len() {
            state.undo()?;
        }

        Ok(state)
    }

    fileio::read_file()
        .and_then(|(undo_list, redo_list)| restore_state(undo_list, redo_list) )
        .or_else(|e| Err( ReversiError::new(format!("{}", e) )))

}
