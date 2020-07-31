use std::io;
use regex::Regex;
use std::io::Write;

use crate::domain_model::reversi_state::ReversiState;
use crate::view::view_util;
use crate::service::reversi_service;
use crate::error::ReversiError;
use crate::domain_model::turn::Turn;

pub fn show(state: &mut ReversiState) { //} -> io::Result<usize> {
    // state.initialize();
    // show_state(state);

    // ゲームループ
    // let mut in_game = true;
    'game_loop: loop {
        show_state(state);

        // メニュー表示。ゲームを終了する？
        view_util::show_header2(format!("{}の番", if state.turn == Turn::Black {"黒"} else {"白"}).as_str());
        println!("例)43[Enter] (4段目の3列目に石を置く)");
        println!("または0でゲーム終了");

        // 入力ループ
        let mut row = 0usize;
        let mut col = 0usize;
        'input_loop: loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();

            // "0"(ゲーム終了要求)の処理
            if line.trim() == "0" {
                println!("ゲームを終了します");
                println!("ゲーム内容を保存しますか？[(y)es/(n)o/(c)ancel]");
                let mut y_n_c = String::new();
                loop {
                    io::stdin().read_line(&mut y_n_c).unwrap();
                    let regex_ync = Regex::new(r"[ync]").unwrap();
                    if regex_ync.is_match(y_n_c.as_str().trim()) {
                        break;
                    } else {
                        println!("y n c のいずれかを入力してください");
                        y_n_c.clear();
                    }
                }
                match y_n_c.as_str().trim() {
                    "y" => {
                        match reversi_service::save(state) {
                            Ok(_) => {
                                println!("ゲーム内容を保存しました");
                            },
                            Err(e) => {
                                println!("ゲームの保存に失敗しました: {}", e.message);
                            }
                        }
                        break 'game_loop;
                    },
                    "n" => {
                        println!("ゲームを終了します");
                        break 'game_loop;
                    },
                    "c" => {
                        println!("ゲームを継続します");
                        continue 'game_loop;
                    },
                    _ => {}
                }
            }

            // オセロのセル指定
            let r = Regex::new(r"^([1-8])([1-8])$").unwrap();
            if !r.is_match(line.trim()) {
                println!("入力が不正です。");
                println!("例)43[Enter]");
                continue 'input_loop;
            }
            let c = r.captures(line.trim()).unwrap();
            // [デバッグ] println!("c:{:#?}", c);
            row = c[1].trim().parse().unwrap();
            col = c[2].trim().parse().unwrap();
            // [デバッグ] println!("row={} col={}", row, col);
            break;
        } // 入力ループ

        // 現在のターンのプレーヤーが石を置く
        match reversi_service::put_stone(state, row, col) {
            Ok(_) => {},
            Err(e) => {
                println!("{}", e.message);
                continue;
            }
        }

        // 勝敗判定。勝ち負け、置くところがなくなった。
        if state.gameover {
            println!("{}", reversi_service::get_result_string(state));
            // in_game = false;
            break 'game_loop;
        }
        
    }

    // Ok(0)
}

pub fn show_state(state: &ReversiState) {
    show_board(state);   
}

pub fn show_board(state: &ReversiState) {
    println!("   1  2  3  4  5  6  7  8(列)");
    println!("  +-----------------------+");
    for row in 1..=8 {
        print!("{} |", row);
        for col in 1..=8 {
            print!("{}", state.board.cells[row][col]);
            io::stdout().flush().unwrap();
            print!("|");
        }
        println!();
        println!("  +--+--+--+--+--+--+--+--+");
    }
    println!("(段)");
}

pub fn show_history(state: &ReversiState) {

}