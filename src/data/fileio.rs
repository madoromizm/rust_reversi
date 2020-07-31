use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use regex::Regex;

use crate::domain_model::r#move::Move;
use crate::domain_model::cell_pos::CellPos;
use crate::domain_model::turn::Turn;
use crate::domain_model::reversi_state::ReversiState;
use crate::error::ReversiError;

static GAMESTATE_FILENAME: &'static str = "othello_gamestate.txt";

pub fn write_file(state: &ReversiState) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(GAMESTATE_FILENAME)?;
    writeln!(file, "{}", state.undo_buffer.len())?;
    for undo_move in &state.undo_buffer {
        writeln!(file, "{} {} {}", 
            turn_to_string( &undo_move.turn ), 
            undo_move.put_pos.row, undo_move.put_pos.col)?;
    }
    for redo_move in &state.redo_buffer {
        writeln!(file, "{} {} {}", 
            turn_to_string( &redo_move.turn ),
            redo_move.put_pos.row, redo_move.put_pos.col)?;
    }
    
    Ok(())
}

fn turn_to_string(turn: &Turn) -> &str {
    match turn {
        Turn::Black => "BLACK",
        Turn::White => "WHITE",
    }
}

// ファイル読み込みを行い、undo buffer, redo bufferを作成する
pub fn read_file() -> Result<(Vec<Move>, Vec<Move>), Box<dyn std::error::Error>> {
    let mut undo_list: Vec<Move> = Vec::new();
    let mut redo_list: Vec<Move> = Vec::new();

    let mut reader = BufReader::new(File::open(GAMESTATE_FILENAME)?);
    let mut buf = String::new();

    // undoの個数を読み取り
    reader.read_line(&mut buf)?;
    let undo_count: usize = buf.trim().parse()?;

    // 残りを読み取る。
    let r = Regex::new(r"^(BLACK|WHITE) ([1-8]) ([1-8])$").unwrap();
    let mut moves: Vec<Move> = Vec::new();
    let mut line_count = 2;
    for line in reader.lines() {
        let content = line?;
        let cap = r.captures(content.as_str())
            .ok_or(ReversiError::new(format!("読み取りに失敗しました 行番号={}", line_count)))?;
        let the_move = Move {
            turn: if cap[1].trim() == "BLACK" { Turn::Black } else { Turn::White },
            put_pos: CellPos {
                row: cap[2].parse()?,
                col: cap[3].parse()?,
            },
            flipped_pos_list: Vec::new(),
        };
        moves.push(the_move);
        line_count += 1;
    }

    undo_list.extend_from_slice(&moves[0..undo_count]);
    redo_list.extend_from_slice(&moves[undo_count..]);

    Ok((undo_list, redo_list))
}
